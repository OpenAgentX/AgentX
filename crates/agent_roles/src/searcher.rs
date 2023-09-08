
use std::collections::HashSet;
use std::sync::{Mutex, Arc, MutexGuard};

use async_trait::async_trait;
use tracing::{info, debug};

use agent_macro::RoleMacro;

use agent_schema::Message;
use agent_memory::Memory;
use agent_provider::LLM;
use agent_actions::{Action, SearchAndSummarize};


use crate::role::{Role, RoleContext, RoleSetting};



// pub struct  Store;

#[derive(RoleMacro)]
pub struct Searcher {
    _llm: Arc<Mutex<LLM>>,
    _setting:  RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    _rc: RoleContext,
}

impl Searcher {
    pub fn new(name: &str, profile: &str, goal: &str,  constraints: &str, desc: &str) -> Self {

        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = SearchAndSummarize::new(name, profile,&setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        Searcher {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            _rc: RoleContext::new(HashSet::from(["WritePRD".to_string()])),
        }

    }

    pub fn default() -> Self {
        let name = "Alice";
        let profile = "Smart Assistant";
        let goal = "Provide search services for users";
        let desc = "";
        let constraints = "Answer is rich and complete";

        Searcher::new(name, profile, goal, constraints, desc)
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


// struct Searcher {
//     name: String,
//     profile: String,
//     goal: String,
//     constraints: String,
//     search_and_summarize: SearchAndSummarize,
// }

// impl Searcher {
//     fn new(name: String, profile: String, goal: String, constraints: String) -> Self {
//         let search_and_summarize = SearchAndSummarize::new("", SearchEngineType::CUSTOM_ENGINE, search_func);
//         Searcher {
//             name,
//             profile,
//             goal,
//             constraints,
//             search_and_summarize,
//         }
//     }
    
//     fn set_search_func(&mut self, search_func: SearchFuncType) {
//         let action = SearchAndSummarize::new("", SearchEngineType::CUSTOM_ENGINE, search_func);
//         self.search_and_summarize = action;
//     }
// }

// fn main() {
//     let mut searcher = Searcher::new(
//         "Alice".to_string(),
//         "Smart Assistant".to_string(),
//         "Provide search services for users".to_string(),
//         "Answer is rich and complete".to_string(),
//     );

//     // Assuming you have a search_func defined
//     searcher.set_search_func(search_func);

//     println!("Name: {}", searcher.name);
//     println!("Profile: {}", searcher.profile);
//     println!("Goal: {}", searcher.goal);
//     println!("Constraints: {}", searcher.constraints);
// }
