
use std::collections::{HashMap, HashSet};

use agent_schema::Message;

// struct Message {
//     // Define the fields of the Message struct
//     // You need to define the fields based on the actual Message struct in your implementation
// }
// TODO 需要优化储存方式
#[derive(Debug)]
pub struct Memory {
    pub storage: Vec<Message>,
    index: HashMap<String, Vec<Message>>, 
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            storage: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn add(&mut self, message: Message) {
        // println!("memory add message{:?}", message);
        if self.storage.contains(&message) {
            return;
        }
        self.storage.push(message.clone());
        // self.index.insert(message.cause_by, message.clone());
        // self.index.get(message.cause_by);
        // if let Some(cause_by) =  {
        self.index
                .entry(message.cause_by.clone())
                .or_insert_with(Vec::new)
                .push(message);
    }

    pub fn add_batch(&mut self, messages: Vec<Message>) {
        for message in messages {
            self.add(message);
        }
    }

    pub fn get_by_role(&self, role: &str) -> Vec<&Message> {
        self.storage
            .iter()
            .filter(|message| message.role == role)
            .collect()
    }

    pub fn get_by_content(&self, content: &str) -> Vec<&Message> {
        self.storage
            .iter()
            .filter(|message| message.content.contains(content))
            .collect()
    }

    pub fn delete(&mut self, message: &Message) {
        if let Some(index) = self.storage.iter().position(|m| m == message) {
            self.storage.remove(index);
        }
        // if let Some(cause_by) = &message.cause_by {
        if let Some(index) = self.index.get_mut(&message.cause_by).and_then(|v| v.iter().position(|m| m == message)) {
            self.index.get_mut(&message.cause_by).unwrap().remove(index);
        }
        // }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
        self.index.clear();
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }

    pub fn try_remember(&self, keyword: &str) -> Vec<&Message> {
        self.storage
            .iter()
            .filter(|message| message.content.contains(keyword))
            .collect()
    }

    pub fn get(&self, k: usize) -> Vec<&Message> {
        if k == 0 {
            self.storage.iter().collect()
        } else {
            self.storage.iter().rev().take(k).collect()
        }
    }

    pub fn has_message(&self, message: &Message, k: usize) -> bool {
        let messages = self.get(k);
        messages.contains(&message)
    }

    // pub fn remember(&self, observed: Vec<&Message>, k: usize) -> Vec<&Message> {
    //     let already_observed = self.get(k);
    //     observed
    //         .into_iter()
    //         .filter(|i| !already_observed.contains(i))
    //         .collect()
    // }

    // pub fn get_by_action(&self, action: &str) -> Vec<&Message> {
    //     self.index.get(action).unwrap_or(&Vec::new()).clone()
    // }

    pub fn get_by_actions(&self, actions: HashSet<String>) -> Vec<&Message> {
        let mut result = Vec::new();
        for action in actions {
            if let Some(messages) = self.index.get(&action) {
                result.extend(messages);
            }
        }
        result
    }
}


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
