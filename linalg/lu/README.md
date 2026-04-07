# LU Decomposition Benchmark

This folder benchmarks the existing `burn` LU decomposition against a new, more optimized implementation.

## Methodology
- **Tool:** `divan`
- **Backend Tested:** `NdArray` (CPU) / `Wgpu` (GPU)
- **Hardware:** [CPU: Intel Core i7-8565U (8), GPU: Intel UHD Graphics 620 (24)]

## How to run
From the root of the workspace, run:
```bash
cargo bench --bench lu_bench
```
## Results

### 2D Input Tensors - NdArray
| Size | Version | Fastest | Slowest | Median | Mean |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **10** | Old | **202.7 µs** | **420.8 µs** | **213.5 µs** | **216.7 µs** |
| | New | 474.4 µs | 2.298 ms | 823.5 µs | 776.4 µs |
| **100** | Old | 17.66 ms | **22.78 ms** | 18.32 ms | 18.49 ms |
| | New | **11.58 ms** | 23.67 ms | **17.9 ms** | **17.13 ms** |
| **500** | Old | 511.2 ms | 653.9 ms | 533.5 ms | 537.6 ms |
| | New | **210.9 ms** | **264.8 ms** | **236.8 ms** | **237.4 ms** |
| **1000** | Old | 2.316 s | 2.612 s | 2.475 s | 2.462 s |
| | New | **751.2 ms** | **2.135 s** | **810.4 ms** | **834.4 ms** |
| **3000** | Old | 37.68 s | 42.49 s | 39.16 s | 39.22 s |
| | New | **6.641 s** | **9.873 s** | **7.008 s** | **7.204 s** |

<br>

### 2D Input Tensors - Wgpu Fusion*
| Size | Version | Fastest | Slowest | Median | Mean |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **10** | New | 8.246 ms | 90.36 ms | 9.272 ms | 23.87 ms |
| **100** | New | 78.39 ms | 170.5 ms | 85.59 ms | 88.06 ms |
| **500** | New | 453.6 ms | 765 ms | 482.4 ms | 487.5 ms |
| **1000** | New | 945.1 ms | 2.317 s | 991.2 ms | 1.018 s |
| **3000** | New | 3.685 s | 4.498 s | 3.949 s | 3.946 s |

<br>

### 3D Input Tensors - NdArray
| Size | Version | Fastest | Slowest | Median | Mean |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **10** | Old | 1.475 ms | 3.918 ms | **1.706 ms** | **1.784 ms** |
| | New | **1.323 ms** | **3.747 ms** | 1.945 ms | 2.022 ms |
| **100** | Old | 123.1 ms | 137.6 ms | 128.6 ms | 129 ms |
| | New | **78.12 ms** | **104.6 ms** | **86.39 ms** | **87.38 ms** |
| **500** | Old | 3.684 s | 4.17 s | 3.962 s | 3.94 s |
| | New | **1.452 s** | **2.796 s** | **1.526 s** | **1.568 s** |
| **1000** | Old | 18.11 s | 19.84 s | 19.22 s | 19.12 s |
| | New | **6.364 s** | **9.097 s** | **6.68 s** | **6.791 s** |

<br>

### 3D Input Tensors - Wgpu Fusion*
| Size | Version | Fastest | Slowest | Median | Mean |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **10** | New | 8.951 ms | 135.9 ms | 9.577 ms | 11.54 ms |
| **100** | New | 90.26 ms | 152.1 ms | 95.88 ms | 98.86 ms |
| **500** | New | 615.5 ms | 799.7 ms | 640.5 ms | 643.4 ms |
| **1000** | New | 1.675 s | 2.42 s | 1.722 s | 1.779 s |

---
**Notes:**
- *\*Wgpu Fusion stands for:* `Fusion<burn_cubecl::backend::CubeBackend<cubecl_wgpu::runtime::WgpuRuntime, f32, i32, u32>>`
- *All benchmarks were run with exactly **100 samples** and **100 iterations**.*
- *The old function was benchmarked only on NdArray backend since it is very slow on Wgpu and it sometimes failed due to the scalar handling.*