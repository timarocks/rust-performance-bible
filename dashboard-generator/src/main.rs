use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct BenchmarkData {
    benchmarks: Option<Vec<BenchmarkResult>>,
}

#[derive(Debug, Deserialize)]
struct BenchmarkResult {
    name: String,
    mean: Option<Mean>,
    throughput: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct Mean {
    point_estimate: f64,
    unit: String,
}

#[derive(Debug, Serialize)]
struct ProcessedBenchmark {
    id: String,
    name: String,
    results: Vec<ProcessedResult>,
}

#[derive(Debug, Serialize)]
struct ProcessedResult {
    name: String,
    mean: f64,
    unit: String,
    throughput: Option<f64>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Starting dashboard generation...");
    
    // Create necessary directories
    fs::create_dir_all("dashboard/data")?;
    
    // Find and process benchmark files
    let benchmark_files = find_benchmark_files()?;
    
    if benchmark_files.is_empty() {
        println!("Warning: No benchmark files found");
    }
    
    let mut benchmarks = Vec::new();
    
    for file in &benchmark_files {
        match process_benchmark_file(file) {
            Ok(benchmark) => {
                println!("Processed: {}", file.display());
                benchmarks.push(benchmark);
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", file.display(), e);
            }
        }
    }
    
    // Generate HTML dashboard
    generate_html_dashboard()?;
    
    // Generate combined benchmarks.json
    let json_path = Path::new("dashboard/data/benchmarks.json");
    let json_content = serde_json::to_string_pretty(&benchmarks)?;
    fs::write(json_path, json_content)?;
    
    // Generate README
    generate_readme()?;
    
    println!("Successfully processed {} benchmark sets", benchmarks.len());
    
    Ok(())
}

fn find_benchmark_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let result_dirs = ["benchmark-results", "benchmark-results/benchmark-results"];
    
    for dir in &result_dirs {
        if Path::new(dir).exists() {
            for entry in WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
                .filter(|e| !e.path().to_string_lossy().contains("benchmark-report"))
            {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    
    Ok(files)
}

fn process_benchmark_file(path: &Path) -> Result<ProcessedBenchmark> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    
    let data: BenchmarkData = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {:?}", path))?;
    
    let file_name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    let name = file_name
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    
    let id = file_name.to_lowercase().replace(' ', "-");
    
    let results = data.benchmarks
        .unwrap_or_default()
        .into_iter()
        .map(|bench| {
            let (mean_value, unit) = bench.mean
                .map(|m| (m.point_estimate, m.unit))
                .unwrap_or((0.0, "ns".to_string()));
            
            ProcessedResult {
                name: bench.name,
                mean: mean_value,
                unit,
                throughput: bench.throughput,
            }
        })
        .collect();
    
    // Copy raw file to dashboard/data
    let dest_path = Path::new("dashboard/data").join(format!("{}.json", file_name));
    fs::copy(path, dest_path)?;
    
    Ok(ProcessedBenchmark { id, name, results })
}

fn generate_html_dashboard() -> Result<()> {
    let timestamp = Utc::now().to_rfc3339();
    
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Performance Bible - Benchmark Dashboard</title>
    <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        .fade-in {{ animation: fadeIn 0.5s; }}
        @keyframes fadeIn {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}
        .chart-container {{
            position: relative;
            height: 300px;
            width: 100%;
        }}
    </style>
</head>
<body class="bg-gray-100">
    <div class="container mx-auto px-4 py-8">
        <header class="mb-8">
            <h1 class="text-3xl font-bold text-gray-800">Rust Performance Bible</h1>
            <p class="text-gray-600">Benchmark Dashboard - Generated at <span id="timestamp">{}</span></p>
        </header>
        
        <div id="content" class="space-y-8">
            <!-- Benchmark reports will be inserted here -->
        </div>
        
        <footer class="mt-12 text-center text-sm text-gray-500">
            <p>Generated by GitHub Actions - Rust Performance Bible</p>
        </footer>
    </div>
    
    <script>
        // Update timestamp
        document.getElementById('timestamp').textContent = new Date().toISOString();
        
        // Load benchmark data
        async function loadBenchmarks() {{
            try {{
                const response = await fetch('data/benchmarks.json');
                if (!response.ok) {{
                    throw new Error(`HTTP error! status: ${{response.status}}`);
                }}
                const data = await response.json();
                
                if (!Array.isArray(data)) {{
                    throw new Error('Invalid benchmark data format');
                }}
                
                const content = document.getElementById('content');
                
                if (data.length === 0) {{
                    content.innerHTML = `
                        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4">
                            <p class="text-sm text-yellow-700">
                                No benchmark data available. Run benchmarks to generate data.
                            </p>
                        </div>
                    `;
                    return;
                }}
                
                data.forEach(benchmark => {{
                    if (!benchmark.results || !benchmark.results.length) return;
                    
                    const section = document.createElement('div');
                    section.className = 'bg-white rounded-lg shadow p-6 fade-in';
                    
                    // Generate table rows
                    const tableRows = benchmark.results.map(result => `
                        <tr class="border-t border-gray-200">
                            <td class="py-2 px-4">${{result.name}}</td>
                            <td class="py-2 px-4 text-right">${{result.mean.toFixed(2)}} ${{result.unit}}</td>
                            <td class="py-2 px-4 text-right">
                                ${{result.throughput ? result.throughput.toFixed(2) + ' ops/s' : 'N/A'}}
                            </td>
                        </tr>
                    `).join('');
                    
                    section.innerHTML = `
                        <h2 class="text-xl font-semibold mb-4">${{benchmark.name}}</h2>
                        <div class="overflow-x-auto mb-6">
                            <table class="min-w-full bg-white">
                                <thead>
                                    <tr class="bg-gray-100">
                                        <th class="py-2 px-4 text-left">Test</th>
                                        <th class="py-2 px-4 text-right">Mean Time</th>
                                        <th class="py-2 px-4 text-right">Throughput</th>
                                    </tr>
                                </thead>
                                <tbody>${{tableRows}}</tbody>
                            </table>
                        </div>
                        <div class="chart-container">
                            <canvas id="${{benchmark.id}}-chart"></canvas>
                        </div>
                    `;
                    
                    content.appendChild(section);
                    
                    // Create chart
                    setTimeout(() => {{
                        const ctx = document.getElementById(`${{benchmark.id}}-chart`).getContext('2d');
                        new Chart(ctx, {{
                            type: 'bar',
                            data: {{
                                labels: benchmark.results.map(r => r.name),
                                datasets: [{{
                                    label: 'Mean Time (lower is better)',
                                    data: benchmark.results.map(r => r.mean),
                                    backgroundColor: 'rgba(79, 70, 229, 0.6)',
                                    borderColor: 'rgba(79, 70, 229, 1)',
                                    borderWidth: 1
                                }}]
                            }},
                            options: {{
                                responsive: true,
                                maintainAspectRatio: false,
                                scales: {{
                                    y: {{
                                        beginAtZero: true,
                                        title: {{
                                            display: true,
                                            text: 'Time (' + (benchmark.results[0]?.unit || 'ns') + ')'
                                        }}
                                    }}
                                }},
                                plugins: {{
                                    title: {{
                                        display: true,
                                        text: 'Performance Comparison'
                                    }},
                                    legend: {{
                                        display: false
                                    }}
                                }}
                            }}
                        }});
                    }}, 100);
                }});
            }} catch (error) {{
                console.error('Error loading benchmark data:', error);
                document.getElementById('content').innerHTML = `
                    <div class="bg-red-50 border-l-4 border-red-400 p-4">
                        <div class="flex">
                            <div class="flex-shrink-0">
                                <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <div class="ml-3">
                                <p class="text-sm text-red-700">
                                    Error loading benchmark data. Please check the GitHub Actions logs for more details.
                                </p>
                            </div>
                        </div>
                    </div>
                `;
            }}
        }}
        
        // Initialize the dashboard when the page loads
        document.addEventListener('DOMContentLoaded', loadBenchmarks);
    </script>
</body>
</html>"#, timestamp);

    fs::write("dashboard/index.html", html)?;
    Ok(())
}

fn generate_readme() -> Result<()> {
    let readme = r#"# Performance Dashboard

This dashboard displays benchmark results for the Rust Performance Bible.

## How It Works

1. Benchmarks are run using Criterion.rs
2. Results are processed and stored in JSON format
3. This dashboard visualizes the results using Chart.js

## Accessing the Dashboard

The dashboard is automatically deployed to GitHub Pages when benchmarks are run on the `main` branch.

## Local Development

To run the dashboard locally:

```bash
# Start a local web server
python3 -m http.server 8000

# Open in your browser
open http://localhost:8000/dashboard
```

## Data Structure

The benchmark data is stored in JSON format with the following structure:

```json
{
  "id": "benchmark-id",
  "name": "Benchmark Name",
  "results": [
    {
      "name": "test_name",
      "mean": 123.45,
      "unit": "ns",
      "throughput": 1000000
    }
  ]
}
```

## Generator

The dashboard is generated using a Rust application that processes Criterion benchmark results."#;
    
    fs::write("dashboard/README.md", readme)?;
    Ok(())
}
