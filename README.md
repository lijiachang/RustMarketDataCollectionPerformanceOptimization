# Rust High-Performance Market Data Collector

[中文文档](./README_zh.md)

A high-performance cryptocurrency market data collection system implemented in Rust, demonstrating various optimization techniques and design patterns for low-latency trading applications.

## Project Overview

This project showcases different approaches to collecting and processing cryptocurrency market data with a focus on performance optimization. It implements multiple design patterns and optimization techniques, from basic mutex-based implementations to lock-free concurrent designs.

### Branch Structure

- `main`: Contains the final optimized version
- `step1`: Basic implementation with Mutex and polling
- More branches coming soon...

## Features

Current implementation:
- High-performance market data collection from Binance
- Memory optimization using Microsoft's mimalloc
- TLS optimization using pure Rust implementation (rustls)
- SIMD-accelerated JSON parsing with sonic-rs
- Thread-safe data sharing mechanisms

- Lock-free implementation
- Multi-exchange support
- Memory alignment optimization
- Distributed collection
- CPU affinity binding

## Prerequisites

- Rust 1.83.0 or higher
- MacOS 15.2 or higher (Windows not supported)
- Preferably a Tokyo-based server for optimal latency

## Dependencies

```toml
[dependencies]
futures-util = "0.3.31"
mimalloc = "0.1.43"
rustls = { version = "0.23.20", features = ["ring"] }
sonic-rs = "0.3.17"
tokio = { version = "1.42.0", features = ["rt", "rt-multi-thread", "macros", "time"] }
tokio-tungstenite = { version = "0.26.1", features = ["rustls-tls-native-roots"] }
```

## Building and Running

```bash
# Build with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run the program
./target/release/your_binary_name
```

## Performance Optimizations

The project implements several performance optimizations:

1. **Memory Allocation**
    - Uses Microsoft's mimalloc for reduced memory fragmentation
    - Optimized for high-performance computing and low latency
    - Better concurrent performance with reduced lock contention

2. **TLS Implementation**
    - Pure Rust implementation with rustls
    - Lower latency compared to OpenSSL
    - Modern security features without legacy overhead

3. **JSON Processing**
    - SIMD-accelerated parsing with sonic-rs
    - Zero-copy data extraction
    - Optimized memory usage
