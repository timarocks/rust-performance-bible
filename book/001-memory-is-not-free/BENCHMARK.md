# Memory Performance Benchmarks

This directory contains benchmarks for the "Memory Is Not Free" chapter of the Rust Performance Bible.

## Structure

```
001-memory-is-not-free/
├── code/                   # Implementation code
│   ├── naive.rs           # Naive implementation with allocations
│   └── optimized.rs       # Optimized zero-copy implementation
├── bench-logs/            # Historical benchmark results
│   └── BENCH_LOG_*.md     # Dated benchmark result files
└── BENCHMARK.md           # This file
```

## Running Benchmarks Locally

1. Navigate to this directory:
   ```bash
   cd book/001-memory-is-not-free
   ```

2. Run the benchmarks:
   ```bash
   cargo bench
   ```

3. For detailed output:
   ```bash
   cargo bench -- --nocapture
   ```

## Benchmark Types

1. **Log Parsing**
   - Measures throughput and latency of parsing log files
   - Compares naive vs optimized implementations
   - Tests with varying input sizes (100 - 100,000 entries)

2. **Allocation Behavior**
   - Tracks memory allocation patterns
   - Measures impact of allocation strategies

## Interpreting Results

- Look for significant differences in throughput (MiB/s)
- Check for consistent performance across input sizes
- Monitor allocation counts and memory usage

## Adding New Benchmarks

1. Add new benchmark functions to the appropriate module
2. Update the benchmark groups in `lib.rs`
3. Document the purpose and expected results in this file
