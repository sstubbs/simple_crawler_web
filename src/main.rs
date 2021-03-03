mod count;
mod crawl;
mod list;
mod store;

use anyhow::Result;
use count::count_response;
use crawl::crawl_response;
use list::list_response;
use std::collections::HashMap;
use store::Store;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup an in memory store for crawled results.
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    // crawls a given domain such as http://localhost:3030/crawl/?url=https://www.facebook.com
    let crawl = warp::get()
        .and(warp::path("crawl"))
        .and(warp::query::<HashMap<String, String>>())
        .and(store_filter.clone())
        .and_then(crawl_response);

    // lists urls for a given domain such as http://localhost:3030/list/?url=https://www.facebook.com
    let list = warp::get()
        .and(warp::path("list"))
        .and(warp::query::<HashMap<String, String>>())
        .and(store_filter.clone())
        .and_then(list_response);

    // counts urls for a given domain such as http://localhost:3030/count/?url=https://www.facebook.com
    let count = warp::get()
        .and(warp::path("count"))
        .and(warp::query::<HashMap<String, String>>())
        .and(store_filter.clone())
        .and_then(count_response);

    // routes
    let routes = warp::get().and(crawl.or(list).or(count));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
