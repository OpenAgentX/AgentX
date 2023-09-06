use std::{collections::HashMap, sync::{Mutex, Arc}, fs, io::Write};
use agent_utils::CodeParser;
use async_trait::async_trait;
use tracing::{debug, info};

use agent_schema::Message;
use agent_prompts::PromptTemplate;
use crate::action_base::Action;
use agent_macro::ActionMacro;
pub use agent_provider::{LLM, LLMBase};


const PROMPT_TEMPLATE: &str = r#"
# Context
{{context}}
-----
Role: You are a project manager; the goal is to break down tasks according to PRD/technical design, give a task list, and analyze task dependencies to start with the prerequisite modules
Requirements: Based on the context, fill in the following missing information, note that all sections are returned in Python code triple quote form seperatedly. Here the granularity of the task is a file, if there are any missing files, you can supplement them
Attention: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code and triple quote.

## Required Python third-party packages: Provided in requirements.txt format

## Required Other language third-party packages: Provided in requirements.txt format

## Full API spec: Use OpenAPI 3.0. Describe all APIs that may be used by both frontend and backend.

## Logic Analysis: Provided as a Python list[str, str]. the first is filename, the second is class/method/function should be implemented in this file. Analyze the dependencies between the files, which work should be done first

## Task list: Provided as Python list[str]. Each str is a filename, the more at the beginning, the more it is a prerequisite dependency, should be done first

## Shared Knowledge: Anything that should be public like utils' functions, config's variables details that should make clear first. 

## Anything UNCLEAR: Provide as Plain text. Make clear here. For example, don't forget a main entry. don't forget to init 3rd party libs.

"#;


#[derive(Debug, ActionMacro)]
pub struct WriteTasks {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl WriteTasks {
    pub fn new(name: &str, context: &str, prefix:&str, profile: &str, _llm: Arc<Mutex<dyn LLMBase>>) -> Self {

        WriteTasks {
            _llm: Box::new(LLM::new()),
            name: name.into(),
            context: context.into(),
            prefix: prefix.into(),
            profile: profile.into()
        }
    }

    async fn _build_prompt(&self, msgs: Vec<&Message>) -> String {
        let template = PromptTemplate::new(PROMPT_TEMPLATE);
        let mut args = HashMap::new();
        // TODO 待优化
        args.insert("context", msgs[0].content.as_str());
        // args.insert("search_information", "");
        let prompt = template.render(&args); 
        // debug!("{:?}", self);
        prompt
    }

    async fn _post_processing(&self, _msgs: Vec<&Message>, llm_response: String) -> String {
        {
            let requirements = CodeParser::new().parse_code("Required Python third-party packages", &llm_response, "plaintext")
                .expect("unable to parse mermaid code for Competitive Quadrant Chart");
            let mut file = fs::File::create("workshop/requirements.txt").unwrap();
            file.write_all(requirements.as_bytes()).expect("failed to write prd.md");
        }
        llm_response
    }
}


const PROMPT_TEMPLATE_RESPONSE_SAMPLE1: &str = r#"
Sure, let's fill in the missing information based on the context provided.

## Required Python third-party packages: Provided in requirements.txt format
```plaintext
curses
....
"#;


const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = r#"
Sure, let's fill in the missing information based on the context provided.

## Required Python third-party packages: Provided in requirements.txt format
```plaintext
curses
```

## Required Other language third-party packages: Provided in requirements.txt format
```plaintext
None
```

## Full API spec: Use OpenAPI 3.0. Describe all APIs that may be used by both frontend and backend.
```yaml
openapi: 3.0.0
info:
  title: Snake Game API
  version: 1.0.0
paths:
  /start_game:
    post:
      summary: Start a new game
      responses:
        '200':
          description: Successfully started a new game
  /move_snake:
    post:
      summary: Move the snake in a specified direction
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                direction:
                  type: string
                  enum: [up, down, left, right]
              required:
                - direction
      responses:
        '200':
          description: Snake successfully moved
```

## Logic Analysis: Provided as a Python list[str, str]. the first is filename, the second is class/method/function should be implemented in this file. Analyze the dependencies between the files, which work should be done first
```python
logic_analysis = [
    ["snake.py", "Snake"],
    ["player.py", "Player"],
    ["food.py", "Food"],
    ["leaderboard.py", "Leaderboard"],
    ["main.py", "__main__"],
]
```

## Task list: Provided as Python list[str]. Each str is a filename, the more at the beginning, the more it is a prerequisite dependency, should be done first
```python
task_list = [
    "snake.py",
    "player.py",
    "food.py",
    "leaderboard.py",
    "main.py",
]
```

## Shared Knowledge: Anything that should be public like utils' functions, config's variables details that should make clear first.
```python
# utils.py
def calculate_speed(score):
    # Calculate game speed based on player's score
    pass

# config.py
MAX_SCORE = 1000
MIN_SCORE = -100
```

## Anything UNCLEAR: Provide as Plain text. Make clear here. For example, don't forget a main entry. don't forget to init 3rd party libs.
```plaintext
- Ensure that the main entry point (`main.py`) properly initializes the game, sets up the necessary components, and handles user input using the `curses` library.
- Remember to initialize the third-party libraries, such as `curses`, properly in the respective files where they are used.
- Keep in mind that the implementation of the `curses` library for the command-line interface might be complex and require careful handling of input and output.
- The game's logic depends on the correct functioning of the Snake, Player, Food, and Leaderboard modules, so implement them according to the provided logic analysis and task list.
- Consider cross-platform compatibility issues when using the `curses` library.
```

With these details, you can start breaking down the tasks, implementing the modules, and ensuring a clear and organized development process for the Snake game project.

"#;
