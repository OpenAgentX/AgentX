// use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

// use agent_prompts::PromptTemplate;
// use agent_schema::Message;
use async_openai::types::ChatCompletionRequestMessage;
// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

use agent_actions::{Action, WritePRD};
// use agent_macro::RoleMacro;
// use agent_memory::Memory;
use agent_provider::LLM;

use crate::role::{Role, RoleContext, RoleSetting};

/// reference from the GPT Researcher
const AUTO_AGENT_INSTRUCTIONS: &str = r#"
This task involves researching a given topic, regardless of its complexity or the availability of a definitive answer. The research is conducted by a specific agent, defined by its type and role, with each agent requiring distinct instructions.
Agent
The agent is determined by the field of the topic and the specific name of the agent that could be utilized to research the topic provided. Agents are categorized by their area of expertise, and each agent type is associated with a corresponding emoji.

examples:
task: "should I invest in apple stocks?"
response: 
{
    "agent": "💰 Finance Agent",
    "agent_role_prompt: "You are a seasoned finance analyst AI assistant. Your primary goal is to compose comprehensive, astute, impartial, and methodically arranged financial reports based on provided data and trends."
}
task: "could reselling sneakers become profitable?"
response: 
{ 
    "agent":  "📈 Business Analyst Agent",
    "agent_role_prompt": "You are an experienced AI business analyst assistant. Your main objective is to produce comprehensive, insightful, impartial, and systematically structured business reports based on provided business data, market trends, and strategic analysis."
}
task: "what are the most interesting sites in Tel Aviv?"
response:
{
    "agent:  "🌍 Travel Agent",
    "agent_role_prompt": "You are a world-travelled AI tour guide assistant. Your main purpose is to draft engaging, insightful, unbiased, and well-structured travel reports on given locations, including history, attractions, and cultural insights."
}
"#;
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AgentRole {
    pub agent: String,
    pub agent_role_prompt: String,
}

// #[derive(RoleMacro)]
pub struct AgentRoleBuilder {
    _llm: Arc<Mutex<LLM>>,
    _setting: RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
}

impl AgentRoleBuilder {
    pub fn new(name: &str, profile: &str, goal: &str, constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = WritePRD::new(name, profile, &setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        Self {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
        }
    }
    pub fn default() -> Self {
        let name = "AgentX";
        let profile = "Agent Role Manager";
        let goal = "Efficiently create a Agent role";
        let desc = "desc";
        let constraints = "";
        AgentRoleBuilder::new(name, profile, goal, constraints, desc)
    }

    /// Determines what agent should be used
    /// Args:
    //     task (str): The research question the user asked
    /// Returns:
    ///     agent - The agent that will be used
    ///     agent_role_prompt (str): The prompt for the agent
    pub async fn choose_agent(&self, task: &str) -> AgentRole {
        let messages = vec![
            json!({"role": "system", "content": AUTO_AGENT_INSTRUCTIONS}),
            json!({"role": "user", "content": task}),
        ];

        let msgs: Vec<ChatCompletionRequestMessage> =
            serde_json::from_value(serde_json::Value::Array(messages))
                .expect("task is not allowed");

        let llm = { self._llm.lock().unwrap() };

        let response = llm.aask_with_role(msgs).await.unwrap();

        let agent_role: AgentRole =
            serde_json::from_str(response.as_str()).expect("task is not allowed");

        agent_role
    }
}
