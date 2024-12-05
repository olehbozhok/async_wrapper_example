use tokio::runtime::Runtime;

use crate::config::Config;
use crate::instance_async::AsyncInstance;

// we build this only for not WASM env
pub struct SyncInstance {
    instance: AsyncInstance,
    runtime: Runtime,
}

impl SyncInstance {
    pub fn new(config: Config) -> Self {
        let rt = Runtime::new().unwrap();

        let async_instance = rt.block_on(AsyncInstance::new(config));

        Self {
            instance: async_instance,
            runtime: rt,
        }
    }

    pub fn do_work(&self) -> String {
        self.runtime.block_on(self.instance.do_work())
    }
}
