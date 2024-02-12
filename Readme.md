Run command:
run --package rust-spell-checker --bin rust-spell-checker --release

env variables: RUST_LOG=info


$env:CUDA_ROOT = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3"
clang++ -O3 -std=c++14 --cuda-path=${env:CUDA_ROOT} --cuda-gpu-arch=sm_86 -L/usr/local/cuda/lib64 -lcudart_static -ldl -lrt -pthread suggest_corrections_kernel.cu -o suggest_corrections_kernel.ptx