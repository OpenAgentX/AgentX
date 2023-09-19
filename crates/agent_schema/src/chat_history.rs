use crate::message::Message;

// 定义聊天历史结构体
#[derive(Debug, Default)]
pub struct ChatHistory {
    messages: Vec<Message>,
}

// 实现聊天历史结构体的方法
impl ChatHistory {
    // 添加消息
    pub fn add_message(&mut self, messages: Vec<Message>) {
        for message in messages {
            self.messages.push(message);
        }
    }

    // 将消息历史转换为字符串
    pub fn to_string(&self, add_sender_prefix: bool) -> String {
        if add_sender_prefix {
            self.messages
                .iter()
                .map(|message| message.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            self.messages
                .iter()
                .map(|message| message.content.clone())
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    // 重置消息历史
    pub fn reset(&mut self) {
        self.messages.clear();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_add_message() {
        let mut chat_history = ChatHistory {
            messages: Vec::new(),
        };

        let message1 = Message {
            cause_by: "User".to_string(),
            content: "Hello, how are you?".to_string(),
            ..Default::default()
        };

        let message2 = Message {
            cause_by: "Assistant".to_string(),
            content: "I'm doing well, thank you!".to_string(),
            ..Default::default()
        };

        chat_history.add_message(vec![message1.clone(), message2.clone()]);

        assert_eq!(chat_history.messages, vec![message1, message2]);
    }

    #[test]
    fn test_to_string() {
        let mut chat_history = ChatHistory {
            messages: Vec::new(),
        };

        let message1 = Message {
            cause_by: "User".to_string(),
            content: "Hello, how are you?".to_string(),
            ..Default::default()
        };

        let message2 = Message {
            cause_by: "Assistant".to_string(),
            content: "I'm doing well, thank you!".to_string(),
            ..Default::default()
        };

        chat_history.add_message(vec![message1.clone(), message2.clone()]);

        let expected_string = format!(
            "[{}]: {}\n[{}]: {}",
            message1.cause_by, message1.content, message2.cause_by, message2.content
        );

        assert_eq!(chat_history.to_string(true), expected_string);
    }

    #[test]
    fn test_reset() {
        let mut chat_history = ChatHistory {
            messages: Vec::new(),
        };

        let message1 = Message {
            cause_by: "User".to_string(),
            content: "Hello, how are you?".to_string(),
            ..Default::default()
        };

        chat_history.add_message(vec![message1.clone()]);
        chat_history.reset();

        assert_eq!(chat_history.messages, Vec::<Message>::new());
    }
}
