
use std::sync::{Arc, Mutex};
use tracing::debug;
use async_trait::async_trait;

use agent_provider::{LLMBase, LLM};
use agent_schema::Message;
use agent_tools::{types::SearchEngine, GoogleSearchClient};
// use agent_macro::ActionMacro;

use crate::action_base::Action;

pub struct GoogleSearch {
    _llm: Box<dyn LLMBase>,
    google_search: Box<dyn SearchEngine>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl GoogleSearch {
    pub fn new(
        name: &str,
        context: &str,
        prefix: &str,
        profile: &str,
        _llm: Arc<Mutex<dyn LLMBase>>,
    ) -> Self {
        Self {
            _llm: Box::new(LLM::new()),
            google_search: Box::new(GoogleSearchClient),
            name: name.into(),
            context: context.into(),
            prefix: prefix.into(),
            profile: profile.into(),
        }
    }
}

/// Boss Requirement without any implementation details
#[async_trait]
impl Action for GoogleSearch {
    fn name(&self) -> &str {
        "GoogleSearch"
    }

    fn set_prefix(&mut self, _prefix: &str, _profile: &str) {}
    fn get_prefix(&self) -> &str {
        "GoogleSearch"
    }

    async fn aask(&self, _prompt: &str) -> String {
        "GoogleSearch".to_owned()
    }

    async fn run(&self, msgs: Vec<&Message>) -> String {
        debug!("GoogleSearch Running {}", msgs[0].content.as_str());
        let response: Vec<agent_tools::types::SearchResult> = self
            .google_search
            .search(msgs[0].content.as_str(), false)
            .await
            .unwrap();
        debug!("GoogleSearch successfully");
        serde_json::to_string(&response).unwrap_or("".to_string())
    }
}
