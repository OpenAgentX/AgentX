use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;


use agent_schema::Message;
use agent_memory::Memory;
use agent_roles::Role;
use tracing::info;

/// "Environment, hosting a batch of roles, roles can publish messages to the environment, and can be observed by other roles."
pub struct Environment {
    pub roles: HashMap<String, Box<dyn Role>>,
    pub message_queue: Arc<Mutex<mpsc::Sender<Message>>>,
    pub memory: Arc<Mutex<Memory>>,
    pub history: String,
}

impl Environment {
    pub fn new() -> Self {
        let (tx, _) = mpsc::channel(); // Adjust the channel size as needed
        Environment {
            roles: HashMap::new(),
            message_queue: Arc::new(Mutex::new(tx)),
            memory:  Arc::new(Mutex::new(Memory::new())),
            history: String::new(),
        }
    }
    /// Add a role in the current environment.
    pub fn add_role(&mut self, mut role: Box<dyn Role>) {
        role.set_env_global_memory(self.memory.clone());
        self.roles.insert(role._get_profile().to_string(), role);
    }

    /// Add a batch of characters in the current environment.
    pub fn add_roles(&mut self, roles: Vec<Box<dyn Role>>) {
        for role in roles {
            self.add_role(role);
        }
    }

    // fn set_manager(&mut self, manager: Box<dyn Manager>) {
    //     // Placeholder for set_manager method
    // }

    /// Post information to the current environment.
    pub fn publish_message(&mut self, message: Message) {
        // Placeholder for publish_message method
        self.memory.lock().unwrap().add(message);
    }

    /// Process all Role runs at once.
    pub async fn run(&mut self, _k: usize) {
        // Placeholder for run method
        // let msg = Message::default();
        // let msg = Message::form("Hello", "user", "cause_by", "instruct_content");
        for role in self.roles.values() {
            info!("----------------------------  Running role {:?} -----------------------", role._get_profile());
            role.run(None).await;
        }
    }

    /// Get a specific role within the environment.
    pub fn get_role(&self, name: &str) -> Option<&Box<dyn Role>> {
        self.roles.get(name)
    }
}


impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}