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
