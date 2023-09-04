use std::error::Error;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;




lazy_static! {
    /// Regex to match placeholders. The pattern matches antyhing between "{{" and "}}". No new line is allowed in the placeholder name.
    ///
    /// TODO: when `LazyCell` is stabilized, use that instead
    pub(crate) static ref PARSE_CODE_MATCH_RE: Regex = Regex::new(r"\{\{.*?\}\}").unwrap();
    pub(crate) static ref PLACEHOLDER_MATCH_RE: Regex = Regex::new(r"```python.*?\s+(.*?)```").unwrap();
}

pub struct CodeParser {
    text: String,
    // pub matches: Vec<(usize, usize)>,
}

impl Default for CodeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeParser {
    pub fn new() -> Self {
        CodeParser {
            text: String::new(),
        }
    }

    pub fn set_text(&mut self, template: &str) {
        self.text = template.to_owned();
    }

    pub fn parse_block(&self, block: &str, text: &str) -> Result<String, Box<dyn Error>> {
        let blocks = self.parse_blocks(text)?;
        // println!("parse_block: {} : {:?}", block, blocks);
        for (k, v) in blocks.iter() {
            if k.contains(block) {
                return Ok(v.clone());
            }
        }
        Ok("".to_string())
        // if let Some(content) = blocks.get(block) {
        //     Ok(content.clone())
        // } else {
        //     Ok("".to_string())
        // }
    }

    pub fn parse_blocks(&self, text: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut block_dict = HashMap::new();
        let mut lines = text.lines();
        let mut current_title = String::new();
        let mut current_content = String::new();
    
        while let Some(line) = lines.next() {
            //  split block by "##"
            if line.starts_with("##") {
                if !current_title.is_empty() {
                    block_dict.insert(current_title.trim().to_string(), current_content.trim().to_string());
                }
                current_title = line[2..].trim().to_string();
                current_content.clear();
            } else {
                current_content.push_str(line);
                current_content.push('\n'); // Add a newline to separate lines
            }
        }
    
        if !current_title.is_empty() {
            block_dict.insert(current_title.trim().to_string(), current_content.trim().to_string());
        }
    
        Ok(block_dict)
    }

    pub fn parse_code(&self, block: &str, text: &str, lang: &str) -> Result<String, Box<dyn Error>> {
        let text = self.parse_block(block, text)?;
       
        let pattern = format!(r"```{}\s*([\s\S]*?)```", lang);
        let re = Regex::new(&pattern).unwrap();

        if let Some(captures) = re.captures(&text) {
            let code = captures.get(1).unwrap().as_str().to_string();
            Ok(code)
        } else {
            Err("Pattern not found".into())
        }
    }

    pub fn parse_str(&self, block: &str, text: &str, lang: &str) -> Result<String, Box<dyn Error>> {
        let code = self.parse_code(block, text, lang)?;
        let mut code = code.split('=').last().unwrap_or("").trim();
        if code.starts_with('\'') || code.starts_with('"') {
            code = &code[1..code.len() - 1];
        }
        Ok(code.to_string())
    }

    pub fn parse_file_list(&self, block: &str, text: &str, lang: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let code = self.parse_code(block, text, lang)?;
        
        let code = code.replace('\n', "").replace(",]", "]");
        let pattern = r"\s*(.*=.*)?(\[.*\]).*";
        let re = Regex::new(pattern).unwrap();
        // println!("parse_file_list:{:?}", code);
        if let Some(captures) = re.captures(&code) {
            let tasks_list_str = captures.get(2).unwrap().as_str();
            println!("tasks_list_str{}", tasks_list_str.trim());
            let tasks: Vec<String> = serde_json::from_str(tasks_list_str.trim())?;
            Ok(tasks)
        } else {
            Err("Pattern not found".into())
        }
    }

    
}