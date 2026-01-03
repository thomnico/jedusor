//! Context management module
//!
//! Combines user input, conversation history, and document content for LLM

/// Conversation message
#[derive(Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Assistant,
}

/// Manages conversation context
pub struct ContextManager {
    history: Vec<Message>,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn add_user_message(&mut self, content: String) {
        self.history.push(Message {
            role: MessageRole::User,
            content,
        });
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.history.push(Message {
            role: MessageRole::Assistant,
            content,
        });
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}

// TODO: Add manager.rs for LLM context building
// TODO: Add document context integration
