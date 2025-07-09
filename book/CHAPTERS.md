# The Rust Performance Bible: Table of Contents

## Core Performance Concepts

1. [Memory Is Not Free](./001-memory-is-not-free.md)
   - Understanding allocation costs
   - Zero-copy parsing techniques
   - Memory pooling patterns

2. [Cache Lines Save Lives](./002-cache-lines-save-lives.md)
   - CPU cache architecture
   - Struct layout optimization
   - False sharing and how to avoid it

3. [Allocators: Choose Wisely](./003-allocators-choose-wisely.md)
   - Global vs. local allocators
   - Custom allocator patterns
   - Allocation strategies for different workloads

## Advanced Optimization

4. [SIMD or Die Trying](./004-simd-or-die-trying.md)
   - Vectorization in Rust
   - SIMD intrinsics
   - Auto-vectorization patterns

5. [Async Without Overhead](./005-async-without-overhead.md)
   - Zero-cost async/await
   - Executor internals
   - Task scheduling strategies

6. [Unsafe But Sound](./006-unsafe-but-sound.md)
   - When to use unsafe
   - Writing sound unsafe code
   - Performance-critical safe abstractions

## Systems Programming Mastery

7. [Zero-Copy Everything](./007-zero-copy-everything.md)
   - Memory mapping
   - Zero-copy serialization
   - I/O optimization

8. [Lock-Free or Locked Up](./008-lock-free-or-locked-up.md)
   - Atomic operations
   - Lock-free data structures
   - Memory ordering

## Performance Engineering

9. [Profile-Driven Truth](./009-profile-driven-truth.md)
   - Profiling tools and techniques
   - Benchmarking methodology
   - Performance testing

10. [Hardware Is Your Friend](./010-hardware-is-your-friend.md)
    - CPU architecture awareness
    - Memory hierarchy optimization
    - Hardware-specific optimizations

## How to Use This Book

1. **Read Linearly**: Chapters build on previous concepts
2. **Run the Code**: All examples are executable
3. **Check the Benchmarks**: See the performance impact
4. **Explore the Patterns**: Reuse optimization techniques

---

*"Premature optimization is the root of all evil, but optimization is the root of all performance." - The Crabcore Way*
