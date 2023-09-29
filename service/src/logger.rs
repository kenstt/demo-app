use std::str::FromStr;
use log::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Logger {        // Logger 我們主要使用的物件
#[allow(dead_code)]    // 因為我們的代碼沒呼叫到它，加這個關掉提醒
guard: WorkerGuard,    // 還記得它嗎，就是要存活才能繼續寫log的東西
}

impl Logger {              // 取得 Builder
pub fn builder() -> LoggerBuilder {
    LoggerBuilder::default()    // 等等要實作預設版本
}
}

pub struct LoggerBuilder {  // 這位就是我們的建築工
level: Level,           // log 的等級
packages: Vec<String>,  // log 需要紀錄的套件名字，這次我們用 Vec
}

impl Default for LoggerBuilder { // 給建築工一個預設版本
fn default() -> Self {
    Self {
        level: Level::Info,
        packages: vec![            // 這是巨集，幫我們展開初始化Vec內容
                                   String::from("core"),
                                   String::from("service"),
                                   String::from("web"),
                                   String::from("https"),
                                   String::from("app"),
                                   String::from("warp"),
                                   String::from("tauri"),
                                   String::from("reqwest"),
        ],
    }
}
}

impl LoggerBuilder {    // 實作建築工的工具箱，技能記得點好點滿(?)
pub fn add_package(mut self, package: &str) -> Self {
    self.packages.push(package.to_string());  // vec用push新增元素
    self            //  builder 的方法特性是回傳 self
}

    pub fn remove_package(mut self, package: &str) -> Self {
        self.packages.retain(|p| p != package);    // 移除package名稱
        self
    }

    pub fn try_set_level(mut self, level: &str) -> Self {
        if let Ok(lv) = Level::from_str(level) {    // 如果字串符合 Level 值
            self.level = lv;                        // 設成該level
        }
        // 以上 if let 的寫法等同下面 match 寫法
        // match Level::from_str(level) {
        //     Ok(lv) => {
        //         self.level = lv;
        //     }
        //     _ => {}
        // }
        self
    }

    pub fn use_env(self) -> Self {    // 從我們的設定檔讀取
        match std::env::var("LOG_LEVEL") {
            Ok(lv) => {
                self.try_set_level(&lv)  // try_set..也是回傳self
            }
            Err(_) => self               // 直接回傳self
        }                                // 整個 match 是回傳值
    }

    fn filter(&self) -> EnvFilter {      // 在這裡組合 warp=debug 等字串
        let mut filter = EnvFilter::from_default_env();
        for package in &self.packages {
            let dir = format!("{}={}", package, self.level);
            filter = filter.add_directive(dir.parse().unwrap());
        }
        filter
    }

    pub fn build(self) -> Logger {  // 建造方法，其他方法約等於藍圖，點這個開工
        let file_appender = tracing_appender::rolling::daily("./logs", "log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::registry()
            .with(self.filter())    // 依設定的package和level組篩選器
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking))
            .init();    // 以上幾行幾乎都和之前的一樣
        Logger {        // 只有這個改回傳 Logger 物件
            guard       // 這個欄位是private，外部不可見
        }
    }
}