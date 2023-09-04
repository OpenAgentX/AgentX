use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};
// use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct SerpAPIWrapper {
    params: HashMap<String, String>,
    serpapi_api_key: Option<String>,
}

impl Default for SerpAPIWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl SerpAPIWrapper {
    pub fn new() -> Self {
        let mut params = HashMap::new();
        params.insert("engine".to_string(), "google".to_string());
        params.insert("google_domain".to_string(), "google.com".to_string());
        params.insert("gl".to_string(), "us".to_string());
        params.insert("hl".to_string(), "en".to_string());

        let serpapi_api_key = std::env::var("SERPAPI_API_KEY").ok();

        SerpAPIWrapper {
            params,
            serpapi_api_key,
        }
    }

    pub async fn run(&self, query: &str) -> Result<String, Box<dyn Error>> {
        let response = self.results(query).await?;
        let result = self._process_response(response);
        Ok(result)
    }

    async fn results(&self, query: &str) -> Result<HashMap<String, serde_json::Value>, reqwest::Error> {
        let mut url = reqwest::Url::parse("https://serpapi.com/search").expect("msg");
        println!("query: {}", query);
        let mut params = self.get_params(query);
        params.insert("source".to_string(), "python".to_string());

        if let Some(serpapi_api_key) = &self.serpapi_api_key {
            params.insert("serp_api_key".to_string(), serpapi_api_key.to_string());
        }

        params.insert("output".to_string(), "json".to_string());
        // println!("{:?}", params);
        url.query_pairs_mut().extend_pairs(params.iter());

        let response = reqwest::get(url).await?.json::<HashMap<String, serde_json::Value>>().await?;
        Ok(response)
    }

    fn get_params(&self, query: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("q".to_string(), query.to_string());

        if let Some(serpapi_api_key) = &self.serpapi_api_key {
            params.insert("api_key".to_string(), serpapi_api_key.to_string());
        }

        params.extend(self.params.clone());
        params
    }

    fn _process_response(&self, response: HashMap<String, serde_json::Value>) -> String {
        // Process response logic here
        serde_json::to_string(&response).expect("msg")
    }
    
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let query = "your_search_query_here";
//     let serpapi_wrapper = SerpAPIWrapper::new();
//     let result = serpapi_wrapper.run(query).await?;
//     println!("Search Result:\n{}", result);
//     Ok(())
// }
