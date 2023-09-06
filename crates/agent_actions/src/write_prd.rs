use std::{collections::HashMap, sync::{Mutex, Arc}, fs, io::Write};
use async_trait::async_trait;
use tracing::{debug, info};

use agent_schema::Message;
use agent_prompts::PromptTemplate;
use crate::action_base::Action;
use agent_macro::ActionMacro;
use agent_utils::{CodeParser, async_save_diagram};

pub use agent_provider::{LLM, LLMBase};

const PROMPT_TEMPLATE: &str = r#"
# Context
## Original Requirements
{{requirements}}

## Search Information
{{search_information}}

## mermaid quadrantChart code syntax example. DONT USE QUOTO IN CODE DUE TO INVALID SYNTAX. Replace the <Campain X> with REAL COMPETITOR NAME
```mermaid
quadrantChart
    title Reach and engagement of campaigns
    x-axis Low Reach --> High Reach
    y-axis Low Engagement --> High Engagement
    quadrant-1 We should expand
    quadrant-2 Need to promote
    quadrant-3 Re-evaluate
    quadrant-4 May be improved
    "Campaign: A": [0.3, 0.6]
    "Campaign B": [0.45, 0.23]
    "Campaign C": [0.57, 0.69]
    "Campaign D": [0.78, 0.34]
    "Campaign E": [0.40, 0.34]
    "Campaign F": [0.35, 0.78]
    "Our Target Product": [0.5, 0.6]
```
-----
Role: You are a professional product manager; the goal is to design a concise, usable, efficient product
Requirements: According to the context, fill in the following missing information, note that each sections are returned in Python code triple quote form seperatedly. If the requirements are unclear, ensure minimum viability and avoid excessive design
ATTENTION: Use '##' to SPLIT SECTIONS, not '#'. AND '## <SECTION_NAME>' SHOULD WRITE BEFORE the code and triple quote.

## Original Requirements: Provide as Plain text, place the polished complete original requirements here

## Product Goals: Provided as Python list[str], up to 3 clear, orthogonal product goals. If the requirement itself is simple, the goal should also be simple

## User Stories: Provided as Python list[str], up to 5 scenario-based user stories, If the requirement itself is simple, the user stories should also be less

## Competitive Analysis: Provided as Python list[str], up to 7 competitive product analyses, consider as similar competitors as possible

## Competitive Quadrant Chart: Use mermaid quadrantChart code syntax. up to 14 competitive products. Translation: Distribute these competitor scores evenly between 0 and 1, trying to conform to a normal distribution centered around 0.5 as much as possible.

## Requirement Analysis: Provide as Plain text. Be simple. LESS IS MORE. Make your requirements less dumb. Delete the parts unnessasery.

## Requirement Pool: Provided as Python list[str, str], the parameters are requirement description, priority(P0/P1/P2), respectively, comply with PEP standards; no more than 5 requirements and consider to make its difficulty lower

## Anything UNCLEAR: Provide as Plain text. Make clear here.

"#;

#[derive(Debug, ActionMacro)]
pub struct WritePRD {
    _llm: Box<dyn LLMBase>,
    name: String,
    context: String,
    prefix: String,
    profile: String,
}
impl WritePRD {
    pub fn new(name: &str, context: &str, prefix:&str, profile: &str, _llm: Arc<Mutex<dyn LLMBase>>) -> Self {

        WritePRD {
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
        args.insert("requirements", msgs[0].content.as_str());
        args.insert("search_information", "");
        let prompt = template.render(&args); 
        prompt
    }
    ///save prd.md and competitive_quadrant_chart.png
    async fn _post_processing(&self, _msgs: Vec<&Message>, llm_response: String) -> String {
        // info!("【WritePRD】 llm_response: {}", llm_response);
        // save the prd.md and competitive_quadrant_chart.png
        let mermaid = CodeParser::new().parse_code("Competitive Quadrant Chart", &llm_response, "mermaid")
            .expect("unable to parse mermaid code for Competitive Quadrant Chart");
        // debug!("mermaid:\n {}", mermaid);
        let res = async_save_diagram(&mermaid, "workshop/competitive_quadrant_chart.png").await;
        if let Ok(_res) = res {
            debug!("save mermaid:\n {}", mermaid);
        } else {
            info!("failed to save workshop/competitive_quadrant_chart.png :\n {}", mermaid);
        }
        let mut file = fs::File::create("workshop/prd.md").unwrap();
        file.write_all(llm_response.as_bytes()).expect("failed to write prd.md");
        llm_response
    }
}

const PROMPT_TEMPLATE_RESPONSE_SAMPLE1: &str = r#"
Sure, let's work through each section step by step.

## Original Requirements
Original Requirements: Write a command-line version of the game "Snake."

....
"#;

const PROMPT_TEMPLATE_RESPONSE_SAMPLE_FULL: &str = r#"
Sure, let's work through each section step by step.

## Original Requirements
Original Requirements: Write a command-line version of the game "Snake."

## Product Goals
Product Goals:
1. Create an engaging and entertaining command-line game that captures the essence of the classic "Snake" game.
2. Ensure smooth and responsive gameplay to provide an enjoyable user experience.
3. Offer a minimalist and intuitive interface that retains the core gameplay mechanics.

## User Stories
User Stories:
1. As a player, I want to control the snake's movement using arrow keys to eat food and grow longer.
2. As a player, I want the game to progressively increase in difficulty as I score higher points.
3. As a player, I want a clear game over message displayed when the snake collides with the wall or itself.
4. As a player, I want a visible score counter that updates in real-time as I collect food.
5. As a player, I want to be able to start a new game quickly after the previous game ends.

## Competitive Analysis
Competitive Analysis:
1. GameX: Simple, addictive gameplay; lacks visual appeal.
2. SnakeMaster: High-quality graphics and customization options; complex controls.
3. RetroSnake: Classic design and controls; lacks modern features.
4. HyperSnake: Fast-paced gameplay with power-ups; overwhelming for new players.
5. UltraSnake: Minimalist design with polished controls; lacks advanced game modes.
6. SuperSnake Deluxe: Extensive levels and challenges; might be overwhelming for casual players.
7. SnakEscape: Innovative level-based gameplay; deviates from classic Snake mechanics.

## Competitive Quadrant Chart
```mermaid
quadrantChart
    title Reach and engagement of Snake games
    x-axis Low Reach --> High Reach
    y-axis Low Engagement --> High Engagement
    quadrant-1 Expanding Stars
    quadrant-2 Promising Players
    quadrant-3 Re-evaluate
    quadrant-4 Potential Growth
    "GameX": [0.25, 0.35]
    "SnakeMaster": [0.6, 0.7]
    "RetroSnake": [0.4, 0.3]
    "HyperSnake": [0.75, 0.8]
    "UltraSnake": [0.55, 0.4]
    "SuperSnake Deluxe": [0.8, 0.6]
    "SnakEscape": [0.7, 0.65]
    "Your Snake Game": [0.6, 0.5]
```

## Requirement Analysis
Requirement Analysis: Develop a command-line Snake game with simple controls, progressive difficulty, real-time scoring, clear game over message, and a quick restart option. Focus on capturing the nostalgic essence of the classic game while offering a smooth and enjoyable experience.

## Requirement Pool
Requirement Pool:
1. Implement snake movement using arrow keys (P0)
2. Design and display food for the snake to consume (P0)
3. Implement collision detection for walls and snake's body (P1)
4. Create a scoring mechanism that updates in real-time (P1)
5. Display a game over message upon collision or game completion (P1)

## Anything UNCLEAR
Please clarify the specific visual and thematic style you envision for the game. Additionally, specify any additional features you'd like to include beyond the core Snake gameplay.

Feel free to let me know if you have any further questions or if you'd like to make any adjustments!
"#;
