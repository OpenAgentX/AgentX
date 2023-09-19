use std::io::Write;
use std::{env, fs};
use std::{collections::HashMap, sync::{Mutex, Arc}};
use agent_utils::{CodeParser, async_save_diagram};
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

## Format example
{{format_example}}
-----
Role: You are an architect; the goal is to design a SOTA PEP8-compliant python system; make the best use of good open source tools
Requirement: Fill in the following missing information based on the context, note that all sections are response with code form seperatedly
Max Output: 8192 chars or 2048 tokens. Try to use them up.
Attention: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code and triple quote.

## Implementation approach: Provide as Plain text. Analyze the difficult points of the requirements, select the appropriate open-source framework.

## Python package name: Provide as Python str with python triple quoto, concise and clear, characters only use a combination of all lowercase and underscores

## File list: Provided as Python list[str], the list of ONLY REQUIRED files needed to write the program(LESS IS MORE!). Only need relative paths, comply with PEP8 standards. ALWAYS write a main.rs or app.rs here

## Data structures and interface definitions: Use mermaid classDiagram code syntax, including classes (INCLUDING __init__ method) and functions (with type annotations), CLEARLY MARK the RELATIONSHIPS between classes, and comply with PEP8 standards. The data structures SHOULD BE VERY DETAILED and the API should be comprehensive with a complete design. 

## Program call flow: Use sequenceDiagram code syntax, COMPLETE and VERY DETAILED, using CLASSES AND API DEFINED ABOVE accurately, covering the CRUD AND INIT of each object, SYNTAX MUST BE CORRECT.

## Anything UNCLEAR: Provide as Plain text. Make clear here.

"#;

const FORMAT_EXAMPLE: &str = r#"
---
## Implementation approach
We will ...

## Python package name
```python
"snake_game"
```

## File list
```python
[
    "main.py",
]
```

## Data structures and interface definitions
```mermaid
classDiagram
    class Game{
        +int score
    }
    ...
    Game "1" -- "1" Food: has
```

## Program call flow
```mermaid
sequenceDiagram
    participant M as Main
    ...
    G->>M: end game
```

## Anything UNCLEAR
The requirement is clear to me.
---
"#;

#[derive(Debug, ActionMacro)]
pub struct WriteDesign {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl WriteDesign {
    pub fn new(name: &str, context: &str, prefix:&str, profile: &str, _llm: Arc<Mutex<dyn LLMBase>>) -> Self {

        Self {
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
        args.insert("format_example", FORMAT_EXAMPLE);
        template.render(&args)
    }
    ///save prd.md and competitive_quadrant_chart.png
    async fn _post_processing(&self, _msgs: Vec<&Message>, llm_response: String) -> String {
        info!("【WriteDesign】 llm_response: {}", llm_response);
        // save the prd.md and competitive_quadrant_chart.png
        {
            let mermaid = CodeParser::new().parse_code("Data structures and interface definitions", &llm_response, "mermaid")
                .expect("unable to parse mermaid code for Data structures and interface definitions");

            let res = async_save_diagram(&mermaid, "workshop/Data_structures_and_interface_definitions.png").await;
            if let Ok(_res) = res {
                debug!("save mermaid:\n {}", mermaid);
            } else {
                info!("failed to save workshop/competitive_quadrant_chart.png :\n {}", mermaid);
            }
        }

        {
            let mermaid = CodeParser::new().parse_code("Program call flow", &llm_response, "mermaid")
            .expect("unable to parse mermaid code for Program call flow");

            let res = async_save_diagram(&mermaid, "workshop/Program_call_flow.png").await;
            if let Ok(_res) = res {
                debug!("save mermaid:\n {}", mermaid);
            } else {
                info!("failed to save workshop/competitive_quadrant_chart.png :\n {}", mermaid);
            }
        }
        let mut file = fs::File::create("workshop/ArchitectDesign.md").unwrap();
        file.write_all(llm_response.as_bytes()).expect("failed to write prd.md");
        llm_response
    }

}

// 测试数据
const PROMPT_TEMPLATE_RESPONSE_SAMPLE1: &str = r#"
## Implementation approach
The implementation will focus on using Python...
"#;

const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = r#"
## Implementation approach
The implementation will focus on using Python to create a command-line version of the "Snake" game. We'll utilize the `curses` library for building the interactive command-line interface and managing keyboard input. For data management, we'll use built-in Python data structures and libraries. The game logic will be structured around classes and functions to ensure modularity and maintainability.

## Python package name
```python
snake_game
```

## File list
- `main.py`
- `snake.py`
- `player.py`
- `leaderboard.py`

## Data structures and interface definitions
```mermaid
classDiagram
    class SnakeGame {
        +__init__(self)
        +start(self)
        +game_over(self)
    }

    class Snake {
        +__init__(self, start_pos)
        +move(self, direction)
        +grow(self)
        +collides_with(self, obj)
    }

    class Food {
        +__init__(self)
        +spawn(self)
    }

    class Player {
        +__init__(self, name)
        +increase_score(self, points)
        +get_score(self)
    }

    class Leaderboard {
        +__init__(self)
        +add_score(self, player)
        +get_high_scores(self)
    }
```

## Program call flow
```mermaid
sequenceDiagram
Player->SnakeGame: Start game
SnakeGame->SnakeGame: Initialize game
SnakeGame->Snake: Initialize snake
SnakeGame->Food: Spawn initial food
SnakeGame->Player: Create player
Player->Snake: Set player for snake
SnakeGame->Leaderboard: Create leaderboard
Snake->SnakeGame: Move snake
SnakeGame->Snake: Check collision with food
Snake->Food: Spawn new food
SnakeGame->Snake: Check collision with walls or self
Snake->Player: Increase score if food eaten
SnakeGame->Snake: Update snake position
SnakeGame->SnakeGame: Update game speed
SnakeGame->Snake: Check collision with walls or self
Snake->Player: Decrease player score on collision
Snake->SnakeGame: End game if collision
Player->Leaderboard: Add player's score
Player->Leaderboard: Get high scores
```

## Anything UNCLEAR
The provided design covers the core aspects of the "Snake" game, including game initialization, player interaction, score tracking, collision detection, and leaderboard management. However, the design doesn't include detailed implementation specifics, error handling, or platform-specific considerations. It's important to note that the `curses` library may have compatibility issues with certain environments, and the design might need adaptations to ensure cross-platform compatibility. Also, handling keyboard input and creating the command-line interface using `curses` might be complex and may require careful implementation.
"#;