use super::agent_manager::{AgentIdAccessor, AgentManager};
use async_trait::async_trait;
use anyhow::Result;

/// 系统是全局的，每一轮都可以查看信息
#[async_trait]
pub trait System : Send + Sync {
    async fn update(&mut self, manager: &mut AgentManager, accessor: &mut AgentIdAccessor) -> Result<()>;
}
