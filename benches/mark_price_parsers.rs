use criterion::{black_box, criterion_group, criterion_main, Criterion};

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r#"[ {},:\t\n\f\[\]]+"#)]
enum MarkPriceToken {
    #[token("\"symbol\"")]
    SymbolTag,
    #[token("\"markPrice\"")]
    MarkPriceTag,

    #[regex(r#""[+-]?((\d+\.?\d*)|(\.\d+))""#)]
    Float,

    #[regex("\"[a-zA-Z-_/]*\"")]
    Text,
}

fn parse_mark_price_logos(msg: &str) -> (f64, String) {
    let mut price = 0.0;
    let mut symbol = String::new();

    let mut lex = MarkPriceToken::lexer(msg);
    while let Some(token) = lex.next() {
        match token {
            Ok(MarkPriceToken::MarkPriceTag) => {
                if lex.next().is_some() {
                    let s = lex.slice();
                    price = fast_float2::parse(&s[1..s.len() - 1]).unwrap();
                }
            }
            Ok(MarkPriceToken::SymbolTag) => {
                if lex.next().is_some() {
                    let s = lex.slice();
                    symbol = s[1..s.len() - 1].to_string();
                }
            }
            _ => {}
        }
    }
    (price, symbol)
}

use sonic_rs::{pointer, JsonValueTrait, Value};

fn parse_mark_price_sonic(msg: &str) -> (f64, String) {
    let root: Value = unsafe { sonic_rs::from_slice_unchecked(msg.as_bytes()).unwrap() };

    let price = fast_float2::parse(root["data"]["markPrice"].as_str().unwrap()).unwrap();

    let symbol = root["symbol"].as_str().unwrap().to_string();

    (price, symbol)
}

fn parse_mark_price_sonic_pointer(msg: &str) -> (f64, String) {
    let price = unsafe {
        fast_float2::parse(
            sonic_rs::get_from_slice_unchecked(msg.as_bytes(), pointer!["data", "markPrice"])
                .unwrap()
                .as_str()
                .unwrap(),
        )
            .unwrap()
    };

    let symbol = unsafe {
        sonic_rs::get_from_slice_unchecked(msg.as_bytes(), pointer!["symbol"])
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    };

    (price, symbol)
}

use serde::Deserialize;

#[derive(Deserialize)]
struct MarkPriceData {
    data: MarketData,
    symbol: String,
}

#[derive(Deserialize)]
struct MarketData {
    #[serde(rename = "markPrice")]
    mark_price: String,
}

// 添加serde解析函数, 用于对比
fn parse_mark_price_serde(msg: &str) -> (f64, String) {
    let data: MarkPriceData = serde_json::from_str(msg).unwrap();
    let price = fast_float2::parse(&data.data.mark_price).unwrap();
    (price, data.symbol)
}

fn parse_mark_price_sonic_struct(msg: &str) -> (f64, String) {
    let data: MarkPriceData = sonic_rs::from_str(msg).unwrap();
    let price = fast_float2::parse(&data.data.mark_price).unwrap();
    (price, data.symbol)
}

const MARK_PRICE: &str =
    r#"{"data":{"markPrice":"177.8919459"},"symbol":"SOL/USDT-P","topic":"mark_price"}"#;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mark_price_parsers");

    group.bench_function("serde parser", |b| {
        b.iter(|| parse_mark_price_serde(black_box(MARK_PRICE)))
    });

    group.bench_function("logos parser", |b| {
        b.iter(|| parse_mark_price_logos(black_box(MARK_PRICE)))
    });

    group.bench_function("sonic parser", |b| {
        b.iter(|| parse_mark_price_sonic(black_box(MARK_PRICE)))
    });

    group.bench_function("sonic pointer parser", |b| {
        b.iter(|| parse_mark_price_sonic_pointer(black_box(MARK_PRICE)))
    });
    group.bench_function("sonic struct parser", |b| {
        b.iter(|| parse_mark_price_sonic_struct(black_box(MARK_PRICE)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
