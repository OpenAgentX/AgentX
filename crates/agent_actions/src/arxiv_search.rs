// use std::env;
use std::io::Write;
use std::{collections::HashMap, sync::{Mutex, Arc}};
use async_trait::async_trait;
use tracing::{debug, info};

use agent_schema::Message;
use agent_prompts::PromptTemplate;
use crate::action_base::Action;
use agent_macro::ActionMacro;
use agent_utils::CodeParser;

pub use agent_provider::{LLM, LLMBase};


const PROMPT_TEMPLATE: &str = r#"
# Context
{{context}}
-----
NOTICE
1. Role: You are an engineer; the main goal is to write PEP8 compliant, elegant, modular, easy to read and maintain Python 3.9 code (but you can also use other programming language)
2. Requirement: Based on the context, implement one following code file, note to return only in code form, your code will be part of the entire project, so please implement complete, reliable, reusable code snippets
3. Attention1: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code.
4. Attention2: If there is any setting, ALWAYS SET A DEFAULT VALUE, ALWAYS USE STRONG TYPE AND EXPLICIT VARIABLE.
5. Attention3: YOU MUST FOLLOW "Data structures and interface definitions". DONT CHANGE ANY DESIGN.
6. Think before writing: What should be implemented and provided in this document?
7. CAREFULLY CHECK THAT YOU DONT MISS ANY NECESSARY CLASS/FUNCTION IN THIS FILE.
Attention: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code and triple quote.

## {{filename}}: Write code with triple quoto. Do your best to implement THIS ONLY ONE FILE. ONLY USE EXISTING API. IF NO API, IMPLEMENT IT.

"#;

// ## {filename}: Please encapsulate your code within triple quotes. Focus your efforts on implementing ONLY WITHIN THIS FILE. Any class or function labeled as MISSING-DESIGN should be implemented IN THIS FILE ALONE. Do NOT make changes to any other files.

const PROMPT_TEMPLATE_RESPONSE_SAMPLE1: &str = r#"
ActionMacro
....
"#;


/// "A wrapper around Arxiv.org "
/// "Useful for when you need to answer questions about Physics, Mathematics, "
/// "Computer Science, Quantitative Biology, Quantitative Finance, Statistics, "
/// "Electrical Engineering, and Economics "
/// "from scientific articles on arxiv.org. "
/// "Input should be a search query."
#[derive(Debug, ActionMacro)]
pub struct SearchArXiv {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl SearchArXiv {
    pub fn new(name: &str, context: &str, prefix:&str, profile: &str, _llm: Arc<Mutex<dyn LLMBase>>) -> Self {

        SearchArXiv {
            _llm: Box::new(LLM::new()),
            name: name.into(),
            context: context.into(),
            prefix: prefix.into(),
            profile: profile.into()
        }
    }
    ///  Use the Arxiv tool.
    async fn _build_prompt(&self, msg: &Vec<Message>) -> String {
        let template = PromptTemplate::new(PROMPT_TEMPLATE);
        let mut args = HashMap::new();
        // TODO 待优化
        args.insert("context", msg[0].content.as_str());
        args.insert("filename", "main.py");
        template.render(&args)
    }

    fn _post_processing(&self, _msgs: &Vec<Message>, llm_response: String) -> String {
        // info!("{}", _msgs[0].content.as_str());
        // save_code
        let code = CodeParser::new().parse_code("main.py", &llm_response, "python").expect("code parsing error");
        info!("_post_processing \n {}", code);
        let mut file = std::fs::File::create("data.py").expect("create failed");
        file.write_all(code.as_bytes()).expect("write failed");
        info!("code written to file" );
        llm_response
    }
}




const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = r#"
"#;
// class BossRequirement(Action):
//     """Boss Requirement without any implementation details"""
//     async def run(self, *args, **kwargs):
//         raise NotImplementedError
