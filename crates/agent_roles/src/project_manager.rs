use std::collections::HashSet;
use std::sync::{Mutex, Arc, MutexGuard};

use async_trait::async_trait;
use tracing::{info};

use agent_memory::Memory;
use agent_actions::{Action, WriteTasks};
use agent_provider::LLM;
// use agent_macro::RoleMacro;

use crate::role::{Role, RoleContext, RoleSetting};

// #[derive(Clone, Debug)]
pub struct ProjectManager {
    _llm: Arc<Mutex<LLM>>,
    _setting:  RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    _rc: RoleContext,
}

impl ProjectManager {
    pub fn new(name: &str, profile: &str, goal: &str,  constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action = WriteTasks::new(name, profile,&setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);

        ProjectManager {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            _rc: RoleContext::new(HashSet::from(["WriteDesign".to_string()])),
        }
    }

    pub fn default() -> Self {
        let name = "Eve";
        let profile = "Project Manager";
        let goal = "Improve team efficiency and deliver with quality and quantity";
        let desc = "";
        let constraints = "";
        ProjectManager::new(name, profile, goal, constraints, desc)
    }
}

#[async_trait]
impl Role for ProjectManager {

    fn set_env_global_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self._rc.env_memory = memory
    }
    
    fn _reset(&mut self) {
        self._states = vec![];
        self._actions = vec![];
    }

    fn _init_actions(&mut self, actions: Vec<Box<dyn Action>>) {
        self._reset();
        for mut action in actions {
            action.set_prefix(&self._setting.get_prefix(), &self._setting.profile);
            self._actions.push(action);
        }
    }

    fn _watch(&mut self, _actions: Vec<Box<dyn Action>>) {

    }
    fn _set_state(&mut self, _state: i32) {
        // let mut _rc = self._get_role_context();
        // _rc.state = state;
    }
    fn _get_profile(&self) -> &str {
        &self._setting.profile
    }

    fn _get_prefix(&self) -> String {
       self._setting.get_prefix()
    }

    fn _get_states(&self) ->Vec<String> {
        self._states.clone()
    }
    fn _get_rc(&self) -> RoleContext {
        self._rc.clone()
    }
    fn _get_rc_env_memory(&self) -> MutexGuard<'_, Memory> {
        self._rc.env_memory.lock().unwrap()
    }
    fn _get_rc_memory(&self) -> MutexGuard<'_, Memory> {
        self._rc.role_memory.lock().unwrap()
    }
    fn _get_action_by_state(&self, state: usize) -> Option<&Box<dyn Action>> {
        let Some(action) = self._actions.get(state) else { return None };
        Some(action)
    }

    fn _get_action_count(&self) -> usize {
        self._actions.len()
    }
}
