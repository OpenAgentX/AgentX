use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// use lazy_static::lazy_static;
use tracing::{debug, info};
use async_openai::types::ChatCompletionRequestMessage;
use uuid::Uuid;
use serde_json::json;

use agent_schema::Message;
use agent_provider::LLM;
use agent_tools::types::SearchResult;
use agent_actions::{Action, GoogleSearch};
use agent_utils::{
    download_pdf::download_pdf,
    file_ops::{read_txt_files, write_to_file},
    html_ops
};
// use agent_macro::RoleMacro;

use crate::role::{Role, RoleContext, RoleSetting};


// type ReportPrompt = fn(&str, &str) -> String;

// lazy_static! {
//     static ref FUNCTION_MAP: HashMap<&'static str, ReportPrompt> = {
//         let mut map = HashMap::new();
//         map.insert("report_prompt", generate_report_prompt as ReportPrompt);
//         map.insert("report_prompt", generate_resource_report_prompt as ReportPrompt);
//         map.insert("report_prompt", generate_outline_report_prompt as ReportPrompt);
//         map.insert("report_prompt", generate_concepts_prompt as ReportPrompt);
//         map
//     };
// }

#[derive(Default)]
pub struct ResearchAgent {
    _llm: Arc<Mutex<LLM>>,
    _setting: RoleSetting,
    _states: Vec<String>,
    _actions: Vec<Box<dyn Action>>,
    dir_path: PathBuf,
    research_summary: String,
    agent_role_prompt: String,
    question: String,
    directory_name: String,
    search_num_urls: usize
}

impl ResearchAgent {
    pub fn new(name: &str, profile: &str, goal: &str, constraints: &str, desc: &str) -> Self {
        let setting = RoleSetting::new(name, profile, goal, constraints, desc);

        let llm = Arc::new(Mutex::new(LLM::new()));

        let mut action =
            GoogleSearch::new(name, profile, &setting.get_prefix(), profile, llm.clone());
        action.set_prefix(&setting.get_prefix(), profile);
        // self.new(name, profile, goal, constraints)
        let directory_name = Uuid::new_v4().to_string();
        Self {
            _llm: llm,
            _setting: setting,
            _states: vec![],
            _actions: vec![Box::new(action)],
            directory_name,
            search_num_urls: 2,
            ..Default::default()
        }
    }
    pub fn default() -> Self {
        let name = "Alice";
        let profile = "Product Manager";
        let goal = "Efficiently create a successful product";
        let desc = "desc";
        let constraints = "";
        ResearchAgent::new(name, profile, goal, constraints, desc)
    }

    /// Determines what agent should be used
    /// Args:
    //     task (str): The research question the user asked
    /// Returns:
    ///     agent - The agent that will be used
    ///     agent_role_prompt (str): The prompt for the agent
    pub fn choose_agent(&self, task: String) {}

    async fn call_agent(&self, action: &str, stream: bool) -> String {
        let messages = vec![
            json!({"role": "system", "content": &self.agent_role_prompt}),
            json!({"role": "user", "content": action}),
        ];

        let msgs: Vec<ChatCompletionRequestMessage> =
            serde_json::from_value(serde_json::Value::Array(messages)).unwrap();
        debug!("call_agent {:?}", msgs);
        let llm = { self._llm.lock().unwrap() };

        let response = llm.aask_with_role(msgs).await.unwrap();
        info!("call_agent:\n {}", response);
        response
    }

    async fn create_search_queries(&self) -> Vec<String> {
        let result = self
            .call_agent(
                generate_search_queries_prompt(&self.question).as_str(),
                false,
            )
            .await;
        let output = format!(
            "🧠 I will conduct my research based on the following queries: {}...",
            result
        );
        // self.websocket.send_json(&json!({"type": "logs", "output": output})).await.unwrap();
        info!("{}", output);
        serde_json::from_str(&result).unwrap()
    }

    async fn async_browse(&self, url: &str) -> String {
        info!("async_browse {}", url);
        if url.ends_with(".pdf") || url.ends_with(".PDF") {
            let collection: Vec<&str> = url.split("/").collect();
            let name = collection[collection.len() - 1];
            info!("async_browse pdf:\n {}", name);
            // let _ = download_pdf(url, name).await;
            info!("✅ pdf: {}", name);
            return "".to_string();
        }

        let content;
        if let Ok(res) = html_ops::scrape(&url).await {
            content = res.text;
            info!("✅ {}", url);
        } else {
            content = "".to_string();
        }
        debug!("📖 {}", content);
        content
    }

    async fn async_search(&self, query: &str) -> Vec<String> {
        let msg = Message {
            content: query.to_string(),
            role: "ResearchAgent".to_string(),
            cause_by: "ResearchAgent".to_string(),
            instruct_content: None,
            send_to: None,
        };
        let response = self._actions[0].run(vec![&msg]).await;
        // response
        let search_results: Vec<SearchResult> =
            serde_json::from_str(&response).expect("failed to parse the search results. ");

        let new_search_urls: Vec<String> =
            search_results.clone().into_iter().map(|x| x.url).collect();

        let output = format!(
            "🌐 Browsing the following sites for relevant information: {:?}...",
            &new_search_urls
        );
        info!("{}", output);
        // self.websocket.send_json(&json!({"type": "logs", "output": output})).await.unwrap();
        // TODO multi-threaded
        let mut tasks = Vec::new();
        for url in &search_results[0..self.search_num_urls] {
            let task = self.async_browse(&url.url).await;
            tasks.push(task);
        }

        // futures::future::join_all(tasks).await;
        tasks
    }

    async fn run_search_summary(&self, query: &str) -> String {
        // let output = format!("🔎 Running research for '{}'", query);
        // self.websocket.send_json(&json!({"type": "logs", "output": output})).await.unwrap();
        info!("🔎 Running research for '{}'", query);

        let responses = self.async_search(query).await;
        let result = responses.join("\n");

        let dir = format!("./outputs/{}/research-{}.txt", self.directory_name, query);
        let dir_path = std::path::Path::new(&dir);
        let parent_dir = dir_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).unwrap();

        let _ = write_to_file(&dir, &result);

        result
    }

    pub async fn conduct_research(&mut self, task: &str) -> String {
        self.question = task.to_string();
        self.research_summary = if self.dir_path.is_dir() {
            read_txt_files(&self.dir_path)
        } else {
            String::new()
        };

        if self.research_summary.is_empty() {
            let search_queries = self.create_search_queries().await;

            for query in search_queries {
                let research_result = self.run_search_summary(&query).await;
                self.research_summary
                    .push_str(&format!("{}\n\n", research_result));
            }
        }

        let total_words = self.research_summary.split_whitespace().count();
        let output = format!("Total research words: {}", total_words);
        // self.websocket.send_json(&json!({"type": "logs", "output": output})).await.unwrap();
        info!("{}", output);

        self.research_summary.clone()
    }

    pub async fn write_report(&self, report_type: &str) -> String { 
        let prompt = get_report_by_type(report_type)(&self.question, &self.research_summary);
        let output = format!("✍️ Writing {} for research task: {}...", report_type, self.question);
        info!("{}", &prompt);
        info!("{}", &output);
        let report = self.call_agent(&prompt, false).await;

        let dir = format!("./outputs/{}/research_report.md", self.directory_name);
        let dir_path = std::path::Path::new(&dir);
        let parent_dir = dir_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).unwrap();

        let _ = write_to_file(&dir, &report);

        prompt
    } 
}

fn generate_agent_role_prompt(agent: &str) -> String {
    let prompts: HashMap<&str, &str> = [
        (
            "Finance Agent",
            "You are a seasoned finance analyst AI assistant. Your primary goal is to compose comprehensive, astute, impartial, and methodically arranged financial reports based on provided data and trends.",
        ),
        (
            "Travel Agent",
            "You are a world-travelled AI tour guide assistant. Your main purpose is to draft engaging, insightful, unbiased, and well-structured travel reports on given locations, including history, attractions, and cultural insights.",
        ),
        (
            "Academic Research Agent",
            "You are an AI academic research assistant. Your primary responsibility is to create thorough, academically rigorous, unbiased, and systematically organized reports on a given research topic, following the standards of scholarly work.",
        ),
        (
            "Business Analyst",
            "You are an experienced AI business analyst assistant. Your main objective is to produce comprehensive, insightful, impartial, and systematically structured business reports based on provided business data, market trends, and strategic analysis.",
        ),
        (
            "Computer Security Analyst Agent",
            "You are an AI specializing in computer security analysis. Your principal duty is to generate comprehensive, meticulously detailed, impartial, and systematically structured reports on computer security topics. This includes Exploits, Techniques, Threat Actors, and Advanced Persistent Threat (APT) Groups. All produced reports should adhere to the highest standards of scholarly work and provide in-depth insights into the complexities of computer security.",
        ),
        (
            "Default Agent",
            "You are an AI critical thinker research assistant. Your sole purpose is to write well written, critically acclaimed, objective and structured reports on given text.",
        ),
    ]
    .iter()
    .cloned()
    .collect();

    match prompts.get(agent) {
        Some(prompt) => prompt.to_string(),
        None => "No such agent".to_string(),
    }
}

fn generate_report_prompt(question: &str, research_summary: &str) -> String {
    format!(
        "\"\"\"{}\"\"\" Using the above information, answer the following question or topic: \"{}\" in a detailed report -- The report should focus on the answer to the question, should be well structured, informative, in depth, with facts and numbers if available, a minimum of 1,200 words and with markdown syntax and apa format. You MUST determine your own concrete and valid opinion based on the given information. Do NOT deter to general and meaningless conclusions. Write all used source urls at the end of the report in apa format",
        research_summary, question
    )
}

fn generate_search_queries_prompt(question: &str) -> String {
    format!(
        "Write 4 google search queries to search online that form an objective opinion from the following: \"{}\" You must respond with a list of strings in the following format: [\"query 1\", \"query 2\", \"query 3\", \"query 4\"]",
        question
    )
}

fn generate_resource_report_prompt(question: &str, research_summary: &str) -> String {
    format!(
        "\"\"\"{}\"\"\" Based on the above information, generate a bibliography recommendation report for the following question or topic: \"{}\". The report should provide a detailed analysis of each recommended resource, explaining how each source can contribute to finding answers to the research question. Focus on the relevance, reliability, and significance of each source. Ensure that the report is well-structured, informative, in-depth, and follows Markdown syntax. Include relevant facts, figures, and numbers whenever available. The report should have a minimum length of 1,200 words.",
        research_summary, question
    )
}

fn generate_outline_report_prompt(question: &str, research_summary: &str) -> String {
    format!(
        "\"\"\"{}\"\"\" Using the above information, generate an outline for a research report in Markdown syntax for the following question or topic: \"{}\". The outline should provide a well-structured framework for the research report, including the main sections, subsections, and key points to be covered. The research report should be detailed, informative, in-depth, and a minimum of 1,200 words. Use appropriate Markdown syntax to format the outline and ensure readability.",
        research_summary, question
    )
}

fn generate_concepts_prompt(question: &str, research_summary: &str) -> String {
    format!(
        "\"\"\"{}\"\"\" Using the above information, generate a list of 5 main concepts to learn for a research report on the following question or topic: \"{}\". The outline should provide a well-structured framework You must respond with a list of strings in the following format: [\"concepts 1\", \"concepts 2\", \"concepts 3\", \"concepts 4, concepts 5\"]",
        research_summary, question
    )
}

fn generate_lesson_prompt(concept: &str) -> String {
    format!(
        "generate a comprehensive lesson about {} in Markdown syntax. This should include the definition of {}, its historical background and development, its applications or uses in different fields, and notable events or facts related to {}.",
        concept, concept, concept
    )
}

fn get_report_by_type(report_type: &str) -> fn(&str, &str) -> String {
    match report_type {
        "research_report" => generate_report_prompt,
        "resource_report" => generate_resource_report_prompt,
        "outline_report" => generate_outline_report_prompt,
        _ => generate_report_prompt, // Default to generate_report_prompt if unknown type
    }
}

fn auto_agent_instructions() -> String {
    "This task involves researching a given topic, regardless of its complexity or the availability of a definitive answer. The research is conducted by a specific agent, defined by its type and role, with each agent requiring distinct instructions.
        Agent
        The agent is determined by the field of the topic and the specific name of the agent that could be utilized to research the topic provided. Agents are categorized by their area of expertise, and each agent type is associated with a corresponding emoji.

        examples:
        task: \"should I invest in apple stocks?\"
        response: 
        {
            \"agent\": \"💰 Finance Agent\",
            \"agent_role_prompt: \"You are a seasoned finance analyst AI assistant. Your primary goal is to compose comprehensive, astute, impartial, and methodically arranged financial reports based on provided data and trends.\"
        }
        task: \"could reselling sneakers become profitable?\"
        response: 
        { 
            \"agent\":  \"📈 Business Analyst Agent\",
            \"agent_role_prompt\": \"You are an experienced AI business analyst assistant. Your main objective is to produce comprehensive, insightful, impartial, and systematically structured business reports based on provided business data, market trends, and strategic analysis.\"
        }
        task: \"what are the most interesting sites in Tel Aviv?\"
        response:
        {
            \"agent:  \"🌍 Travel Agent\",
            \"agent_role_prompt\": \"You are a world-travelled AI tour guide assistant. Your main purpose is to draft engaging, insightful, unbiased, and well-structured travel reports on given locations, including history, attractions, and cultural insights.\"
        }
    ".to_string()
}
