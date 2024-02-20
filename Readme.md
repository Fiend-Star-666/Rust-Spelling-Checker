Developed a sophisticated spelling checker in Rust, integrating advanced algorithms for unparalleled accuracy and efficiency.

Key Features:
- Levenshtein & Wagner-Fischer Algorithms: Utilizes these algorithms for precise error detection and distance calculation.
- CUDA Integration: Leverages CUDA for accelerated computation, significantly enhancing performance.
- Rayon for Parallel Computing: Implements Rayon, enabling efficient parallel data processing.
- NLP for Context Awareness: Employs natural language processing to understand context, improving the accuracy of correction suggestions.
- Soundex Algorithm: Uses Soundex for phonetic comparisons, catching errors missed by traditional spell checkers.

This project is a showcase of my ability to blend multiple technologies — from CUDA and Rayon for performance to NLP and Soundex for accuracy — in creating a state-of-the-art spelling checker.


** Its a work in progress

current performance:
currently working on a cpu:
takes in a 1.5M dataset of words finds the issues from a 650k words dictionary in about 35ms and gives 3 predictions for each error word in about 22 seconds for 4000 unique corrections which are duplicated many times over.

trying to figure out a way to make it work on cuda


Run command:
run --package rust-spell-checker --bin rust-spell-checker --release

env variables: RUST_LOG=info


$env:CUDA_ROOT = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3"
clang++ -O3 -std=c++14 --cuda-path=${env:CUDA_ROOT} --cuda-gpu-arch=sm_86 -L/usr/local/cuda/lib64 -lcudart_static -ldl -lrt -pthread suggest_corrections_kernel.cu -o suggest_corrections_kernel.ptx


$env:Path += ";C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64"


