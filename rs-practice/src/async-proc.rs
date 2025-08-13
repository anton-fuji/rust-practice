use std::time::Duration;
use tokio::time::sleep;

async fn say(s: &str) {
    for _ in 0..5 {
        // 一時停止・中断ポイント
        sleep(Duration::from_millis(100)).await;
        println!("{s}")
    }
}

#[tokio::main]
async fn main() {
    // tokio::spawnで起動
    let f1 = tokio::spawn(say("World"));
    let f2 = tokio::spawn(say("Hello"));

    f1.await.unwrap();
    f2.await.unwrap();
}
