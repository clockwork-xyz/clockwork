FROM ubuntu:18.04

# Set dependency versions.
ENV SOLANA_VERSION=v1.10.8

# Configure path.
ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
ENV PATH="${HOME}/soteria-linux-develop/bin/:${PATH}"

# Install base utilities.
RUN mkdir -p /workdir && \
    mkdir -p /tmp && \
    apt-get update && \
    apt-get upgrade && \ 
    apt-get install -y build-essential git curl wget jq pkg-config libssl-dev libudev-dev

# Move into root.
WORKDIR ${HOME}

# Install Rust.
RUN curl "https://sh.rustup.rs" -sfo rustup.sh && \
    sh rustup.sh -y && \
    rustup component add rustfmt clippy

# Install Solana.
RUN sh -c "$(curl -sSfL https://release.solana.com/${SOLANA_VERSION}/install)"

# Install Soteria.
RUN sh -c "$(curl -k https://supercompiler.xyz/install)"

# Install BPF toolchain.
WORKDIR ${HOME}/.local/share/solana/install/active_release/bin/sdk/bpf
RUN chmod +x env.sh && ./env.sh

# Set workdir.
WORKDIR /workdir
