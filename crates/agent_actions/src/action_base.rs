

use async_trait::async_trait;
use agent_schema::Message;

#[async_trait]
pub trait Action: Send + Sync {
    // Placeholder for Action trait
    // fn _get_llm(&self) -> MutexGuard<'_, dyn LLMBase>;
    fn name(&self) -> &str;
    fn set_prefix(&mut self, prefix: &str, profile: &str);
    fn get_prefix(&self) -> &str;
    async fn aask(&self, prompt: &str) -> String;
    /// 这里接收的是 所有信息，但是不是所有行为都会用到
    /// 有些只需要一条，所以使用条件有限制
    /// TODO 需要重新设计这里
    async fn run(&self, prompt: Vec<Message>)-> String;
}