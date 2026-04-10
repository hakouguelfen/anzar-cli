use crate::{shared::configuration::CacheDriver, theme::theme};
use dialoguer::Select;

pub fn select_cache() -> (CacheDriver, String) {
    let caches: Vec<CacheDriver> = vec![CacheDriver::MemCached, CacheDriver::Redis];
    let choice = Select::with_theme(&theme())
        .with_prompt("Select cache")
        .items(&caches)
        .default(0)
        .interact()
        .unwrap();

    let cache = caches[choice].clone();
    let uri = match caches[choice] {
        CacheDriver::MemCached => "memcache://cache:11211",
        CacheDriver::Redis => todo!(),
    };

    (cache, uri.to_string())
}
