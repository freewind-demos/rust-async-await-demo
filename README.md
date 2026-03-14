# Rust Async Await Demo

## 简介

演示 Rust 异步编程。

## 基本原理

async/await 是 Rust 的异步语法，Future 是惰性的，需要运行时执行。

## 启动和使用

```bash
cargo run
```

## 教程

### async 函数

```rust
async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(Duration::from_millis(100)).await;
    "完成".to_string()
}
```

### 运行异步

```rust
let rt = Runtime::new().unwrap();
rt.block_on(async {
    let result = fetch_data().await;
    println!("{}", result);
});
```

### 并发执行

```rust
let (r1, r2) = tokio::join!(async1(), async2());
```
