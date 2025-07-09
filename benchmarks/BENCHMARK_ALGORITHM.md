# The CrabCore Benchmark Algorithm

A systematic, ethical approach to performance analysis that ensures reproducible, thorough benchmarking while maintaining transparency and scientific rigor.

## Overview

Benchmarking is not about running random performance tests. It's about systematic measurement, analysis, and documentation following the [AI Guidance Protocol](../docs/AI_GUIDANCE_PROTOCOL.md). This algorithm provides a structured approach that ensures:

1. **Reproducibility**: All tests can be exactly reproduced
2. **Transparency**: Full disclosure of methodology and environment
3. **Fair Comparison**: Apples-to-apples comparison between implementations
4. **Actionable Insights**: Clear, practical recommendations

## Ethical Considerations

1. **Bias Mitigation**: Actively work to identify and mitigate biases in test data and methodology
2. **Resource Awareness**: Be mindful of computational resource usage
3. **Honest Reporting**: Present both strengths and limitations of the analysis
4. **Privacy Protection**: Ensure no sensitive data is included in benchmarks

## Prerequisites

### Required Tools

```bash
# Core benchmarking
cargo install criterion \
    --features html_reports \
    --locked

# Performance analysis
cargo install flamegraph \
    cargo-bloat \
    cargo-asm

# System monitoring
cargo install heaptrack \
    cargo-llvm-cov \
    cargo-audit

# Quality control
cargo install cargo-udeps \
    cargo-geiger
```

### Environment Setup

1. **Hardware Monitoring**:
   ```bash
   # Linux
   sudo apt install lm-sensors hwloc
   
   # macOS
   brew install osx-cpu-temp
   ```

2. **System Configuration**:
   - Disable CPU frequency scaling
   - Disable turbo boost during measurements
   - Run in single-user mode for consistent results

## The CrabCore Benchmark Methodology

### 1. Benchmark Design (15 min)

**Goal**: Define clear, measurable objectives and methodology.

1. **Define Success Metrics**:
   ```markdown
   - [ ] Throughput (ops/sec)
   - [ ] Latency (p50, p90, p99, p999)
   - [ ] Memory usage (peak RSS, allocations)
   - [ ] Cache efficiency (L1/L2/L3 hit rates)
   - [ ] Instruction-level parallelism
   ```

2. **Create Benchmark Suite**
   ```rust
   // book/001-memory-is-not-free/benches/memory_patterns.rs
   use criterion::{criterion_group, criterion_main, Criterion};
   use std::hint::black_box;
   
   // Import implementations
   mod implementations {
       pub mod naive;
       pub mod optimized;
   }
   
   // Test data generation
   fn generate_test_data(count: usize) -> String {
       // Generate realistic, deterministic test data
       // Include metadata for realistic parsing scenarios
   }
   
   // Benchmark groups
   fn bench_parsing(c: &mut Criterion) {
       let mut group = c.benchmark_group("log_parsing");
       
       for size in [100, 1_000, 10_000, 100_000] {
           let data = generate_test_data(size);
           
           group.bench_with_input(
               format!("naive_{}", size), 
               &data,
               |b, input| b.iter(|| {
                   black_box(implementations::naive::parse_logs(input))
               })
           );
           
           // Similar for optimized implementation
       }
   }
   
   criterion_group!(benches, bench_parsing);
   criterion_main!(benches);
   ```

3. **Document Methodology**
   - Clearly document test environment
   - Specify warm-up and measurement procedures
   - Define pass/fail criteria

### 2. Performance Measurement (15 min)

**Goal**: Collect accurate, reproducible performance data.

1. **Run Benchmarks**
   ```bash
   # Run with full instrumentation
   RUSTFLAGS="-C target-cpu=native -C lto=fat" \
   cargo bench --features=unstable -- --warm-up-time 1 --measurement-time 3
   ```

2. **Collect System Metrics**
   - CPU utilization
   - Memory usage
   - Cache performance
   - Thermal conditions

3. **Generate Reports**
   - HTML reports with Criterion
   - Flamegraphs for hot paths
   - Memory profiling with DHAT

### 3. Analysis & Documentation (30 min)

**Goal**: Turn data into actionable insights.

1. **Performance Analysis**
   - Identify bottlenecks
   - Compare against theoretical limits
   - Check for variance and outliers

2. **Create Comprehensive Report**
   ```markdown
   ## Benchmark Results
   
   ### Key Metrics
   - **Throughput**: X ops/sec (Y% improvement)
   - **Latency**: p50=Zμs, p99=Wμs
   - **Memory**: Peak usage of A MB (B% reduction)
   
   ### Optimization Opportunities
   1. [ ] Data structure improvements
   2. [ ] Algorithmic optimizations
   3. [ ] Memory access patterns
   
   ### Recommendations
   - [ ] Adopt zero-copy parsing
   - [ ] Pre-allocate collections
   - [ ] Consider SIMD optimizations
   ```

3. **Update Documentation**
   - Add to `bench-logs/` with timestamp
   - Update relevant sections of the book
   - Create/update performance regressions tests
   ```

4. **Define input scales**
   ```rust
   const SIZES: &[usize] = &[
       100,      // Small: Cache-friendly
       1_000,    // Medium: Working set
       10_000,   // Large: Cache pressure
       100_000,  // Huge: Memory bandwidth limited
   ];
   ```

### Phase 2: Baseline Measurement (10 minutes)

**Goal**: Establish the performance baseline with the naive implementation.

1. **Run baseline benchmark**
   ```bash
   cargo bench -- naive --save-baseline naive
   ```

2. **Record key metrics**
   ```
   Baseline Metrics:
   - Time per iteration: ___ µs
   - Throughput: ___ MB/s
   - Standard deviation: ___ %
   - Cache misses: ___ (if available)
   ```

3. **Save flame graph**
   ```bash
   cargo flamegraph --bench your_feature -- --bench naive
   mv flamegraph.svg results/baseline_flame.svg
   ```

### Phase 3: Optimization Testing (15 minutes per variant)

**Goal**: Measure each optimization and verify correctness.

For each optimization variant:

1. **Run comparative benchmark**
   ```bash
   cargo bench -- optimized --baseline naive
   ```

2. **Verify correctness**
   ```rust
   #[test]
   fn verify_optimization_correctness() {
       let data = generate_test_data();
       assert_eq!(
           naive::process(&data),
           optimized::process(&data),
           "Optimization produces different results!"
       );
   }
   ```

3. **Document improvements**
   ```
   Optimization: [Name]
   Improvement: ___x faster
   Trade-offs: [memory usage, complexity, etc.]
   Surprising behavior: [if any]
   ```

### Phase 4: Deep Profiling (20 minutes)

**Goal**: Understand WHERE the improvements come from.

1. **Memory profiling**
   ```bash
   # Linux with valgrind
   valgrind --tool=massif --massif-out-file=massif.out \
            target/release/bench_binary
   ms_print massif.out > memory_profile.txt
   
   # Look for:
   # - Peak memory usage
   # - Allocation frequency
   # - Memory growth patterns
   ```

2. **CPU profiling**
   ```bash
   # Linux with perf
   cargo bench --no-run
   perf record --call-graph=dwarf \
        target/release/deps/your_bench-*
   perf report
   
   # macOS with Instruments
   cargo instruments -t "Time Profiler" \
         --bench your_feature
   ```

3. **Allocation tracking**
   ```bash
   # Count allocations
   cargo build --release --example your_example
   
   # Linux
   ltrace -e malloc,free ./target/release/examples/your_example 2>&1 | wc -l
   
   # Or use custom allocator
   ```

4. **Cache analysis**
   ```bash
   # Linux with perf
   perf stat -e cache-misses,cache-references \
        target/release/bench_binary
   
   # Look for cache miss rate
   ```

### Phase 5: Analysis & Visualization (10 minutes)

**Goal**: Transform raw data into actionable insights.

1. **Create comparison table**
   
   | Input Size | Naive (ms) | Optimized (ms) | Improvement | Memory (MB) |
   |------------|------------|----------------|-------------|-------------|
   | 100        | ___        | ___            | ___x        | ___         |
   | 1,000      | ___        | ___            | ___x        | ___         |
   | 10,000     | ___        | ___            | ___x        | ___         |
   | 100,000    | ___        | ___            | ___x        | ___         |

2. **Generate visualization**
   ```python
   # Simple Python script for graphs
   import matplotlib.pyplot as plt
   
   sizes = [100, 1000, 10000, 100000]
   naive_times = [...]
   optimized_times = [...]
   
   plt.loglog(sizes, naive_times, 'r-o', label='Naive')
   plt.loglog(sizes, optimized_times, 'g-o', label='Optimized')
   plt.xlabel('Input Size')
   plt.ylabel('Time (ms)')
   plt.legend()
   plt.savefig('results/performance_comparison.png')
   ```

3. **Identify patterns**
   - Is improvement consistent across sizes?
   - Where does the optimization break down?
   - What's the limiting factor?

### Phase 6: Documentation (5 minutes)

**Goal**: Preserve knowledge for future developers.

1. **Update results file**
   ```markdown
   ## Benchmark: [Feature Name]
   Date: YYYY-MM-DD
   Machine: [CPU, RAM, OS]
   Rust Version: X.XX.X
   
   ### Summary
   - Best improvement: ___x at ___ size
   - Memory reduction: ___%
   - Key insight: [what made the difference]
   
   ### Detailed Results
   [tables and graphs]
   
   ### Reproduction Steps
   ```

2. **Commit with semantic message**
   ```bash
   git add benchmarks/results/
   git commit -m "bench: optimize [feature] (Nx improvement)
   
   - Baseline: X ms/iteration
   - Optimized: Y ms/iteration  
   - Technique: [brief description]"
   ```

## Quick Reference Card

```bash
# Save this as benchmark.sh in your project root

#!/bin/bash
FEATURE=$1
DATE=$(date +%Y-%m-%d)

echo "=== Crabcore Benchmark Protocol ==="
echo "Feature: $FEATURE"
echo "Date: $DATE"
echo ""
echo "Phase 1: Setup ✓"
echo "[ ] Test data covers edge cases"
echo "[ ] Benchmark file created"
echo ""
echo "Phase 2: Baseline"
echo "cargo bench -- $FEATURE --save-baseline naive"
echo ""
echo "Phase 3: Optimization" 
echo "cargo bench -- $FEATURE --baseline naive"
echo ""
echo "Phase 4: Profiling"
echo "cargo flamegraph --bench $FEATURE"
echo "valgrind --tool=massif target/release/deps/$FEATURE-*"
echo ""
echo "Phase 5: Analysis"
echo "[ ] Comparison table filled"
echo "[ ] Graphs generated"
echo ""
echo "Phase 6: Documentation"
echo "[ ] Results saved to benchmarks/results/$DATE-$FEATURE.md"
echo "[ ] Git commit with improvement stats"
```

## Common Pitfalls to Avoid

1. **Benchmarking in debug mode** - Always use `--release`
2. **Too few iterations** - Let Criterion determine sample size
3. **Forgetting to black_box** - Prevents optimizer from eliminating code
4. **Inconsistent environment** - Close other applications, use consistent power settings
5. **Measuring the wrong thing** - Ensure you're measuring the actual work, not setup

## Interpreting Results

### What's a Good Improvement?

- **2x**: Solid optimization, worth the complexity
- **5x**: Excellent result, definitely keep
- **10x+**: Exceptional, likely fixing a fundamental issue
- **<1.5x**: Consider if added complexity is worth it

### When to Stop Optimizing

Stop when you hit one of these:
1. Target performance is achieved
2. Next optimization would be <20% improvement
3. Code complexity exceeds maintenance capacity
4. You're optimizing the wrong bottleneck

## Example Benchmark Report

```markdown
# Benchmark Results: Zero-Allocation JSON Parser
Date: 2025-07-07
Machine: M1 MacBook Pro (8-core, 16GB)
Rust: 1.75.0

## Executive Summary
Achieved 12x performance improvement by eliminating allocations in JSON parsing through lifetime management and pre-calculation of string positions.

## Results

| JSON Size | Naive (µs) | Zero-Alloc (µs) | Improvement | Allocations |
|-----------|------------|-----------------|-------------|-------------|
| 1 KB      | 45.2       | 3.8             | 11.9x       | 23 → 0      |
| 10 KB     | 523.7      | 42.1            | 12.4x       | 234 → 0     |
| 100 KB    | 5,847.3    | 468.9           | 12.5x       | 2,341 → 0   |

## Key Insights
1. Allocation overhead dominates parsing time
2. Performance improvement is consistent across sizes
3. Zero-allocation approach trades API ergonomics for speed

## Reproduction
```bash
cd benchmarks
cargo bench -- json_parser
```
```

---

Remember: **Benchmarks are only as good as your measurement discipline.** Follow this algorithm consistently, and you'll build a reliable performance knowledge base.