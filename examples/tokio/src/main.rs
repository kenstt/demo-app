#![allow(clippy::all)]
#![allow(dead_code, unused_variables)]

use rand::random;
use chrono::Utc;

#[tokio::main]
async fn main_5分身() {
    println!("======程式開始======");
    for p in 1..6 {                    // 再多包一層 p for person
        tokio::spawn(async move {      // 這裡必需多一個 move
            for i in 1..6 {
                let rand = random::<u64>() % 1000;
                let delay = std::time::Duration::from_millis(rand);
                tokio::time::sleep(delay).await;
                let padding = " ".repeat((p * 9) as usize);    // 把外面的 p move 進來
                println!(
                    "{:12}: {} \x1b[9{}m分{}-攻{}\x1b[0m",
                    Utc::now().format("%S%.f"),
                    padding, p, p, i);
            }
        });
    }
    for i in 1..6 {                // === 主體 part ===
        let rand = random::<u64>() % 1000;
        let delay = std::time::Duration::from_millis(rand);
        tokio::time::sleep(delay).await;
        println!("{:12}: \x1b[93m主體-攻{}\x1b[0m",
                 Utc::now().format("%S%.f"),
                 i);
    }
    // tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    println!("======程式結束======");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
}

#[tokio::main]
async fn main() {  // mpsc
    println!("======程式開始======");
    let (tx, rx) = std::sync::mpsc::channel();    // 產生事件通道
    let tx1 = tx.clone();               // tx是發送，透過clone產生多個發送者
    tokio::spawn(async move {
        for i in 1..6 {
            let rand = random::<u64>() % 1000;
            let delay = std::time::Duration::from_millis(rand);
            tokio::time::sleep(delay).await;
            let padding = " ".repeat((1 * 14) as usize);
            println!("{:12}: {} \x1b[93m客戶1 tx：{}\x1b[0m",
                     Utc::now().format("%S%.f"), padding, rand);
            tx1.send((i, rand)).unwrap();        // tx1 被move進來，後面不能再使用
        }
    });
    let tx2 = tx.clone();                        // 產生另一個發送者tx
    tokio::spawn(async move {
        for i in 1..6 {
            let rand = random::<u64>() % 1000;
            let delay = std::time::Duration::from_millis(rand);
            tokio::time::sleep(delay).await;
            let padding = " ".repeat((2 * 14) as usize);
            println!("{:12}: {} \x1b[91m客戶2 tx：{}\x1b[0m",
                     Utc::now().format("%S%.f"), padding, rand);
            tx2.send((i, rand)).unwrap();        // 傳遞資料進channel queue
        }
    });
    loop {                                       // 一直在監聽
        let msg = rx.recv().unwrap();            // 如果接收到訊息再往下處理
        let (i, rand) = msg;
        println!("{:12}: \x1b[95m主機rx：{}\x1b[0m",
                 Utc::now().format("%S%.f"), rand);
    }                                            // forever loop
}