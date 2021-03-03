use super::store::Store;
use std::collections::HashMap;

// TODO handler currently contains more nested error checking than I would like.
// This needs to be cleaned up once the best solution is decided by warp developer
// and `anyhow` crate is hopefully supported to some degree.
// see https://github.com/seanmonstar/warp/issues/307,
// https://github.com/seanmonstar/warp/issues/712 and
// https://github.com/seanmonstar/warp/pull/458
pub async fn count_response(
    query: HashMap<String, String>,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if query.contains_key("url") {
        let url = query.get("url");
        if url.is_some() {
            let uu = url.unwrap();
            let r = store.url_list.read();
            let results = r.get(uu);
            if results.is_some() {
                let ru = results.unwrap();
                let count = vec![ru.len()];
                let resp = serde_json::to_string(&count).unwrap();
                Ok(Box::new(resp))
            } else {
                Ok(Box::new("This url has not been crawled"))
            }
        } else {
            Ok(Box::new("Please enter a valid url such as http://localhost:3030/count/?url=https://www.facebook.com"))
        }
    } else {
        Ok(Box::new("Please enter a valid url such as http://localhost:3030/count/?url=https://www.facebook.com"))
    }
}
