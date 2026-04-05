use burn::backend::{NdArray, Wgpu};
use burn::tensor::backend::Backend;
use burn::tensor::linalg::{lu_decomposition, lu_factor};
use burn::tensor::{Distribution, Tensor};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

const SIZES: &[usize] = &[10, 100, 500, 1000, 3000];
const BATCHED_SIZES: &[usize] = &[10, 100, 500, 1000];
const BATCH_SIZE: usize = 8;

#[divan::bench(
    types = [NdArray, Wgpu],
    args = SIZES
)]
fn old_lu_2d<B: Backend>(bencher: Bencher, size: &usize) {
    let device = Default::default();
    let tensor = Tensor::<B, 2>::random([*size, *size], Distribution::Default, &device);
    
    bencher
        .with_inputs(|| tensor.clone())
        .bench_values(|t| {
            let (res1, res2) = lu_decomposition::<B>(t);
            black_box((res1.into_data(), res2.into_data())); 
        });
}

#[divan::bench(
    types = [NdArray, Wgpu],
    args = SIZES
)]
fn new_lu_2d<B: Backend>(bencher: Bencher, size: &usize) {
    let device = Default::default();
    let tensor = Tensor::<B, 2>::random([*size, *size], Distribution::Default, &device);
    
    bencher
        .with_inputs(|| tensor.clone())
        .bench_values(|t| {
            let (res1, res2) = lu_factor::<B, 2, 1>(t);
            black_box((res1.into_data(), res2.into_data()));
        });
}

#[divan::bench(
    types = [NdArray, Wgpu],
    args = BATCHED_SIZES
)]
fn old_lu_3d<B: Backend>(bencher: Bencher, size: &usize) {
    let device = Default::default();
    let tensor = Tensor::<B, 3>::random([BATCH_SIZE, *size, *size], Distribution::Default, &device);
    
    bencher
        .with_inputs(|| tensor.clone())
        .bench_values(|t| {
            let mut res1_batches = Vec::with_capacity(BATCH_SIZE);
            let mut res2_batches = Vec::with_capacity(BATCH_SIZE);
            
            for i in 0..BATCH_SIZE {
                // Extract the slice for this batch index. 
                // This yields a 3D tensor of shape [1, size, size]
                let matrix_3d = t.clone().slice([i..i+1, 0..*size, 0..*size]);
                // Squeeze the batch dimension (dim 0) to get a 2D tensor
                let matrix_2d = matrix_3d.squeeze_dim::<2>(0);
                
                let (r1, r2) = lu_decomposition::<B>(matrix_2d);
                
                // Unsqueeze back to 3D and store in our vectors
                res1_batches.push(r1.unsqueeze::<3>());
                res2_batches.push(r2.unsqueeze::<3>());
            }
            
            // Concatenate all the 3D tensors along the batch dimension (dim 0)
            let final_res1 = Tensor::cat(res1_batches, 0);
            let final_res2 = Tensor::cat(res2_batches, 0);
            black_box((final_res1.into_data(), final_res2.into_data()));
        });
}

#[divan::bench(
    types = [NdArray, Wgpu],
    args = BATCHED_SIZES
)]
fn new_lu_3d<B: Backend>(bencher: Bencher, size: &usize) {
    let device = Default::default();
    let tensor = Tensor::<B, 3>::random([BATCH_SIZE, *size, *size], Distribution::Default, &device);
    
    bencher
        .with_inputs(|| tensor.clone())
        .bench_values(|t| {
            let (res1, res2) = lu_factor::<B, 3, 2>(t);
            black_box((res1.into_data(), res2.into_data()));
        });
}