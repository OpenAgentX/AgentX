
use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashSet};

use tracing::{info, debug};
use async_trait::async_trait;


use agent_schema::Message;
use agent_actions::Action;
use agent_memory::Memory;

use crate::template::prefix_template;

/// role setting for agent
#[derive(Debug, Clone, PartialEq)]
pub struct RoleSetting {
    pub name: String,
    pub profile: String,
    pub goal: String,
    pub constraints: String,
    pub desc: String,
}

impl RoleSetting {
    pub fn new(name: &str, profile: &str, goal: &str, constraints: &str, desc: &str) -> Self {
        RoleSetting {
            name: name.to_string(),
            profile: profile.to_string(),
            goal: goal.to_string(),
            constraints: constraints.to_string(),
            desc: desc.to_string(),
        }
    }

    pub fn get_prefix(&self) -> String {
        prefix_template(&self.profile, &self.name, &self.goal, &self.constraints)
    }
}

#[derive(Clone)]
pub struct RoleContext {
    pub env_memory: Arc<Mutex<Memory>>,
    pub role_memory: Arc<Mutex<Memory>>,
    pub state: i32,
    todo: Option<Arc<Mutex<dyn Role + Send + Sync>>>,
    watch: HashSet<String>,
}

impl RoleContext {
    pub fn new(watch: HashSet<String>) -> Self {
         // Initialize fields accordingly
        let state = 1;
        RoleContext {
            env_memory: Arc::new(Mutex::new(Memory::new())),
            role_memory: Arc::new(Mutex::new(Memory::new())),
            state,
            todo: None,
            watch,
        }
    }

    pub fn history(self) -> String{
        let _msgs = self.env_memory.lock().unwrap().get(0);
        "history".to_string()
    }
    /// 获得关注动作对应的信息
    /// The important_memory method also takes a mutable reference to self and returns a vector of Message instances.
    /// It locks the role_memory field, calls the get method of the locked Memory instance on index 0, 
    /// clones each element in the resulting vector, and adds them to the msg_data vector.
    /// Finally, it returns the msg_data vector.
    pub fn important_memory(self) -> Vec<Message>{
        let role_memory = self.role_memory.lock().unwrap();
        let msgs = role_memory.get(0).clone();

        let mut msg_data = Vec::new();
        for msg in msgs.iter() {
            let msg: Message = msg.to_owned().clone();
            msg_data.push(msg);
        }
        // debug!("important_memory: {:?}", msg_data);
        msg_data
    }

}



#[async_trait]
pub trait Role: Send + Sync {
    // Instance method signature
    /// Reset the role.
    fn _reset(&mut self);
    /// Initialize actions for the role.
    fn _init_actions(&mut self, actions: Vec<Box<dyn Action>>);
    /// Watch actions for the role.
    fn _watch(&mut self, actions: Vec<Box<dyn Action>>);
    /// Set the state for the role.
    fn _set_state(&mut self, state: i32);
    /// Get states for the role.
    fn _get_states(&self)-> Vec<String>;
    /// Set the global environment memory for the role.
    fn set_env_global_memory(&mut self, memory: Arc<Mutex<Memory>>);
    /// Get the profile of the role.
    fn _get_profile(&self) -> &str;
    /// Get the role's prefix.
    fn _get_prefix(&self) -> String;
    /// Get the role's context.
    fn _get_rc(&self) -> RoleContext;
    /// Get the environment memory within the role's context.
    fn _get_rc_env_memory(&self) -> MutexGuard<'_, Memory>;
    /// Get the role's memory within the role's context.
    fn _get_rc_memory(&self) -> MutexGuard<'_, Memory>;
    /// Get an action by its state.
    fn _get_action_by_state(&self, state: usize) -> Option<&Box<dyn Action>>;
    /// Get the count of actions for the role.
    fn _get_action_count(&self) -> usize;


// -------------------------------------------------------------------------------
// --------------------- 下面是每个Agent通用的运行逻辑 -------------------------------
// ----------------------------- 不要修改 ------------------------------------------
    /// If the role belongs to the environment, the role's message will be broadcast to the environment.
    fn _publish_message(&self, message: Message) {
        debug!("【{}】publish following message to the environment...", self._get_profile());
        // if not self._rc.env:
        //     # If env does not exist, do not publish the message
        //     return
        let mut env_memory = self._get_rc_env_memory();
        env_memory.add(message.clone());
        // self._rc.env.publish_message(message)
    }

    /// Receive messages from an environment.
    /// - Add the message to the history.
    /// - Store the message in the role's memory.
    fn recv(&self, message: Message) {
        // debug!("Received a message for the agent: {:?}", message.cause_by);
        debug!("【{}】Received a new message from {:?} {:?}", self._get_profile(), message.role, message.cause_by);
        if self._get_rc_memory().has_message(&message, 0) {
            debug!("Message already exists");
            return
        }
        debug!("Message does not exist, storing it in memory for 【{}】", self._get_profile());
        self._get_rc_memory().add(message);
    }

    /// Observe from the environment, gather important information, and add it to memory.
    /// Here, we do not directly receive messages; instead, we retrieve information from the entire environment's records.
    async fn _observe(&self) -> Vec<Message> {
        // Retrieve information from the environment
        info!("Observing from the environment to gather important information");
        // if not self._rc.env:
        //     return 0
        // New messages
        let mut news: Vec<Message> = Vec::new();
        //** Note that env_memory is locked, and you need to release it before calling other functions.
        //** In general, you don't need to manually release the Mutex lock. The lock is automatically released when MutexGuard goes out of scope.
        {
            // Perform data operations while locked
            let env_memory = self._get_rc_env_memory();
            let _env_msgs = env_memory.get(0);
            // debug!("【{}】 Environment information env_msgs {:?}", self._get_profile(), env_memory.storage.clone());
            // env_msgs = self._rc.env.memory.get()
            // Retrieve messages based on the types of actions being watched (watch)
            let observed = env_memory.get_by_actions(self._get_rc().watch);
            // debug!("【{}】 Environment information Subscribed {:?} action information {:?}", self._get_profile(), self._get_rc().watch, observed);
            // Already observed messages
            let role_memory = self._get_rc_memory();
            let already_observed = role_memory.get(0);
            // debug!("【{}】 Already observed information {:?}", self._get_profile(), already_observed);

            for message in observed {
                if !already_observed.contains(&message) {
                    news.push(message.clone())
                }
            }
        }
        // news_text = [f"{i.role}: {i.content[:20]}..." for i in news]
        // if news_text:
        //     logger.debug(f'{self._setting} observed: {news_text}')
        // return len(news)
        news

    }

    /// - Think about what to do next and decide the next action.
    /// - If there's only one action, then that's the only option.
    async fn _think_next_action(&self) -> i32 {
        // "Think about what to do next and decide the next action."
        let next_state = 0;
        debug!("The current Agent has {} actions", self._get_action_count());
        if self._get_action_count() == 1 {
            // If there's only one action, that's the only option
            return next_state
        }
        debug!("Multiple actions available, need to decide which action based on the message");
        // TODO: This section is not yet enabled
        // let prompt = self._get_prefix();
        // // let _history = self._get_role_context().clone().history();
        // let status_prompt = state_template(self._get_rc().history(), self._get_states().join("\n"), self._get_states().len());
        // info!(status_prompt);
        // // Call LLM to choose the next task
        // next_state = self._get_llm().aask(prompt).await;
        // debug!("_think ask llm next_state {}", next_state);
        next_state
    }

    /// Think first (_think) and then act.
    async fn _execute_next_action(&self, action_state: usize) -> Message {
        let important_memory = self._get_rc().clone().important_memory();
        let mut action_result: String = "".into();
        let mut cause_by: String = "".into();

        match self._get_action_by_state(action_state) {
            Some(action) => {
                info!("【{}】action.run, will do  {:?}", self._get_profile(), action.name());
                action_result = action.run(important_memory).await;
                cause_by = action.name().to_owned();
            },
            None => println!("error occurred"),
        }
        info!("【{}】action_result:\n {}", self._get_profile(), &action_result);

        // let response = self._get_role_context().todo.run(important_memory);

        // if self._actions.len() > action_state {
        //     self._actions[action_state];
        // }
    
        // info!("【{}】action_result:\n {}", self._get_profile(),  termimad::inline(&action_result));
        // info!("【{}】action_result:\n {}", self._get_profile(),  termimad::inline(&action_result));

        let msg = Message {
            content: action_result,
            instruct_content: None,
            role: self._get_profile().to_string(),
            cause_by,
            ..Default::default()
        };
        // Store in the environment for all agents to see
        self._get_rc_env_memory().add(msg.clone());
        // Store in the Agent's memory
        self._get_rc_memory().add(msg.clone());
        msg
    }

    /// Think first, then act. The same for every Agent.
    /// _think -> _act
    async fn _react(&self) -> Message {
        debug!("【{}】thinking about the next action...", self._get_profile());
        let next_state = self._think_next_action().await;
        debug!("【{}】executing the next action...  next_state: {}",self._get_profile(), next_state);
        self._execute_next_action(next_state as usize).await
    }

    /// Receive messages and respond with actions.
    async fn handle(&self, message: Message) -> Message {
        // Store in the agent's memory
        self.recv(message);
        info!("No new messages. Waiting");
        self._react().await
    }

    /// Observe, think based on observations, and act.
    /// self.recv(message)
    async fn run(&self, message: Option<Message>) -> Option<Message> {
        // info!("role: {} ---> {:?}", &self.name, message);

        // Store the message
        match message {
            Some(message) => {
                // If there's a message, store it
                self.recv(message);
            }
            None => {
                // If there's no message, observe until a new message appears
                let news = self._observe().await;
                // If there's no new information, suspend and return directly
                if news.is_empty() {
                    info!("No new information. Waiting");
                    return None
                }
                // There are already new messages, continue processing
                // debug!("【{}】New messages: {:?}", self._get_profile(), );
                for msg in &news {
                    // debug!("【{}】New message {:?} sent to recv", self._get_profile(), msg.cause_by);
                    self.recv(msg.clone());
                }
            },
        }
        debug!("---------------New messages to be processed-----------------");
        let rsp = self._react().await;
        // Publish the response to the environment, waiting for the next subscriber to process
        self._publish_message(rsp.clone());
        Some(rsp)
    }


}
