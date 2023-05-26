# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app/server

# Copy the server code to the container
COPY ./server .

# Build the server code
RUN cargo build --release

# Set the entrypoint command to run the server
CMD ["cargo", "run", "--release"]