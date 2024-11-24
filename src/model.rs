use crate::utils::date_from_epoch_time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Conversation {
    pub title: String,
    pub items: Vec<ConversationItem>,
    pub date: String,
}

impl Conversation {
    pub fn new(title: String, items: Vec<ConversationItem>, date: String) -> Conversation {
        Conversation { title, items, date }
    }

    pub fn from(gpt_interaction: GPTInteraction) -> Conversation {
        let mut conversation_items: Vec<ConversationItem> = Vec::new();

        for object in gpt_interaction.mapping {
            if let Some(item) = Self::process_interaction_node(object.1) {
                conversation_items.push(item);
            }
        }
        conversation_items.sort_by(|item1, item2| item1.time.total_cmp(&item2.time));
        Conversation::new(
            gpt_interaction.title,
            conversation_items,
            date_from_epoch_time(gpt_interaction.update_time),
        )
    }

    fn process_interaction_node(node: Node) -> Option<ConversationItem> {
        let message = node.message?;
        let role = message.author.role;
        if role != "assistant" && role != "user" {
            return None;
        }
        let content_parts = message.content.parts?;
        let text = content_parts
            .iter()
            .filter_map(|part| match part {
                Part::String(s) => Some(s.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if text.trim().is_empty() {
            return None;
        }
        Some(ConversationItem::new(
            text,
            role,
            message.create_time.unwrap_or(0.0),
        ))
    }
}

pub struct ConversationItem {
    pub text: String,
    pub author: String,
    pub time: f64,
}

impl ConversationItem {
    pub fn new(text: String, author: String, time: f64) -> ConversationItem {
        ConversationItem { text, author, time }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GPTInteraction {
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub mapping: HashMap<String, Node>,
}

impl GPTInteraction {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub message: Option<Message>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: String,
    pub author: Author,
    pub create_time: Option<f64>,
    pub update_time: Option<f64>,
    pub content: Content,
    pub status: String,
    pub end_turn: Option<bool>,
    pub weight: f64,
    pub metadata: MessageMetadata,
    pub recipient: String,
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub role: String,
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub content_type: String,
    pub parts: Option<Vec<Part>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Part {
    String(String),
    Object(serde_json::Value),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageMetadata {
    #[serde(flatten)]
    pub additional_metadata: HashMap<String, serde_json::Value>,
}
