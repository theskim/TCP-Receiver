# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the client code to the container
COPY ./client .

# Build the client code
RUN cargo build --release

# Set the entrypoint command to run the client
CMD ["cargo", "run", "--release"]