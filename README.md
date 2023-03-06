# llvm-mlir-sys
Rust bindings for the MLIR C API

## mlir-c headers doxygen

https://mlir.llvm.org/doxygen/dir_79e8035470b34d72c544e1d5f313f619.html

### Compile mlir

```bash
wget https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.0-rc3/llvm-project-16.0.0rc3.src.tar.xz

mkdir ~/mlir
tar -xf llvm-project-16.0.0rc3.src.tar.xz
cd llvm-project-16.0.0rc3.src.tar
mkdir build
cd build

cmake -G Ninja ../llvm \
   -DLLVM_ENABLE_PROJECTS=mlir \
   -DLLVM_BUILD_EXAMPLES=ON \
   -DLLVM_TARGETS_TO_BUILD="X86;AArch64" \
   -DCMAKE_BUILD_TYPE=RelWithDebInfo \
   -DLLVM_ENABLE_ASSERTIONS=ON \
   -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DLLVM_ENABLE_LLD=ON \
   -DCMAKE_INSTALL_PREFIX=~/mlir
```

### Set the env var to the compiled MLIR
```bash
export MLIR_SYS_160_PREFIX=~/mlir
```

## Learning Resources

- https://mlir.llvm.org/docs/Tutorials/
