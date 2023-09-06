use std::collections::HashSet;
// use std::default;
use std::sync::{Mutex, Arc, MutexGuard};

use agent_macro::RoleMacro;
use agent_memory::Memory;
use agent_schema::Message;
use async_trait::async_trait;
use tracing::{info, debug};

use crate::role::{Role, RoleContext, RoleSetting};


use agent_actions::{Action, WriteCode};
use agent_provider::LLM;


// pub struct  Store;

// #[derive(Clone, Debug)]
#[derive(RoleMacro)]
pub struct Engineer {
    _llm: Arc<Mutex<LLM>>,
    _setting:  RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    _rc: RoleContext,
}

impl Engineer {
    pub fn new(name: &str, profile: &str, goal: &str,  constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = WriteCode::new(name, profile,&setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        Engineer {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            _rc: RoleContext::new(HashSet::from(["WriteTasks".to_string()])),
        }
    }

    pub fn default() -> Self {
        let name = "Alex";
        let profile = "Engineer";
        let goal = "Write elegant, readable, extensible, efficient code";
        let desc = "";
        let constraints = "The code you write should conform to code standard like PEP8, be modular, easy to read and maintain";
        Engineer::new(name, profile, goal, constraints, desc)
    }

    fn _before_action(&self, env_msgs: &Vec<Message>,  role_msgs: &Vec<Message>) {
        info!(" {:?}", env_msgs);
    }

    fn _after_action(&self, message: Message) -> Message {
        message
    }
}
