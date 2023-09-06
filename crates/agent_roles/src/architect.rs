use std::collections::HashSet;
use std::sync::{Mutex, Arc, MutexGuard};

use agent_macro::RoleMacro;
use agent_schema::Message;
use async_trait::async_trait;
use tracing::{info, debug};

use agent_memory::Memory;
use agent_actions::{Action, WriteDesign};
use agent_provider::LLM;
// use agent_macro::RoleMacro;

use crate::role::{Role, RoleContext, RoleSetting};



// pub struct  Store;

// #[derive(Clone, Debug)]
#[derive(RoleMacro)]
pub struct Architect {
    _llm: Arc<Mutex<LLM>>,
    _setting:  RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    _rc: RoleContext,
}

impl Architect {
    pub fn new(name: &str, profile: &str, goal: &str,  constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = WriteDesign::new(name, profile,&setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        Architect {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            _rc: RoleContext::new(HashSet::from(["WritePRD".to_string()])),
        }
    }

    pub fn default() -> Self {
        let name = "Bob";
        let profile = "Architect";
        let goal = "Design a concise, usable, complete python system";
        let desc = "";
        let constraints = "Try to specify good open source tools as much as possible";
        Architect::new(name, profile, goal, constraints, desc)
    }

    fn _before_action(&self, env_msgs: &Vec<Message>,  role_msgs: &Vec<Message>) {
        info!(" {:?}", env_msgs);
    }

    fn _after_action(&self, message: Message) -> Message {

        // CodeParser::new().parse_code("Competitive Quadrant Chart", &prd_text, "mermaid");
        // let mermaid = CodeParser::new().parse_code("Competitive Quadrant Chart", &prd_text, "mermaid")
        //     .expect("unable to parse mermaid code for Competitive Quadrant Chart");
        // let _ = async_save_diagram(&mermaid, "workshop/CompetitiveQuadrantChart.png").await;
        // prd_text

        message
        
    }
}
