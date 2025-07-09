# The Rust Performance Bible

> *In the beginning was the Word, and the Word was 'fast', and 'fast' was with Rust.*

A comprehensive guide to writing high-performance Rust code, based on real optimization experience and the Crabcore Framework principles.

## What Is This?

The Rust Performance Bible is a collection of hard-earned optimization knowledge, patterns, and techniques for Rust developers who care about performance. Each chapter demonstrates real optimizations with working code and benchmarks you can run yourself.

## The Crabcore Creed

![Experience the Ultimate Speed of the Unlimited Power](https://i.makeagif.com/media/5-28-2017/yBzg4n.gif)
> *[Stick Stickly by Attack Attack!](https://www.youtube.com/watch?v=KDzt6yI3Dw8) is the Official Anthem of the Crabcore Creed because it's fast, it's loud, and I’ve listened to it something like 666 times while working on the Crabcore Framework and this Bible.*

### In Crabcore We Believe:

- Memory allocation is a sin that must be minimized
- Cache misses are wounds upon our soul  
- Every CPU cycle is sacred
- Benchmarks are the only truth
- Performance is not premature optimization - it's a way of life

## The Ten Commandments of Performance

- Thou _**shalt not allocate**_ in hot paths
- Thou _**shalt respect**_ cache locality
- Thou _**shalt measure**_ before optimizing
- Thou _**shalt vectorize**_ when parallel
- Thou _**shalt not block**_ when async will do
- Thou _**shalt profile**_ in release mode
- Thou _**shalt know**_ thy hardware
- Thou _**shalt benchmark**_ religiously
- Thou _**shalt use**_ unsafe when proven safe
- Thou _**shalt share**_ thy optimizations

## Core Principles

- **Measure Everything**: No optimization without benchmarks
- **Understand Your Hardware**: Cache lines, memory bandwidth, CPU architecture matter
- **Zero-Cost Abstractions**: Rust lets you go fast safely
- **Practical Focus**: Real code, real improvements

## Structure

```
rust-performance-bible/
├── book/          # Core chapters (see [CHAPTERS.md](./book/CHAPTERS.md))
├── patterns/      # Reusable optimization patterns
├── gists/         # Quick code snippets  
├── benchmarks/    # Benchmark suite
├── examples/      # Complete examples
└── crabcore/      # The underlying framework
```

## The Chapters

1. **[001 · Memory Is Not Free](book/001-memory-is-not-free.md)** - Understanding allocation costs
2. **002 · Cache Lines Save Lives** - Data layout optimization (coming soon)
3. **003 · Choose Your Allocator** - Custom memory management (coming soon)
4. **004 · SIMD or Die Trying** - Vectorization techniques (coming soon)
5. **005 · Async Without Overhead** - High-performance concurrency (coming soon)
6. More chapters in development...

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-performance-bible
cd rust-performance-bible

# Run benchmarks to see the techniques in action
cd benchmarks
cargo bench

# Explore examples
cd ../examples/log_parser
cargo run --release
```

## Benchmark First

Every optimization in this bible comes with benchmarks. Run them on your machine to see real results:

```bash
cd benchmarks
cargo bench -- --save-baseline before

# Apply optimization from the book
# ... 

cargo bench -- --baseline before
```

Your hardware, your results. No magic numbers.

### Tima's Benchmark Algorithm

See [BENCHMARK_ALGORITHM.md](benchmarks/BENCHMARK_ALGORITHM.md) for details.

## Philosophy

Performance optimization in Rust isn't about premature optimization - it's about understanding the cost of your decisions and having the tools to make informed choices when performance matters.

This resource focuses on:
- **Practical techniques** that apply to real systems
- **Measurable improvements** with reproducible benchmarks  
- **Understanding why** optimizations work, not just how

## Current Status

This is an active project being developed into a comprehensive resource. New chapters and examples are being added regularly.

## Tools You'll Need

```bash
# For benchmarking
cargo install criterion

# For profiling
cargo install flamegraph

# For analysis
# macOS: Instruments
# Linux: perf, valgrind
```

## Getting Started

1. Browse the [Table of Contents](./book/CHAPTERS.md) to find topics of interest
2. Read chapters in order for a structured learning path
3. Run the examples and benchmarks to see optimizations in action

## Contributing

Contributions are welcome! Please read our [Contribution Guidelines](CONTRIBUTING.md) first.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*"The only way to go fast is to go well." - Robert C. Martin*

---

## GitHub Repository Description

```
The Rust Performance Bible: A comprehensive guide to writing high-performance Rust code. Covers memory optimization, zero-cost abstractions, async/await, SIMD, lock-free programming, and systems programming techniques. Includes working code examples and benchmarks.