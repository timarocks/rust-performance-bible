# GitHub Actions Benchmark Automation

This directory contains automated workflows for the Rust Performance Bible benchmarking suite.

## Workflows

### 1. `benchmark.yml` - Comprehensive Benchmark Suite

**Triggers:**
- Push to `main` branch
- Pull requests
- Manual dispatch

**Features:**
- Runs on multiple OS (Ubuntu, macOS)
- Tests on stable and nightly Rust
- Generates flamegraphs and performance profiles
- Stores historical benchmark data
- Creates performance dashboard

**Outputs:**
- Benchmark results JSON
- Performance comparison reports
- Flamegraphs
- Binary size analysis
- Historical tracking on `gh-pages` branch

### 2. `pr-benchmark.yml` - PR Performance Check

**Triggers:**
- Pull requests that modify source code

**Features:**
- Compares PR performance against base branch
- Comments results directly on PR
- Fails CI if performance regression detected
- Lightweight and fast

**Example PR Comment:**
```
## Benchmark Results: Performance Check Passed

Click to see detailed benchmark comparison
```

## Setup Instructions

1. **Enable GitHub Pages** (for benchmark dashboard):
   - Go to Settings → Pages
   - Source: Deploy from branch
   - Branch: `gh-pages` / `docs` folder

2. **Add Repository Secrets** (if needed):
   - No additional secrets required for basic operation
   - Uses `GITHUB_TOKEN` automatically provided

3. **First Run**:
   - The first run will establish baseline metrics
   - Historical data accumulates over time

## Usage

### Running Benchmarks Manually

```bash
# Trigger benchmark workflow manually
gh workflow run benchmark.yml

# With custom baseline
gh workflow run benchmark.yml -f baseline=v1.0.0
```

### Interpreting Results

**Performance Changes:**
- Green: Performance improvement or neutral (within 5%)
- Yellow: Minor regression (5-15%)
- Red: Major regression (>15%)

**Benchmark Dashboard:**
- Available at: `https://[username].github.io/rust-performance-bible/benchmarks/`
- Updates automatically on each push to main

### Local Testing

Test workflows locally using [act](https://github.com/nektos/act):

```bash
# Test PR workflow
act pull_request -W .github/workflows/pr-benchmark.yml

# Test main workflow
act push -W .github/workflows/benchmark.yml
```

## Customization

### Adding New Benchmarks

1. Add benchmark to `benchmarks/benches/`
2. Update workflow to include new benchmark:

```yaml
- name: Run benchmarks
  run: |
    cargo bench --bench memory_patterns
    cargo bench --bench your_new_benchmark
```

### Changing Performance Thresholds

Edit regression detection in `pr-benchmark.yml`:

```bash
# Current: Any performance increase is a regression
if grep -E "\+[0-9]+\.[0-9]+%" perf-summary.txt > /dev/null; then

# Change to: Only >10% increase is a regression  
if grep -E "\+[1-9][0-9]+\.[0-9]+%" perf-summary.txt > /dev/null; then
```

### Platform-Specific Benchmarks

Add conditions for OS-specific benchmarks:

```yaml
- name: macOS-specific benchmark
  if: matrix.os == 'macos-latest'
  run: cargo bench --bench metal_performance
```

## Troubleshooting

**Benchmarks timing out:**
- Increase timeout: `timeout-minutes: 30`
- Reduce benchmark iterations in CI

**Cache issues:**
- Clear cache in Actions tab
- Bump cache key version

**Permission errors:**
- Ensure workflow has write permissions for PR comments
- Check repository settings for Actions permissions

## Best Practices

1. **Keep benchmarks fast in CI** - Use fewer iterations than local
2. **Use baseline saves** - Avoid re-running base branch
3. **Monitor trends** - Single results can be noisy
4. **Document anomalies** - Hardware differences affect results

---

For questions or improvements, open an issue in the main repository.