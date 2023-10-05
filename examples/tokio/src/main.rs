use rand::random;
use chrono::Utc;

#[tokio::main]
async fn main() {
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