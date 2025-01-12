
# Rust高性能行情数据采集器

[English](./README.md)

使用Rust实现的高性能加密货币行情数据采集系统，展示了各种优化技术和低延迟交易应用的设计模式。

## 项目概述

本项目展示了不同的加密货币市场数据采集和处理方法，重点关注性能优化。从基本的互斥锁实现到无锁并发设计，实现了多种设计模式和优化技术。

### 分支结构

- `main`: 包含最终优化版本
- `step1`: 基于Mutex和轮询的基础实现
- 更多分支开发中...

## 功能特性

当前实现：
- 币安交易所高性能行情数据采集
- 使用Microsoft的mimalloc进行内存优化
- 使用纯Rust实现的rustls进行TLS优化
- 使用sonic-rs进行SIMD加速的JSON解析
- 线程安全的数据共享机制

- 无锁实现
- 多交易所支持
- 内存对齐优化
- 分布式采集
- CPU亲和性绑定

## 环境要求

- Rust 1.83.0 或更高版本
- MacOS 15.2 或更高版本（不支持Windows）
- 建议使用东京服务器以获得最佳延迟

## 依赖项

```toml
[dependencies]
futures-util = "0.3.31"
mimalloc = "0.1.43"
rustls = { version = "0.23.20", features = ["ring"] }
sonic-rs = "0.3.17"
tokio = { version = "1.42.0", features = ["rt", "rt-multi-thread", "macros", "time"] }
tokio-tungstenite = { version = "0.26.1", features = ["rustls-tls-native-roots"] }
```

## 构建和运行

```bash
# 使用本地CPU优化进行构建
RUSTFLAGS="-C target-cpu=native" cargo build --release

# 运行程序
./target/release/your_binary_name
```

## 性能优化

本项目实现了多项性能优化：

1. **内存分配**
    - 使用Microsoft的mimalloc减少内存碎片
    - 针对高性能计算和低延迟场景优化
    - 通过减少锁竞争提高并发性能

2. **TLS实现**
    - 使用纯Rust实现的rustls
    - 相比OpenSSL具有更低的延迟
    - 现代安全特性，无历史负担

3. **JSON处理**
    - 使用sonic-rs进行SIMD加速解析
    - 零拷贝数据提取
    - 优化的内存使用

## 优化说明

1. **内存优化**
    - 使用mimalloc替代Rust默认内存分配器
    - 优势：
        - 减少内存碎片
        - 性能优化
        - 减少底层锁竞争
        - 跨平台支持
        - 低延迟

2. **TLS优化**
    - 使用rustls替代openssl
    - 优势：
        - 纯Rust实现，延迟更低
        - 简单、现代、安全
        - 去除旧版加密算法
        - 无外部依赖
