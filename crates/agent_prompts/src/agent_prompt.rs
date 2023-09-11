pub struct AgentPrompt {
    pub ai_name: String,
    pub ai_role: String,
    pub profile: String,
    pub desc: String,
    pub base_prompt: String,
    pub goals: Vec<String>,
    pub constraints: Vec<String>,
    pub actions: Vec<String>,
    pub tools: Vec<String>,
    pub resources: Vec<String>,
    pub evaluations: Vec<String>,
    pub response_format: String,
}
