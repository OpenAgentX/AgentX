
mod role;
mod template;
mod product_manager;
mod architect;
mod project_manager;
mod engineer;
mod qa_engineer;
// mod searcher;

pub use role::{Role, RoleContext, RoleSetting};
pub use product_manager::ProductManager;
pub use architect::Architect;
pub use project_manager::ProjectManager;
pub use engineer::Engineer;
pub use qa_engineer::QaEngineer;
// pub use searcher::Searcher;