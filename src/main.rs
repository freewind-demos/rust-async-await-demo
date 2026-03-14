fn main() {
    println!("=== Rust async/await 异步编程演示 ===\n");

    // 注意：需要使用 tokio 或其他异步运行时
    // 这里演示基本概念，实际运行需要异步运行时

    // 1. async/await 基本语法
    println!("--- async/await 基础 ---");

    // 2. Tokio 运行时
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 使用 block_on 运行异步代码
    rt.block_on(async {
        let result = fetch_data().await;
        println!("获取到的数据: {}", result);
    });

    // 3. 并发执行多个异步任务
    println!("\n--- 并发执行 ---");
    rt.block_on(async {
        // 使用 tokio::join! 并发执行
        let (result1, result2) = tokio::join!(
            fetch_data(),
            fetch_data2()
        );
        println!("结果1: {}, 结果2: {}", result1, result2);
    });

    // 4. 并发创建任务
    println!("\n--- tokio::spawn ---");
    rt.block_on(async {
        let handle = tokio::spawn(async {
            slow_async_function().await
        });

        let result = handle.await.unwrap();
        println!("spawn 结果: {}", result);
    });

    // 5. Select（同时等待多个 futures）
    println!("\n--- tokio::select! ---");
    rt.block_on(async {
        tokio::select! {
            result = fast_future() => {
                println!("fast future 完成: {}", result);
            }
            result = slow_future() => {
                println!("slow future 完成: {}", result);
            }
        }
    });

    // 6. Stream 处理
    println!("\n--- 异步流 ---");
    rt.block_on(async {
        use tokio::sync::mpsc;

        let (tx, mut rx) = mpsc::channel(10);

        // 发送端
        tokio::spawn(async move {
            for i in 1..=3 {
                tx.send(i).await.unwrap();
            }
        });

        // 接收端
        while let Some(msg) = rx.recv().await {
            println!("收到: {}", msg);
        }
    });

    println!("\n=== 总结 ===");
    println!("async/await 是 Rust 的异步编程语法");
    println!("Future 是惰性的，需要运行时执行");
    println!("tokio 是最流行的异步运行时");
    println!("join! 并发执行多个异步任务");
}

// 异步函数
async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    "数据加载完成".to_string()
}

async fn fetch_data2() -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    "第二个数据".to_string()
}

async fn slow_async_function() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    42
}

async fn fast_future() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    1
}

async fn slow_future() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    2
}
