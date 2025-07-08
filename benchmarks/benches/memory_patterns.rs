use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;

// Import the patterns we're testing
include!("../../book/001-memory-is-not-free/naive.rs");
include!("../../book/001-memory-is-not-free/optimized.rs");
include!("../../patterns/memory/memory_pool.rs");

// Test data generator
fn generate_log_data(lines: usize) -> String {
    let mut data = String::with_capacity(lines * 100);
    for i in 0..lines {
        use std::fmt::Write;
        let metadata_count = i % 5;
        write!(
            &mut data,
            "2024-01-01T12:00:{:02}|{}|Request processed",
            i % 60,
            if i % 4 == 0 { "ERROR" } else { "INFO" }
        ).unwrap();
        
        // Add variable metadata
        for j in 0..metadata_count {
            write!(&mut data, "|key{}=value{}", j, i).unwrap();
        }
        data.push('\n');
    }
    data
}

fn benchmark_log_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("log_parsing");
    
    // Test different data sizes
    for size in [100, 1_000, 10_000, 100_000] {
        let test_data = generate_log_data(size);
        let data_size = test_data.len() as u64;
        
        group.throughput(Throughput::Bytes(data_size));
        
        // Benchmark naive approach
        group.bench_with_input(
            BenchmarkId::new("naive", size),
            &test_data,
            |b, data| {
                b.iter(|| {
                    let logs = parse_logs(black_box(data));
                    black_box(logs.len())
                })
            },
        );
        
        // Benchmark zero-copy approach
        group.bench_with_input(
            BenchmarkId::new("zero_copy", size),
            &test_data,
            |b, data| {
                b.iter(|| {
                    let logs: Vec<_> = parse_logs_fast(black_box(data)).collect();
                    black_box(logs.len())
                })
            },
        );
        
        // Benchmark with memory pool
        let pool = StringPool::new(1_000_000);
        group.bench_with_input(
            BenchmarkId::new("with_pool", size),
            &test_data,
            |b, data| {
                b.iter(|| {
                    let logs = process_logs_with_pool(black_box(data), &pool);
                    black_box(logs.len())
                })
            },
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
            let logs = parse_logs(black_box(&test_data));
            // Force evaluation of all allocations
            for log in &logs {
                black_box(&log.timestamp);
                black_box(&log.level);
                black_box(&log.message);
                black_box(&log.metadata);
            }
        })
    });
    
    group.finish();
}

// Memory pool efficiency
fn benchmark_memory_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");
    
    // Compare pool allocation vs heap allocation
    group.bench_function("heap_allocation", |b| {
        b.iter(|| {
            let mut strings = Vec::with_capacity(1000);
            for i in 0..1000 {
                strings.push(format!("String number {}", i));
            }
            black_box(strings)
        })
    });
    
    group.bench_function("pool_allocation", |b| {
        let pool = MemoryPool::<String>::new(1000);
        b.iter(|| {
            let mut guards = Vec::with_capacity(1000);
            for i in 0..1000 {
                let mut guard = pool.allocate().unwrap();
                *guard = format!("String number {}", i);
                guards.push(guard);
            }
            black_box(guards)
        })
    });
    
    group.finish();
}

// Specific patterns from the article
fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_patterns");
    
    let data = vec!["hello", "world", "rust", "performance"];
    
    // Bad pattern: String concatenation in loop
    group.bench_function("concat_with_allocation", |b| {
        b.iter(|| {
            let mut result = String::new();
            for s in &data {
                result = result + s; // Allocates every time!
            }
            black_box(result)
        })
    });
    
    // Good pattern: Pre-allocated with push_str
    group.bench_function("concat_preallocated", |b| {
        b.iter(|| {
            let mut result = String::with_capacity(
                data.iter().map(|s| s.len()).sum()
            );
            for s in &data {
                result.push_str(s);
            }
            black_box(result)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_log_parsing,
    benchmark_allocations,
    benchmark_memory_pool,
    benchmark_string_operations
);
criterion_main!(benches);