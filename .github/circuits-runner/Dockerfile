# Base image (Ubuntu 22.04)
FROM ubuntu:22.04

# Install required packages
RUN apt-get update && apt-get install -y \
    unzip wget curl sudo git build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash && \
    echo 'export PATH="$HOME/.bun/bin:$PATH"' >> ~/.bashrc

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Circom
RUN mkdir -p /root/bin && \
    wget https://github.com/iden3/circom/releases/download/v2.1.9/circom-linux-amd64 -O /root/bin/circom && \
    chmod +x /root/bin/circom

# Set PATH
ENV PATH="/root/bin:${PATH}"

# Install Yarn
RUN npm install -g yarn

# Install Git
RUN apt-get update && apt-get install -y git

# Add entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Entrypoint
ENTRYPOINT ["/entrypoint.sh"]