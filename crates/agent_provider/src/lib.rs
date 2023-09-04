mod llmbase;
mod openai;


pub use llmbase::LLMBase;
pub use openai::OpenAIGPTAPI as LLM;