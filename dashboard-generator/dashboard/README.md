# Performance Dashboard

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

The dashboard is generated using a Rust application that processes Criterion benchmark results.