
use crate::types::{Error, SearchEngine, SearchResult};
use crate::common::{APP_ACCEPT_LANGUAGE, APP_USER_AGENT};

use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use scraper::{ElementRef, Html, Selector};

use async_trait::async_trait;


pub struct GoogleSearchClient;

#[async_trait]
impl SearchEngine for GoogleSearchClient {
    async fn search(&self, query: &str, save_html_page: bool) -> Result<Vec<SearchResult>, Error> {
        let http_client = reqwest::Client::new();
        let req_res = http_client
            .get(format!(
                "https://www.google.com/search?q={}&num=20&lr=en&lr=en",
                query
            ))
            .header(USER_AGENT, APP_USER_AGENT)
            .header(ACCEPT_LANGUAGE, APP_ACCEPT_LANGUAGE)
            .send()
            .await?
            .text()
            .await?;

        if save_html_page {
            std::fs::write(format!("google ({}).html", query), &req_res).unwrap();
        }
        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("a h3").unwrap();

        let results = doc.select(&sel).take(13);

        let results_text = results.map(|x| {
            let mut elem = x;
            let mut prev_elem = elem;

            while elem.value().name() != "a" {
                let p = elem.parent();
                elem = ElementRef::wrap(p.unwrap()).unwrap();
            }
            let url = match GoogleSearchClient::get_target_url(elem.value().attr("href").unwrap()) {
                Some(url) => url,
                None => return None,
            };

            while elem.select(&Selector::parse("a h3").unwrap()).count() <= 1
                && elem.text().fold(0, |acc, a| acc + a.len()) < 700
            {
                let p = elem.parent();
                prev_elem = elem;
                elem = ElementRef::wrap(p.unwrap()).unwrap();
            }
            let texts = prev_elem.text().collect::<Vec<_>>();
            let content = "".to_string();
            // let content = ;

            // let content;
            // if let Ok(res) = extractor::scrape(&url) {
            //     content = res.text;
            // } else {
            //     content = "".to_string();
            // }

            // let content =
            Some(SearchResult {
                title: x.text().collect::<Vec<_>>().join(""),
                url,
                content,
                description: GoogleSearchClient::get_description(texts),
            })
        });

        Ok(results_text
            .filter(|x| x.is_some())
            .take(10)
            .map(|x| x.unwrap())
            .collect())
    }

    fn name(&self) -> String {
        "Google".to_string()
    }
}

fn is_base_domain(text: &str) -> bool {
    //  https://stackoverflow.com
    //  stackoverflow.com
    text.contains(".")
        || (text.starts_with("http://") || text.starts_with("https://"))
        || text.matches("/").count() == 2
}

fn is_url_representation(text: &str) -> bool {
    text.starts_with(" › ")
}

fn is_combo_url(texts: &[&str], idx: usize) -> bool {
    texts.len() > idx + 1 && is_base_domain(texts[idx]) && is_url_representation(texts[idx + 1])
}

fn is_combo_title_and_url(texts: &[&str], idx: usize) -> bool {
    texts.len() > idx + 2 && is_combo_url(texts, idx + 1)
}

fn is_starting_url(text: &str) -> bool {
    if text.matches(" › ").count() == 0 {
        return false;
    }
    let mut tokens = text.split(" › ");
    is_base_domain(&tokens.next().unwrap())
}

fn is_starting_translate_this_page(text: &str) -> bool {
    text == " · "
}

impl GoogleSearchClient {
    fn get_description(texts: Vec<&str>) -> Option<String> {
        // first text is the title
        let texts = &texts[1..];
        let mut description = String::new();
        let mut do_continues = 0;
        let mut adjusted_idx_base = 0;

        for (idx, text) in texts.iter().enumerate() {
            if do_continues > 0 {
                do_continues -= 1;
                continue;
            }
            if is_combo_title_and_url(texts, idx) {
                do_continues = 2;
                adjusted_idx_base = idx + 3;
                continue;
            }
            if is_combo_url(texts, idx) {
                do_continues = 1;
                adjusted_idx_base = idx + 2;
                continue;
            }
            if is_starting_url(text) {
                adjusted_idx_base = idx + 1;
                continue;
            }
            if is_starting_translate_this_page(text) {
                do_continues = 1;
                adjusted_idx_base = idx + 2;
                continue;
            }
            if idx != adjusted_idx_base && !text.starts_with(" ") && texts[idx - 1].ends_with(".") {
                break;
            }
            description.push_str(text);
        }
        if description.is_empty() {
            return None;
        }
        Some(description)
    }

    fn get_target_url(url: &str) -> Option<String> {
        if url.starts_with("/url?q=") {
            Some(url.chars().skip(7).take_while(|x| *x != '&').collect())
        } else if url.starts_with("/") {
            // links going to google.com
            // ex "similar image" results have the same semantics as "related" results
            None
        } else {
            Some(url.to_string())
        }
    }
}
