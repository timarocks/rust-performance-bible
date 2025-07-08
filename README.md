# The Rust Performance Bible

> "In the beginning was the Word, and the Word was 'fast', and 'fast' was with Rust."

Welcome to the temple of performance. This is not just documentation - it's a belief system.

## The Crabcore Creed

![Experience the Ultimate Speed of the Unlimited Power](https://i.makeagif.com/media/5-28-2017/yBzg4n.gif)
> [Stick Stickly by Attack Attack!](https://www.youtube.com/watch?v=KDzt6yI3Dw8) is the Offcial Anthem of the Crabcore Creed

We believe:
- Memory allocation is a sin that must be minimized
- Cache misses are wounds upon our soul
- Every CPU cycle is sacred
- Benchmarks are the only truth
- Performance is not premature optimization - it's a way of life

## Start Your Journey

1. **Read the [MANIFESTO](MANIFESTO.md)** - Understand why we're here
2. **Begin with [Chapter 001: Memory Is Not Free](book/001-memory-is-not-free.md)**
3. **Run the benchmarks** - See the truth for yourself
4. **Join the congregation** - Contribute your own optimizations

## Quick Wins

Looking for immediate enlightenment? Start here:

- [Zero-allocation JSON parser](gists/zero_alloc_json.rs) - Parse without the allocation plague
- [Memory pool pattern](patterns/memory/memory_pool.rs) - Pre-allocate your way to salvation
- [SIMD summation](gists/simd_sum.rs) - 8x faster through vectorization

## Benchmark Everything

```bash
# Clone the temple
git clone https://github.com/timarocks/rust-performance-bible
cd rust-performance-bible

# Run the benchmarks of truth
cd benchmarks
cargo bench

# See the visualization of performance
cd ../tools/visualizer
cargo run -- ../benchmarks/results/