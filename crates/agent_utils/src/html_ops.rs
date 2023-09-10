use reqwest;
use readability::extractor;
use readability::error::Error;
use url::Url;

pub async fn scrape(url: &str) -> Result<extractor::Product, Error> {
    let body = reqwest::get(url.clone())
        .await
        .map_err(|e| {
            // TODO: error
        }).unwrap()
        .text()
        .await
        .map_err(|e| {
            // TODO: error
        }).unwrap();

    // Need to convert to something that `impl`s `Read`
    let mut res = body.as_bytes();
    let url = Url::parse(&url).unwrap();
    extractor::extract(&mut res, &url)
}