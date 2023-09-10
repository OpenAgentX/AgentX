
mod role;
mod template;
mod product_manager;
mod architect;
mod project_manager;
mod engineer;
mod qa_engineer;
mod searcher;
mod role_builder;
mod research_agent;

pub use role::{Role, RoleContext, RoleSetting};
pub use product_manager::ProductManager;
pub use architect::Architect;
pub use project_manager::ProjectManager;
pub use engineer::Engineer;
pub use qa_engineer::QaEngineer;
pub use searcher::Searcher;
pub use role_builder::AgentRoleBuilder;
pub use research_agent::ResearchAgent;