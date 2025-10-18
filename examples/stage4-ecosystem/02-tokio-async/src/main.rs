use tokio::time::{sleep, Duration};

async fn my_async_task(id: i32) {
    println!("任务 {} 开始", id);
    sleep(Duration::from_millis(1000)).await;
    println!("任务 {} 完成", id);
}

#[tokio::main]
async fn main() {
    println!("Tokio 异步演示");

    // --- 任务 --- 
    // 生成一个异步任务
    let task1 = tokio::spawn(my_async_task(1));
    let task2 = tokio::spawn(my_async_task(2));

    println!("\n--- 任务 ---");
    println!("已生成两个异步任务");

    // 等待任务完成
    task1.await.unwrap();
    task2.await.unwrap();

    println!("\n所有任务已完成");

    println!("\n演示完成。");
}