use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// Simple log parsing implementations for benchmarking
mod naive {
    use super::LogEntry;
    
    pub fn parse_logs(input: &str) -> Vec<LogEntry> {
        input.lines()
            .map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    LogEntry {
                        timestamp: parts[0].to_string(),
                        level: parts[1].to_string(),
                        message: parts[2..].join("|")
                    }
                } else {
                    LogEntry {
                        timestamp: "".to_string(),
                        level: "ERROR".to_string(),
                        message: line.to_string()
                    }
                }
            })
            .collect()
    }
}

mod optimized {
    use super::LogEntry;
    
    pub fn parse_logs(input: &str) -> Vec<LogEntry> {
        let mut entries = Vec::with_capacity(input.lines().count());
        
        for line in input.lines() {
            let mut parts = line.splitn(3, '|');
            
            let entry = LogEntry {
                timestamp: parts.next().unwrap_or("").to_string(),
                level: parts.next().unwrap_or("UNKNOWN").to_string(),
                message: parts.next().unwrap_or("").to_string(),
            };
            
            entries.push(entry);
        }
        
        entries
    }
}

// Import only what we use
use naive::parse_logs as parse_logs_naive;
use optimized::parse_logs as parse_logs_optimized;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

// Test data generator
fn generate_log_data(lines: usize) -> String {
    let mut data = String::with_capacity(lines * 100);
    for i in 0..lines {
        use std::fmt::Write;
        let level = if i % 4 == 0 { "ERROR" } else { "INFO" };
        let message = "Request processed";
        
        write!(
            &mut data,
            "2025-01-01T12:00:{:02}|{}|{}",
            i % 60,
            level,
            message
        ).unwrap();
        
        // Add metadata
        let metadata_count = i % 5;
        for j in 0..metadata_count {
            write!(&mut data, "|key{}=value{}", j, i).unwrap();
        }
        
        data.push('\n');
    }
    
    data
}

fn benchmark_log_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("log_parsing");
    
    // Test with different input sizes
    for size in [10, 100, 1000].iter() {
        let input = generate_log_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("naive", size),
            &input,
            |b, i| b.iter(|| parse_logs_naive(black_box(i)))
        );
        
        group.bench_with_input(
            BenchmarkId::new("optimized", size),
            &input,
            |b, i| b.iter(|| parse_logs_optimized(black_box(i)))
        );
    }
    
    group.finish();
}

// Allocation count benchmark
fn benchmark_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_behavior");
    group.sample_size(10); // Fewer samples for allocation tracking
    
    let test_data = generate_log_data(1000);
    
    // This benchmark is more about profiling than timing
    group.bench_function("naive_allocations", |b| {
        b.iter(|| {
            let logs = parse_logs_naive(black_box(&test_data));
            // Force evaluation of all allocations
            for log in &logs {
                black_box(&log.timestamp);
                black_box(&log.level);
                black_box(&log.message);
            }
            black_box(logs);
        })
    });
    
    group.bench_function("optimized_allocations", |b| {
        b.iter(|| {
            let logs = parse_logs_optimized(black_box(&test_data));
            // Force evaluation of all allocations
            for log in &logs {
                black_box(&log.timestamp);
                black_box(&log.level);
                black_box(&log.message);
            }
            black_box(logs);
        })
    });
    
    group.finish();
}

// Memory efficiency benchmarks
fn benchmark_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    
    // Compare different ways to build strings
    group.bench_function("string_push_str", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(4000);
            for i in 0..1000 {
                s.push_str(&i.to_string());
            }
            black_box(s);
        })
    });
    
    group.bench_function("string_collect", |b| {
        b.iter(|| {
            let s: String = (0..1000).map(|i| i.to_string()).collect();
            black_box(s);
        })
    });
    
    group.finish();
}

// String processing benchmarks
fn benchmark_string_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_processing");
    
    let test_data = generate_log_data(1000);
    
    group.bench_function("parse_and_count_errors", |b| {
        b.iter(|| {
            let logs = parse_logs_optimized(black_box(&test_data));
            let error_count = logs.iter()
                .filter(|log| log.level == "ERROR")
                .count();
            black_box(error_count);
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_log_parsing,
    benchmark_allocations,
    benchmark_memory_efficiency,
    benchmark_string_processing
);

criterion_main!(benches);