# This Dockerfile provides a Linux-based environment, pre-installed with Solana dev tooling
# such as Rust, the Solana CLI, and the latest Soteria code scanner.
# 
# You can pull the latest published image from Dockerhub (https://hub.docker.com/r/clockworkxyz/dev)
# Or you can build an image from source using the Docker CLI:
#  ```sh
#  docker build -t clockworkxyz/dev .
#  ```
# 
# Note: When building Docker images on an M1 Mac, you should use the `--platform linux/amd64` flag.
# 

FROM projectserum/build

# Set dependency versions.
ENV SOLANA_VERSION=v1.10.34

# Configure path.
ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
ENV PATH="${HOME}/soteria-linux-develop/bin/:${PATH}"

# Install base utilities.
RUN mkdir -p /workdir && \
    mkdir -p /tmp && \
    apt-get update && \
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

# Set workdir.
WORKDIR /workdir
