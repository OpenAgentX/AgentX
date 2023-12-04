
use std::{collections::HashMap, fmt};


#[derive(Debug, PartialEq)]
pub struct RawMessage {
    content: String,
    role: String,
}

// #[derive(Derivative)]
// #[derivative(Default(new="true"), Clone, Debug, PartialEq)]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub cause_by: String,
    pub instruct_content: Option<String>,
    pub send_to: Option<String>
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.cause_by.is_empty() {
            write!(f, "[{}]: {}", self.cause_by, self.content)
        } else {
            write!(f, "{}", self.content)
        }
    }
}

impl Message {

    pub fn new(content: &str, role: &str, cause_by: &str, instruct_content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: role.to_string(),
            cause_by: cause_by.to_string(),
            instruct_content: Some(instruct_content.into()),
            send_to:None
        }
    }

    pub fn form(content: &str, role: &str, cause_by: &str, instruct_content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: role.to_string(),
            cause_by: cause_by.to_string(),
            instruct_content: Some(instruct_content.into()),
            send_to:None
        }
    }

    pub fn to_dict(&self) -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("role".to_string(), self.role.clone());
        dict.insert("content".to_string(), self.content.clone());
        dict
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserMessage(Message);

#[derive(Debug, Clone, PartialEq)]
pub struct SystemMessage(Message);

#[derive(Debug, Clone, PartialEq)]
pub struct AIMessage(Message);

impl UserMessage {
    pub fn new(content: &str) -> Self {
        UserMessage(Message::form(content, "user", "", ""))
    }
}

impl SystemMessage {
    pub fn new(content: &str) -> Self {
        SystemMessage(Message::form(content, "system", "", ""))
    }
}

impl AIMessage {
    pub fn new(content: &str) -> Self {
        AIMessage(Message::form(content, "assistant", "", ""))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn message() {
        // Placeholder for logs module
        // if log_enabled!(Level::Info) {
        //     let test_content = "test_message";
        //     let msgs = vec![
        //         UserMessage::new(test_content),
        //         SystemMessage::new(test_content),
        //         AIMessage::new(test_content),
        //         Message::new(test_content, "QA", ""),
        //     ];
        //     info!("{:?}", msgs);
        // }
    }
}