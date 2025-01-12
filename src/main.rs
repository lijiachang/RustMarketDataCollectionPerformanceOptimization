

use futures_util::{SinkExt, StreamExt};
use sonic_rs::{pointer, JsonValueTrait, Value};
use std::sync::Arc;
use std::time::Duration;
use crossbeam_utils::atomic::AtomicCell;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use mimalloc::MiMalloc;
// MiMalloc通常适用于高性能计算、低延迟、高并发或需要精细内存控制的应用。我们替换了Rust自带的内存分配器, 改用微软现代的内存分配器, 可以带来以下的优势
//    减少内存碎片
//    性能优化
//    减少底层锁竞争,提高并发性能
//    跨平台
//    低延迟

#[global_allocator] // 这是一个特殊的属性宏，用于指定全局内存分配器 Rust 允许我们通过这个属性替换默认的系统分配器 一个程序中只能有一个全局分配器

static GLOBAL: MiMalloc = MiMalloc;  //MiMalloc 实现了 Rust 的 GlobalAlloc trait 这行代码创建了 mimalloc 分配器的全局实例
// 当程序启动时，Rust 运行时会使用被 #[global_allocator] 标记的分配器
// 所有的内存分配操作（包括 Box、Vec、String 等）都会使用这个分配器
// 这个替换是全局性的，影响整个程序的内存分配

#[derive(Default, Debug)]
struct Ticker {
    /// 主所(买1, 卖1)
    left_exchange: (AtomicCell<f64>, AtomicCell<f64>),
    /// 副所(买1, 卖1)
    right_exchange: (AtomicCell<f64>, AtomicCell<f64>),
}
//  AtomicCell相比普通的Mutex的优势
//   使用原子操作来实现线程安全, 避免上下文阻塞
//   由于不需要锁的释放, 通常比Mutex快, 尤其是读写频繁的场景
//   简单易用
//   无死锁风险,内部不使用锁机制
//   较小的内存开销
//   跨线程共享, 可以直接在多线程中共享而无需包装
// 我们设计的数据结构很简单, 不涉及复杂类型, 从实现来看, 就已经满足Copy trait, 符合AtomicCell设计原则, 也符合高性能, 低延迟读写的场景


#[tokio::main]
async fn main() {
    // 将TLS库由openssl切换至rustls, 这样会带来以下优势
    //    纯Rust实现, 具有较低的延迟
    //    简单,现代, 安全, 减少了内存漏洞
    //    去掉了老旧的加密算法
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");


    let ticker = Arc::new(Ticker::default());
    let ticker_clone_1 = Arc::clone(&ticker);
    let ticker_clone_2 = Arc::clone(&ticker);

    // 策略回调
    let strategy_callback = Arc::new(move || {
        // 读取行情
        let bid1 = ticker.left_exchange.0.load();
        let ask1 = ticker.left_exchange.1.load();

        let bid2 = ticker.right_exchange.0.load();
        let ask2 = ticker.right_exchange.1.load();

        // ... 模拟执行策略
        println!("{:?}", (bid1, ask1, bid2, ask2));
    });

    let strategy_callback_clone = Arc::clone(&strategy_callback);


    // 某安采集线程
    tokio::spawn(async move {
        loop {
            // 连接某安
            let (ws_stream, _) = connect_async("wss://fstream.binance.com/ws/btcusdt@bookTicker")
                .await
                .unwrap();

            // 切分流
            let (mut write, mut read) = ws_stream.split();

            loop {
                // 处理币安传来的消息
                match read.next().await.unwrap().unwrap() {
                    Message::Text(data) => {
                        // 通过利用sonic_rs的simd指令特性, 来通过unsafe方法跳过检验json完整性,生成document对象, 然后提取相应的买1和卖1数据, 这样的话,通过unsafe可以节省至少一倍左右的时间

                        // 将数据转换成document对象
                        // 解析到sonic_rs::Value后，底层是一个 Key-Value pair 的数组，而不会建立 HashMap 或 BTreeMap, 因此没有建表开销。
                        let root: Value =
                            unsafe { sonic_rs::from_slice_unchecked(data.as_bytes()).unwrap() };

                        // 无锁写入
                        ticker_clone_1
                            .left_exchange
                            .0
                            .store(root.get("b").as_str().unwrap().parse().unwrap());
                        ticker_clone_1
                            .left_exchange
                            .1
                            .store(root.get("a").as_str().unwrap().parse().unwrap());

                        // 执行策略回调
                        strategy_callback()
                    }
                    Message::Ping(_) => {
                        // 收到ping, 回复pong
                        write.send(Message::Pong(Default::default())).await.unwrap();
                    }
                    _ => break,
                }
            }
        }
    });

    // Bitget采集线程
    let task = tokio::spawn(async move {
        loop {
            // 连接Bitget
            let (ws_stream, _) = connect_async("wss://ws.bitget.com/v2/ws/public")
                .await
                .unwrap();

            // 切分流
            let (mut write, mut read) = ws_stream.split();

            // 订阅btc频道
            let data = sonic_rs::json!({
                "op": "subscribe",
                "args": [
                    {
                        "instType": "USDT-FUTURES",
                        "channel": "books1",
                        "instId": "BTCUSDT"
                    }
                ]
            });
            write
                .send(Message::Text(Utf8Bytes::from(data.to_string())))
                .await
                .unwrap();
            // data.to_string() - 将JSON数据转换为字符串
            // Utf8Bytes::from() - 将字符串转换为UTF-8编码的字节序列
            // Message::Text() - 创建一个WebSocket文本消息


            // 定时ping
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(30)).await;

                    write
                        .send(Message::Text(Utf8Bytes::from("ping")))
                        .await
                        .unwrap();
                }
            });

            // 跳过 订阅成功回调
            read.next().await.unwrap().unwrap();

            loop {
                // 处理Bitget传来的消息
                while let Ok(data) = read.next().await.unwrap() {
                    // 跳过pong心跳
                    if data.len() == 4 {
                        continue;
                    }

                    if let Message::Text(data) = data {
                        // 这里我展示了另外一种提取数据的手段
                        // 优点是零拷贝
                        let bid: f64 = unsafe {
                            sonic_rs::get_from_slice_unchecked(
                                data.as_bytes(),
                                pointer!["data", 0, "bids", 0, 0],
                            )
                                .unwrap()
                        }
                            .as_str()
                            .unwrap()
                            .parse()
                            .unwrap();
                        //     data.as_bytes() - 将文本数据转换为字节切片
                        //     pointer!["data", 0, "bids", 0, 0] - 定义JSON路径
                        //         "data" - 最外层键
                        //         0 - 第一个数组元素
                        //         "bids" - bids数组
                        //         0 - 第一个报价
                        //         0 - 价格（第一个字段）

                        let ask: f64 = unsafe {
                            sonic_rs::get_from_slice_unchecked(
                                data.as_bytes(),
                                pointer!["data", 0, "asks", 0, 0],
                            )
                                .unwrap()
                        }
                            .as_str()
                            .unwrap()
                            .parse()
                            .unwrap();

                        // 无锁写入
                        ticker_clone_2.right_exchange.0.store(bid);
                        ticker_clone_2.right_exchange.1.store(ask);

                        // 执行策略回调
                        strategy_callback_clone();

                        continue;
                    }

                    // 占位符, 不会走到这里
                    todo!()
                }
            }
        }
    });

    task.await.unwrap();
}