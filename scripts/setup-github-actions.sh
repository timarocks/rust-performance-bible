#!/bin/bash
# Setup GitHub Actions for Rust Performance Bible benchmarks

set -e

echo "Setting up GitHub Actions for Rust Performance Bible"
echo "======================================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "benchmarks" ]; then
    echo "Error: Must run from repository root"
    exit 1
fi

# Create workflow directory
echo "Creating workflow directory..."
mkdir -p .github/workflows

# Function to download workflow files
download_workflow() {
    local filename=$1
    local description=$2
    
    echo "Creating $filename - $description"
    
    # In real usage, you'd copy the files from templates
    # For now, we'll create a marker
    touch ".github/workflows/$filename"
    echo "# $description" > ".github/workflows/$filename"
    echo "# Copy content from rust-performance-bible templates" >> ".github/workflows/$filename"
}

# Download workflows
download_workflow "benchmark.yml" "Comprehensive benchmark suite"
download_workflow "pr-benchmark.yml" "PR performance checks"

# Create directories for results
echo "Creating benchmark result directories..."
mkdir -p benchmarks/results
mkdir -p benchmarks/profiles

# Create initial benchmark baseline script
cat > benchmarks/create-baseline.sh << 'EOF'
#!/bin/bash
# Create initial baseline for benchmarks

echo "Creating baseline benchmarks..."
cargo bench -- --save-baseline main

echo "Baseline created! Use 'cargo bench -- --baseline main' to compare"
EOF

chmod +x benchmarks/create-baseline.sh

# Create PR template
echo "Creating PR template..."
mkdir -p .github
cat > .github/pull_request_template.md << 'EOF'
## Description
Brief description of changes

## Performance Impact
- [ ] I have run benchmarks locally
- [ ] No performance regression detected
- [ ] Performance improvement: ___x

## Benchmark Results
```
# Paste local benchmark results here
cargo bench -- --baseline main
```

## Checklist
- [ ] Tests pass
- [ ] Benchmarks pass
- [ ] Documentation updated
EOF

# Create benchmark visualization script
echo "Creating visualization script..."
cat > benchmarks/visualize.py << 'EOF'
#!/usr/bin/env python3
"""Visualize benchmark results"""

import json
import sys
import matplotlib.pyplot as plt

def visualize_results(json_file):
    with open(json_file, 'r') as f:
        data = json.load(f)
    
    # Add visualization logic here
    print(f"Loaded benchmark data from {json_file}")
    print("Visualization coming soon...")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python visualize.py <benchmark-results.json>")
        sys.exit(1)
    
    visualize_results(sys.argv[1])
EOF

chmod +x benchmarks/visualize.py

# Create GitHub Pages setup
echo "Creating GitHub Pages structure..."
mkdir -p docs/benchmarks
cat > docs/benchmarks/.gitkeep << 'EOF'
# Benchmark results will be published here
EOF

# Summary
echo ""
echo "Setup complete!"
echo ""
echo "Next steps:"
echo "1. Copy workflow content from templates to .github/workflows/"
echo "2. Commit and push changes"
echo "3. Enable GitHub Pages in repository settings"
echo "4. Run 'cd benchmarks && ./create-baseline.sh' to create initial baseline"
echo ""
echo "Workflows will activate on next push/PR"
echo ""
echo "Optional: Install 'act' to test workflows locally:"
echo "  brew install act  # macOS"
echo "  # or visit: https://github.com/nektos/act"