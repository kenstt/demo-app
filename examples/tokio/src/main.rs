use rand::random;
use chrono::Utc;

#[tokio::main]
async fn main() {
    println!("======程式開始======");
    tokio::spawn(async {            // === 分身 part ===
        for i in 1..6 {
            let rand = random::<u64>() % 1000;    // 產生隨機數 0~1000 作為等待豪秒
            let delay = std::time::Duration::from_millis(rand);
            tokio::time::sleep(delay).await;
            let padding = " ".repeat((1 * 9) as usize);   // 插入空白讓版面整齊
            println!("{:12}: {} \x1b[94m分身-攻{}\x1b[0m",
                     Utc::now().format("%S%.f"),
                     padding,
                     i);
        }
    });
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