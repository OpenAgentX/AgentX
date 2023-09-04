use std::collections::HashSet;
use std::sync::{Mutex, Arc, MutexGuard};

use async_trait::async_trait;
use tracing::{debug};

use agent_memory::Memory;
use agent_actions::{Action, WritePRD};
use agent_provider::LLM;
use agent_macro::RoleMacro;

use crate::role::{Role, RoleContext, RoleSetting};


#[derive(RoleMacro)]
pub struct ProductManager {
    _llm: Arc<Mutex<LLM>>,
    _setting:  RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    _rc: RoleContext,
}

impl ProductManager {
    pub fn new(name: &str, profile: &str, goal: &str,  constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = WritePRD::new(name, profile,&setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        ProductManager {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            _rc: RoleContext::new(HashSet::from(["BossRequirement".to_string()])),
        }
    }
    pub fn default() -> Self {
        let name = "Alice";
        let profile = "Product Manager";
        let goal = "Efficiently create a successful product";
        let desc = "desc";
        let constraints = "";
        ProductManager::new(name, profile, goal, constraints, desc)
    }
}