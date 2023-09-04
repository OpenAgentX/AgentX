
use agent_roles::Role;
use agent_schema::Message;
use tracing::{debug, info};

use  crate::config::Config;
use crate::environment::Environment;

pub struct SoftwareCompany {
    environment: Environment,
    config: Config,
    investment: f64,
    idea: String,
}

impl SoftwareCompany {
    pub fn new(yaml_file: &str) -> Self {
        // let environment = Arc::new(Mutex::new(Environment::new()));
        let environment = Environment::new();
        SoftwareCompany {
            environment,
            config: Config::new(yaml_file).unwrap(),
            investment: 0.0,
            idea: String::new(),
        }
    }

    pub fn hire(&mut self, roles: Vec<Box<dyn Role>>) {
        // Placeholder for hire method
        self.environment.add_roles(roles);
    }

    pub fn invest(&mut self, _money: f32) {
        // Placeholder for invest method
    }

    pub fn _check_balance(&self) {
        // Placeholder for _check_balance method
    }

    pub fn start_project(&mut self, idea: &str) {
        self.idea = idea.to_owned();
        let first_message = Message {
            content: idea.to_owned(),
            role: "BOSS".to_owned(),
            cause_by: "BossRequirement".to_owned(),
            ..Default::default()
        };

        self.environment.publish_message(first_message)
    }

    pub async fn run(&mut self, mut n_round: i32) -> String {
        // Placeholder for run method
        // while !self.environment.lock().await.message_queue.is_empty() {
        while n_round > 0 {
            self._check_balance();
           
            debug!("n_round: {}", n_round);
            n_round -= 1;
            // Placeholder for logging and round handling
            if n_round == 0 {
                break;
            }
            self.environment.run(n_round.try_into().unwrap()).await;
            // Placeholder for running environment
        }
        // Placeholder for returning history
        String::new()
    }
}

