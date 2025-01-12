

use futures_util::{SinkExt, StreamExt};
use sonic_rs::{JsonValueTrait, Value};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

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

#[derive(Default)]
struct Ticker {
    /// 主所(买1, 卖1)
    left_exchange: (f64, f64),
    /// 副所(买1, 卖1)
    right_exchange: (f64, f64),
}

type Tickers = Arc<Mutex<Ticker>>;

#[tokio::main]
async fn main() {
    // 将TLS库由openssl切换至rustls, 这样会带来以下优势
    //    纯Rust实现, 具有较低的延迟
    //    简单,现代, 安全, 减少了内存漏洞
    //    去掉了老旧的加密算法
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");


    let ticker = Arc::new(Tickers::default());
    let ticker_clone_1 = Arc::clone(&ticker);

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

                        let mut temp_ticker = ticker.lock().unwrap();

                        // 保存主所的买一卖一
                        temp_ticker.left_exchange = (
                            root.get("b").as_str().unwrap().parse().unwrap(),
                            root.get("a").as_str().unwrap().parse().unwrap(),
                        );
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

    // 执行策略的线程
    let task = tokio::task::spawn_blocking(move || {
        loop {
            // 获取最新行情
            // 通过Drop trait来达到快速释放锁
            let temp_ticker = {
                let temp_ticker = ticker_clone_1.lock().unwrap();
                // 因为f64是Rust的基本类型,所以实现了Copy trait, 这里会执行按位复制,不会涉及堆分配
                (temp_ticker.left_exchange, temp_ticker.right_exchange)
            };

            // .......这里模拟执行策略
            println!("{:?}", temp_ticker);

            // 大概每10ms轮训一次新行情
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    task.await.unwrap();
}