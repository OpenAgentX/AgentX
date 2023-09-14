
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
use agent_tools::SerpAPIWrapper;

pub use agent_provider::{LLM, LLMBase};


const SEARCH_AND_SUMMARIZE_SYSTEM: &str = r#"
### Requirements
1. Please summarize the latest dialogue based on the reference information (secondary) and dialogue history (primary). Do not include text that is irrelevant to the conversation.
- The context is for reference only. If it is irrelevant to the user's search request history, please reduce its reference and usage.
2. If there are citable links in the context, annotate them in the main text in the format [main text](citation link). If there are none in the context, do not write links.
3. The reply should be graceful, clear, non-repetitive, smoothly written, and of moderate length, in {{LANG}}.

### Dialogue History (For example)
A: MLOps competitors

### Current Question (For example)
A: MLOps competitors

### Current Reply (For example)
1. Alteryx Designer: <desc> etc. if any
2. Matlab: ditto
3. IBM SPSS Statistics
4. RapidMiner Studio
5. DataRobot AI Platform
6. Databricks Lakehouse Platform
7. Amazon SageMaker
8. Dataiku
"#;

// const SEARCH_AND_SUMMARIZE_SYSTEM_EN_US: &str = SEARCH_AND_SUMMARIZE_SYSTEM.(LANG="en-us")

const SEARCH_AND_SUMMARIZE_PROMPT: &str = r#"
### Reference Information
{{CONTEXT}}

### Dialogue History
{{QUERY_HISTORY}}
{{QUERY}}

### Current Question
{{QUERY}}

### Current Reply: Based on the information, please write the reply to the Question


"#;


const SEARCH_AND_SUMMARIZE_SALES_SYSTEM: &str = r#"
## Requirements
1. Please summarize the latest dialogue based on the reference information (secondary) and dialogue history (primary). Do not include text that is irrelevant to the conversation.
- The context is for reference only. If it is irrelevant to the user's search request history, please reduce its reference and usage.
2. If there are citable links in the context, annotate them in the main text in the format [main text](citation link). If there are none in the context, do not write links.
3. The reply should be graceful, clear, non-repetitive, smoothly written, and of moderate length, in Simplified Chinese.

# Example
## Reference Information
...

## Dialogue History
user: Which facial cleanser is good for oily skin?
Salesperson: Hello, for oily skin, it is suggested to choose a product that can deeply cleanse, control oil, and is gentle and skin-friendly. According to customer feedback and market reputation, the following facial cleansers are recommended:...
user: Do you have any by L'Oreal?
> Salesperson: ...

## Ideal Answer
Yes, I've selected the following for you:
1. L'Oreal Men's Facial Cleanser: Oil control, anti-acne, balance of water and oil, pore purification, effectively against blackheads, deep exfoliation, refuse oil shine. Dense foam, not tight after washing.
2. L'Oreal Age Perfect Hydrating Cleanser: Added with sodium cocoyl glycinate and Centella Asiatica, two effective ingredients, it can deeply cleanse, tighten the skin, gentle and not tight.
"#;

const SEARCH_AND_SUMMARIZE_SALES_PROMPT: &str = r#"
## Reference Information
{{CONTEXT}}

## Dialogue History
{{QUERY_HISTORY}}
{{QUERY}}
> {{ROLE}}: 

"#;

const SEARCH_FOOD: &str = r#"
# User Search Request
What are some delicious foods in Xiamen?

# Requirements
You are a member of a professional butler team and will provide helpful suggestions:
1. Please summarize the user's search request based on the context and avoid including unrelated text.
2. Use [main text](reference link) in markdown format to **naturally annotate** 3-5 textual elements (such as product words or similar text sections) within the main text for easy navigation.
3. The response should be elegant, clear, **without any repetition of text**, smoothly written, and of moderate length.
"#;




#[derive(Debug, ActionMacro)]
pub struct SearchAndSummarize {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
    search_engine: SerpAPIWrapper,
}
impl SearchAndSummarize {
    pub fn new(name: &str, context: &str, prefix:&str, profile: &str, _llm: Arc<Mutex<dyn LLMBase>>) -> Self {

        Self {
            _llm: Box::new(LLM::new()),
            name: name.into(),
            context: context.into(),
            prefix: prefix.into(),
            profile: profile.into(),
            search_engine: SerpAPIWrapper::default(),
        }
    }

    async fn _build_prompt(&self, msgs: Vec<&Message>) -> String {
        let query = msgs[0].content.as_str();
        let rsp = self.search_engine.run(query).await;

        match rsp {
            Ok(rsp) => {
                let template = PromptTemplate::new(SEARCH_AND_SUMMARIZE_PROMPT);
                let mut args = HashMap::new();
                // TODO 待优化
                args.insert("ROLE", self.profile.as_str());
                args.insert("CONTEXT", rsp.as_str());
                args.insert("QUERY_HISTORY", "main.py");
                args.insert("QUERY", query);
                template.render(&args)
            },
            Err(_) => return "".into(),
        }



    }

    async fn _post_processing(&self, _msgs: Vec<&Message>, llm_response: String) -> String {
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



const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = "";

// class SearchAndSummarize(Action):
//     def __init__(self, name="", context=None, llm=None, engine=None, search_func=None):
//         self.config = Config()
//         self.engine = engine or self.config.search_engine

//         try:
//             self.search_engine = SearchEngine(self.engine, run_func=search_func)
//         except pydantic.ValidationError:
//             self.search_engine = None

//         self.result = ""
//         super().__init__(name, context, llm)

//     async def run(self, context: list[Message], system_text=SEARCH_AND_SUMMARIZE_SYSTEM) -> str:
//         if self.search_engine is None:
//             logger.warning("Configure one of SERPAPI_API_KEY, SERPER_API_KEY, GOOGLE_API_KEY to unlock full feature")
//             return ""

//         query = context[-1].content
//         # logger.debug(query)
//         rsp = await self.search_engine.run(query)
//         self.result = rsp
//         if not rsp:
//             logger.error("empty rsp...")
//             return ""
//         # logger.info(rsp)

//         system_prompt = [system_text]

//         prompt = SEARCH_AND_SUMMARIZE_PROMPT.format(
//             # PREFIX = self.prefix,
//             ROLE=self.profile,
//             CONTEXT=rsp,
//             QUERY_HISTORY="\n".join([str(i) for i in context[:-1]]),
//             QUERY=str(context[-1]),
//         )
//         result = await self._aask(prompt, system_prompt)
//         logger.debug(prompt)
//         logger.debug(result)
//         return result