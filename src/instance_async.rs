use std::time::Duration;

use crate::config::Config;

// is used for WASM and not WASM environment
pub struct AsyncInstance {
    config: Config,
}

impl AsyncInstance {
    pub async fn new(config: Config) -> Self {
        // do some async work
        println!("start async init work");
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("done async init work");
        Self { config }
    }

    pub async fn do_work(&self) -> String {
        println!("start do work");
        tokio::time::sleep(Duration::from_secs(2)).await;
        let msg = format!("{} done", self.config.message);
        println!("{msg}");
        println!("done do work");

        msg
    }
}
