# LU Decomposition Benchmark

This folder benchmarks the existing `burn` LU decomposition against a new, optimized implementation.

## Methodology
- **Tool:** `divan`
- **Backend Tested:** `NdArray` (CPU) / `Wgpu` (GPU)
- **Hardware:** [CPU: Intel(R) Core(TM) i7-8565U (8), GPU: Intel UHD Graphics 620 (24)]

## How to run
From the root of the workspace, run:
```bash
cargo bench --bench lu_bench
```