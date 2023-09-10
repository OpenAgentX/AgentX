
// use url::{percent_encode, percent_encode_byte};
// use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS,};

const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LIB_NAME: &'static str = env!("CARGO_PKG_NAME");
const BASE_URL: &'static str = "https://api.duckduckgo.com/";

pub fn build_quackngo_query_url(query: &str) -> String {
    let mut url = String::from(BASE_URL);
    url.push_str("?q=");

    let query_encoded = format!("{}", utf8_percent_encode(query, CONTROLS));

    url.push_str(&query_encoded);

    url.push_str("&format=json");
    url.push_str("&pretty=0");
    url.push_str("&no_redirect=1");
    url.push_str("&skip_disambig=1");
    url.push_str("&no_html=1");
    url.push_str("&t=");
    url.push_str(LIB_NAME);
    url.push_str("-v");
    url.push_str(LIB_VERSION);

    url.to_string()
}