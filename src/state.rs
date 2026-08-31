use crate::usage::UsageData;
use std::sync::{OnceLock, RwLock};

static CACHE: OnceLock<RwLock<Option<UsageData>>> = OnceLock::new();

fn lock() -> &'static RwLock<Option<UsageData>> {
    CACHE.get_or_init(|| RwLock::new(None))
}

pub fn store(usage: UsageData) {
    if let Ok(mut g) = lock().write() {
        *g = Some(usage);
    }
}

pub fn get() -> Option<UsageData> {
    lock().read().ok().and_then(|g| g.clone())
}
