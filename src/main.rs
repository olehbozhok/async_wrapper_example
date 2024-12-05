mod config;
mod instance_async;
mod instance_sync;

use config::Config;
use instance_sync::SyncInstance;

fn main() {
    // we use blocking instance that use async instance inside
    let instance = SyncInstance::new(Config {
        message: "some long work".to_string(),
    });

    instance.do_work();
}
