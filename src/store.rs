use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

type Items = HashMap<String, Vec<String>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    base_url: String,
    crawled_urls: Vec<String>,
}

#[derive(Clone)]
pub struct Store {
    pub url_list: Arc<RwLock<Items>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            url_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
