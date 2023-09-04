use std::sync::{Mutex, Arc};
use agent_provider::LLMBase;
use agent_schema::Message;
use async_trait::async_trait;
use crate::action_base::Action;

pub struct BossRequirement {
    _llm: Arc<Mutex<dyn LLMBase>>,
    name: String,
    context: String
}

/// Boss Requirement without any implementation details
#[async_trait]
impl Action for BossRequirement {
    fn name(&self) -> &str {
        "BossRequirement"
    }

    fn set_prefix(&mut self, _prefix: &str, _profile: &str ){

    }
    fn get_prefix(&self) -> &str {
        "BossRequirement"
    }
    
    // fn _get_llm(&self) -> MutexGuard<'_, dyn LLMBase> {
    //     self._llm.lock().unwrap()
    // }
    async fn aask(&self, _prompt: &str) -> String {
        "BossRequirement".to_owned()
    }

    async fn run(&self, _prompt: Vec<Message>)-> String {
        "BossRequirement".to_owned()
    }
}
