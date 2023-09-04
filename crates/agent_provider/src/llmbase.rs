
use std::fmt;
use async_trait::async_trait;


#[async_trait]
pub trait LLMBase: Send + Sync + fmt::Debug {
    async fn aask(&self, prompt: String) -> String;
}