# Use the official Rust image as the base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Rust project files to the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application in release mode
RUN cargo build --release

FROM ubuntu:latest

# Set the working directory for the application
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/auth-service .

# Expose the port your application listens on
EXPOSE 8080

# Set the default command to run the application
CMD ["./auth-service"]

