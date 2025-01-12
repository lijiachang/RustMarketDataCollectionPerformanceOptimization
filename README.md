# Rust High-Performance Market Data Collector

[中文文档](./README_zh.md)

A high-performance cryptocurrency market data collection system implemented in Rust, demonstrating various optimization techniques and design patterns for low-latency trading applications.

## Project Overview

This project showcases different approaches to collecting and processing cryptocurrency market data with a focus on performance optimization. It implements multiple design patterns and optimization techniques, from basic mutex-based implementations to lock-free concurrent designs.

### Branch Structure

- `main`: Contains the final optimized version
- `step1`: Basic implementation with Mutex and polling
- `step2`: Lock-free implementation with AtomicCell and event-driven processing
   - Replaced Mutex with AtomicCell for lock-free operations
   - Implemented event-driven callback mechanism
   - Optimized JSON parsing with zero-copy extraction
- More branches coming soon...

## Features

Current implementation:
- Memory optimization using Microsoft's mimalloc
- TLS optimization using pure Rust implementation (rustls)
- SIMD-accelerated JSON parsing with sonic-rs
- Thread-safe data sharing mechanisms

Todo:
- Lock-free implementation
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
./target/release/RustMarketDataCollectionPerformanceOptimization
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

## JSON Parser Comparison: Logos vs Sonic-rs

Positioning:

Sonic-rs:
- Purpose: High-performance JSON parser and serialization library
- Main use: JSON data processing
- Features: SIMD acceleration, zero-copy, performance focused

Logos:
- Purpose: Lexer generator
- Main use: Text tokenization and lexical analysis
- Features: Zero-copy, compile-time generation, general purpose

Performance Comparison:
```bash
cargo bench
```

Results on M4 Mac Mini:
```bash
mark_price_parsers/logos parser
                        time:   [47.776 ns 47.927 ns 48.090 ns]
                        change: [-2.4401% -1.9585% -1.4345%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
mark_price_parsers/sonic parser
                        time:   [149.09 ns 149.69 ns 150.31 ns]
                        change: [-1.0662% -0.6079% -0.1409%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
mark_price_parsers/sonic pointer parser
                        time:   [145.28 ns 148.97 ns 152.89 ns]
                        change: [+0.1168% +1.1496% +2.5450%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
```

Performance Analysis:

Why Logos is faster:

    Compile-time generated specialized parsing code
    Minimal parsing principle, only processes required fields
    Avoids full JSON parsing overhead

Sonic-rs advantages:

    Support for complete JSON operations
    Type safety
    SIMD acceleration
    Zero-copy implementation (pointer mode)
