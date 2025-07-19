## Installation

```bash
apt update && apt -y install build-essential \
    curl \
    clang \
    libssl-dev \
    libtinfo-dev \
    pkg-config \
    xz-utils \
    zlib1g-dev
    
mkdir -p /data/llvm7
cd /data/llvm7
curl -sSf -L -O http://mirrors.kernel.org/ubuntu/pool/universe/l/llvm-toolchain-7/llvm-7_7.0.1-12_amd64.deb && \
curl -sSf -L -O http://mirrors.kernel.org/ubuntu/pool/universe/l/llvm-toolchain-7/llvm-7-dev_7.0.1-12_amd64.deb && \
curl -sSf -L -O http://mirrors.kernel.org/ubuntu/pool/universe/l/llvm-toolchain-7/libllvm7_7.0.1-12_amd64.deb && \
curl -sSf -L -O http://mirrors.kernel.org/ubuntu/pool/universe/l/llvm-toolchain-7/llvm-7-runtime_7.0.1-12_amd64.deb && \
apt-get update && apt-get install -y ./*.deb && \
rm -rf ./*.deb

curl -sSf -L https://sh.rustup.rs | bash -s -- -y
export LD_LIBRARY_PATH=/usr/local/cuda/nvvm/lib64
export LLVM_CONFIG=/usr/bin/llvm-config-7
```