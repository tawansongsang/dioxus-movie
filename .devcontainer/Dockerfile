FROM fedora:latest as rust

# install base packages for rust developer
RUN dnf update -y \
    && dnf in git ripgrep bat \
    cargo clippy rustup rust-doc rustfmt rust-analyzer \
    rust-src rust-std-static-wasm32-unknown-unknown \
    -y

# Setup variable environment
ENV TZ=Asia/Bangkok \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

FROM rust as final

RUN cargo binstall dioxus-cli
RUN dnf in nodejs npm -y