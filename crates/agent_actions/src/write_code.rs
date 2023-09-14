
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

pub use agent_provider::{LLM, LLMBase};


const PROMPT_TEMPLATE: &str = r#"
# Context
{{context}}
-----
NOTICE
1. Role: You are an engineer; the main goal is to write PEP8 compliant, elegant, modular, easy to read and maintain Python 3.9 code (but you can also use other programming language)
2. Requirement: Based on the context, implement one following code file, note to return only in code form, your code will be part of the entire project, so please implement complete, reliable, reusable code snippets
3. Attention1: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code.
4. Attention2: If there is any setting, ALWAYS SET A DEFAULT VALUE, ALWAYS USE STRONG TYPE AND EXPLICIT VARIABLE.
5. Attention3: YOU MUST FOLLOW "Data structures and interface definitions". DONT CHANGE ANY DESIGN.
6. Think before writing: What should be implemented and provided in this document?
7. CAREFULLY CHECK THAT YOU DONT MISS ANY NECESSARY CLASS/FUNCTION IN THIS FILE.
Attention: Use '##' to split sections, not '#', and '## <SECTION_NAME>' SHOULD WRITE BEFORE the code and triple quote.

## {{filename}}: Write code with triple quoto. Do your best to implement THIS ONLY ONE FILE. ONLY USE EXISTING API. IF NO API, IMPLEMENT IT.

"#;

// ## {filename}: Please encapsulate your code within triple quotes. Focus your efforts on implementing ONLY WITHIN THIS FILE. Any class or function labeled as MISSING-DESIGN should be implemented IN THIS FILE ALONE. Do NOT make changes to any other files.

const PROMPT_TEMPLATE_RESPONSE_SAMPLE1: &str = r#"
ActionMacro
....
"#;


#[derive(Debug, ActionMacro)]
pub struct WriteCode {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl WriteCode {
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
        args.insert("filename", "main.py");
        template.render(&args)
    }

    async fn _post_processing(&self, _msgs: Vec<&Message>, llm_response: String) -> String {
        // info!("{}", _msgs[0].content.as_str());
        // design = [i for i in context if i.cause_by == WriteDesign][0]
        // save_code
        let code = CodeParser::new().parse_code("main.py", &llm_response, "python").expect("code parsing error");
        info!("_post_processing \n {}", code);
        let mut file = std::fs::File::create("data.py").expect("create failed");
        file.write_all(code.as_bytes()).expect("write failed");
        info!("code written to file" );
        llm_response
    }
}




const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = r#"
## main.py

```python
import pygame
import requests

# Set up the API endpoint URLs
START_GAME_URL = "http://localhost:5000/game/start"
MOVE_SNAKE_URL = "http://localhost:5000/game/move/"
RESTART_GAME_URL = "http://localhost:5000/game/restart"

# Set up the game screen
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
pygame.display.set_caption("Snake Game")

# Set up game loop variables
running = True

def start_game():
    """
    Send a POST request to the API to start the game
    """
    response = requests.post(START_GAME_URL)
    if response.status_code == 200:
        print("Game started successfully")
    else:
        print("Error starting the game")
        
def move_snake(direction):
    """
    Send a POST request to the API to move the snake in the given direction
    """
    url = MOVE_SNAKE_URL + direction
    response = requests.post(url)
    if response.status_code == 200:
        print("Snake moved successfully")
    else:
        print("Error moving the snake")
        
def restart_game():
    """
    Send a POST request to the API to restart the game
    """
    response = requests.post(RESTART_GAME_URL)
    if response.status_code == 200:
        print("Game restarted successfully")
    else:
        print("Error restarting the game")

def main():
    """
    The main game loop
    """
    # Start the game
    start_game()
    
    clock = pygame.time.Clock()
    
    while running:
        # Handle events
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    move_snake("up")
                elif event.key == pygame.K_DOWN:
                    move_snake("down")
                elif event.key == pygame.K_LEFT:
                    move_snake("left")
                elif event.key == pygame.K_RIGHT:
                    move_snake("right")
                elif event.key == pygame.K_r:
                    restart_game()
        
        # Update the screen
        screen.fill((0, 0, 0))
        pygame.display.flip()
        
        clock.tick(60)

if __name__ == "__main__":
    main()
```

The `main.py` file handles the pygame window setup and the main game loop. It uses the requests library to send HTTP requests to the Snake Game API to start the game, move the snake, and restart the game. The `start_game()`, `move_snake(direction)`, and `restart_game()` functions handle sending the requests and parsing the response. The `main()` function contains the main game loop, which handles pygame events and updates the screen. The game loop runs at 60 frames per second.
"#;