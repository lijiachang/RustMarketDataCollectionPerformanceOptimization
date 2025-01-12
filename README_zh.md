
# Rust高性能行情数据采集器

[English](./README.md)

使用Rust实现的高性能加密货币行情数据采集系统，展示了各种优化技术和低延迟交易应用的设计模式。

## 项目概述

本项目展示了不同的加密货币市场数据采集和处理方法，重点关注性能优化。从基本的互斥锁实现到无锁并发设计，实现了多种设计模式和优化技术。

### 分支结构

- `main`: 包含最终优化版本
- `step1`: 基于Mutex和轮询的基础实现
- `step2`: 基于AtomicCell的无锁实现和事件驱动处理
   - 用AtomicCell替换Mutex实现无锁操作
   - 实现事件驱动的回调机制
   - 增加Bitget交易所数据采集
   - 优化JSON解析，实现零拷贝提取
   - 添加WebSocket心跳维护
- 更多分支开发中...

## 功能特性

当前实现：
- 使用Microsoft的mimalloc进行内存优化
- 使用纯Rust实现的rustls进行TLS优化
- 使用sonic-rs进行SIMD加速的JSON解析
- 线程安全的数据共享机制

- 无锁实现
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
./target/release/RustMarketDataCollectionPerformanceOptimization
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

## JSON解析性能对比: Logos vs Sonic-rs

功能定位

sonic-rs

    定位：高性能 JSON 解析器和序列化库
    主要用途：JSON 数据处理
    特点：SIMD加速，零拷贝，性能优先

logos

    定位：词法分析器（Lexer）生成器
    主要用途：文本分词和词法分析
    特点：零拷贝，编译时生成，通用性强


下面对比下性能差异
```bash
cargo bench
```

在我的电脑中（M4 Mac Mini）得出的结果是：
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
性能分析

    Logos最快的原因：
        编译时生成专用解析代码
        最小解析原则，只处理需要的字段
        避免了完整JSON解析的开销

    Sonic-rs的优势：
        支持完整的JSON操作
        类型安全
        SIMD加速
        零拷贝实现（pointer方式）


文献
- 了解什么是mimalloc?
   https://zhuanlan.zhihu.com/p/671433123

- 了解什么是sonic_rs?
   https://github.com/cloudwego/sonic-rs/blob/main/docs/performance_zh.md

- Logos代码库
  https://github.com/maciejhirsz/logos

- Rust的闭包官方概念
   https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html

- AtomicCell
   https://docs.rs/crossbeam-utils/latest/crossbeam_utils/atomic/struct.AtomicCell.html

- Rust采集行情
  https://mp.weixin.qq.com/s/3ucu3OVzrlgTupmP9zD0OQ