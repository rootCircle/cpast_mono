FROM debian:bookworm-slim

# Update package lists and install necessary build tools and compilers
RUN apt-get update && apt-get install -y \
    build-essential \
    gcc \
    g++ \
    openjdk-17-jdk \
    python3 \
    python3-pip \
    ruby \
    curl \
    --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*  # Clean up APT cache to reduce image size

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Security: Create a non-root user and group (appuser)
RUN groupadd -r appuser && useradd -r -s /bin/bash -g appuser appuser

# Set the default user to the non-root user
USER appuser

# Define the default command (this is just an example, replace with your actual startup command)
CMD ["g++", "--help"]

