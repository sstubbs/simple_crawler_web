mod count;
mod crawl;
mod list;
mod store;

use anyhow::{Context, Result};
use count::count_response;
use crawl::crawl_response;
use list::list_response;
use std::collections::HashMap;
use store::Store;
use warp::Filter;

pub async fn run_server(port: u16) -> Result<()> {
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

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run_server(3030)
        .await
        .with_context(|| "failed to run server")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_server;
    use anyhow::{Context, Result};
    use tokio::time::{sleep, Duration};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct SimpleCrawlerMock {
        mock_server: MockServer,
    }

    impl SimpleCrawlerMock {
        async fn new() -> Result<Self> {
            Ok(SimpleCrawlerMock {
                mock_server: MockServer::start().await,
            })
        }

        async fn mock(self, meth: &str, url: &str, body: &str) -> Result<Self> {
            Mock::given(method(meth))
                .and(path(url))
                .respond_with(ResponseTemplate::new(200).set_body_string(body))
                .mount(&self.mock_server)
                .await;
            Ok(self)
        }
    }

    async fn setup_mocks() -> Result<(String, u16)> {
        // start the mock server
        let mock = SimpleCrawlerMock::new()
            .await
            .with_context(|| format!("Failed to start mock server"))?;

        // url of mock server
        let mock_url = &mock.mock_server.uri().to_owned();
        let mock_port = &mock.mock_server.address().port().to_owned();

        // setup mocks
        mock
            // mock1
            .mock(
                "GET",
                "/crawl",
                format!(
                    "<a href=\"{mock_url}/crawl2\">aaa</a>\
                <a href=\"{mock_url}/crawl3\">aaa</a>",
                    mock_url = mock_url
                )
                .as_ref(),
            )
            .await
            .with_context(|| format!("Failed to add mock1"))?
            // mock 2
            .mock(
                "GET",
                "/crawl2",
                format!(
                    "<a href=\"{mock_url}/crawl3\">aaa</a>\
                <a href=\"{mock_url}/crawl4\">aaa</a>",
                    mock_url = mock_url
                )
                .as_ref(),
            )
            .await
            .with_context(|| format!("Failed to add mock2"))?
            // mock 3
            .mock(
                "GET",
                "/crawl3",
                format!(
                    "<a href=\"{mock_url}/crawl5\">aaa</a>\
                <a href=\"{mock_url}/crawl6\">aaa</a>",
                    mock_url = mock_url
                )
                .as_ref(),
            )
            .await
            .with_context(|| format!("Failed to add mock3"))?
            // mock 4
            .mock(
                "GET",
                "/crawl4",
                format!(
                    "<a href=\"{mock_url}/crawl7\">aaa</a>\
                <a href=\"{mock_url}/crawl8\">aaa</a>",
                    mock_url = mock_url
                )
                .as_ref(),
            )
            .await
            .with_context(|| format!("Failed to add mock3"))?;
        Ok((mock_url.to_owned(), mock_port.to_owned()))
    }

    fn mock_expected_results(mock_url: &str) -> Vec<String> {
        let mut expected = vec![
            format!("{}/crawl", mock_url),
            format!("{}/crawl2", mock_url),
            format!("{}/crawl3", mock_url),
            format!("{}/crawl4", mock_url),
            format!("{}/crawl5", mock_url),
            format!("{}/crawl6", mock_url),
            format!("{}/crawl7", mock_url),
            format!("{}/crawl8", mock_url),
        ];
        expected.sort();
        expected
    }

    #[tokio::test]
    #[allow(unused_must_use)]
    async fn crawl_list_count_test() -> Result<()> {
        // start mock website server
        let (mock_url, mock_port) = setup_mocks()
            .await
            .with_context(|| format!("Failed to setup mock server"))?;

        // start web app server
        let web_port = mock_port + 1;
        let web_url = format!("http://127.0.0.1:{}", &web_port);
        tokio::spawn(async move {
            run_server(web_port).await;
        });
        // unfortunately sleep is needed as there is no confirmation sent when web server is running
        sleep(Duration::from_millis(500)).await;

        //1. crawl mock domain
        let test1_url = format!("{}/crawl/?url={}/crawl", &web_url, &mock_url);
        let body1 = reqwest::get(&test1_url)
            .await
            .with_context(|| format!("Request failed GET request for {}", &test1_url))?
            .text()
            .await
            .with_context(|| format!("Request failed to extract text for {}", &test1_url))?;
        assert_eq!(body1, "Successfully crawled url");

        //2. list mock domain
        let test2_url = format!("{}/list/?url={}/crawl", &web_url, &mock_url);
        let body2 = reqwest::get(&test2_url)
            .await
            .with_context(|| format!("Request failed GET request for {}", &test2_url))?
            .text()
            .await
            .with_context(|| format!("Request failed to extract text for {}", &test2_url))?;
        let expected1 = mock_expected_results(&mock_url);
        let mut actual1: Vec<String> = serde_json::from_str(&body2)?;
        actual1.sort();
        assert_eq!(expected1, actual1);

        //3. count mock domain
        let test3_url = format!("{}/count/?url={}/crawl", &web_url, &mock_url);
        let body3 = reqwest::get(&test3_url)
            .await
            .with_context(|| format!("Request failed GET request for {}", &test3_url))?
            .text()
            .await
            .with_context(|| format!("Request failed to extract text for {}", &test3_url))?;
        let expected2: Vec<usize> = vec![expected1.len()];
        let actual2: Vec<usize> = serde_json::from_str(&body3)?;
        assert_eq!(expected2, actual2);

        Ok(())
    }
}
