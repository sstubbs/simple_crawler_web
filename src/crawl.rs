use super::store::Store;
use simple_crawler::SimpleCrawler;
use std::collections::HashMap;

// TODO handler currently contains more nested error checking than I would like.
// This needs to be cleaned up once the best solution is decided by warp developer
// and `anyhow` crate is hopefully supported to some degree.
// see https://github.com/seanmonstar/warp/issues/307,
// https://github.com/seanmonstar/warp/issues/712 and
// https://github.com/seanmonstar/warp/pull/458
pub async fn crawl_response(
    query: HashMap<String, String>,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if query.contains_key("url") {
        let url = query.get("url").unwrap();
        let simple_crawler = SimpleCrawler::new().url(url);
        if simple_crawler.is_ok() {
            // Do crawl
            let scu = simple_crawler.unwrap().crawl_concurrent(4).await;
            if scu.is_ok() {
                //  Get Results
                let scc = scu.unwrap();
                let results: Vec<String> = scc.urls.keys().cloned().collect();

                // Store them
                store
                    .url_list
                    .write()
                    .insert(url.to_owned(), results.to_owned());
                Ok(Box::new("Successfully crawled url"))
            } else {
                Ok(Box::new("Failed to crawl url"))
            }
        } else {
            Ok(Box::new("Please enter a valid url such as http://localhost:3030/crawl/?url=https://www.facebook.com"))
        }
    } else {
        Ok(Box::new("Please enter a valid url such as http://localhost:3030/crawl/?url=https://www.facebook.com"))
    }
}
