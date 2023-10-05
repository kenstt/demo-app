
#[tokio::main]
async fn main() {
    println!("======程式開始======");        // 標注讓我們知道發生什麼事
    tokio::spawn(async {                    // 分身詠唱之術
        for i in 1..6 {
            println!("分身-攻{}", i);        // 模擬分身做一些事情
        }
    });
    println!("======程式結束======");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap(); // 等待終端輸入的資料
}