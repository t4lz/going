# syntax=docker/dockerfile:1
FROM arm64v8/ubuntu:latest

RUN apt update
RUN apt install -y python3 pip cmake curl openssl libssl-dev pkg-config clang libclang-dev llvm llvm-dev wget lldb vim less

# For lldb https://bugs.launchpad.net/ubuntu/+source/llvm-defaults/+bug/1972855
RUN mkdir -p /usr/lib/local/lib/python3.10/
RUN ln -s /usr/lib/llvm-14/lib/python3.10/dist-packages/ /usr/lib/local/lib/python3.10/dist-packages

RUN wget -L "https://golang.org/dl/go1.19.2.linux-arm64.tar.gz" && tar -C /usr/local -xzf go1.19.2.linux-arm64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install bindgen-cli
RUN rustup toolchain install nightly

RUN wget -L "https://github.com/frida/frida/releases/download/15.2.2/frida-gum-devkit-15.2.2-linux-arm64.tar.xz"
RUN tar -xf frida-gum-devkit-15.2.2-linux-arm64.tar.xz
RUN mv frida-gum.h /usr/local/include
RUN mv libfrida-gum.a /usr/local/lib

