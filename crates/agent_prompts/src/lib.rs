mod prompt_processing;
// mod auto_agent_instructions;
mod agent_prompt;

pub use prompt_processing::PromptTemplate;
pub use agent_prompt::AgentPrompt;
// pub use auto_agent_instructions::{AutoAgentInstructions};




// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
