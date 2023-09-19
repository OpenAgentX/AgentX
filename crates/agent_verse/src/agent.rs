#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoleSetting {
    pub name: String,
    pub profile: String,
    pub goal: String,
    pub constraints: String,
    pub desc: String,
}

impl RoleSetting {
    pub fn new(name: &str, profile: &str, goal: &str, constraints: &str, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            profile: profile.to_string(),
            goal: goal.to_string(),
            constraints: constraints.to_string(),
            desc: desc.to_string(),
        }
    }
    pub fn get_info(&self) -> String {
        format!(
            "You are a {}, named {}, your goal is {}, and the constraint is {}.",
            self.profile, self.name, self.goal, self.constraints
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AgentState {
    Initialization,
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState::Initialization
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Agent {
    pub attrs: RoleSetting,
    pub prompt: String,
    pub alive: bool,
    pub state: AgentState,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            attrs: RoleSetting::default(),
            prompt: String::from("default"),
            alive: true,
            state: AgentState::default(),
        }
    }

    pub fn build(prompt: &str, name: &str, profile: &str, goal: &str, constraints: &str, desc: &str) -> Self {
        Agent {
            attrs: RoleSetting::new(name, profile, goal, constraints, desc),
            prompt: String::from(prompt),
            alive: true,
            state: AgentState::default(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn invalid(&mut self) {
        self.alive = false;
    }

    pub fn reset(&mut self) {
        self.alive = true;
    }
}
