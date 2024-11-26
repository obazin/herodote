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

/// Represents an interaction with a Generative Pre-trained Transformer (GPT) model.
///
/// This struct is used to store details about a specific interaction, including its
/// title, creation and update timestamps, and a mapping of conversation nodes.
/// Each node can represent an individual message or part of the conversation hierarchy.
///
/// # Fields
///
/// * `title` - A `String` representing the title of the interaction, which can be used to identify or
///   describe the conversation's context or subject.
///
/// * `create_time` - A `f64` value representing the timestamp when the interaction was created.
///   This is typically expressed in seconds since the Unix epoch.
///
/// * `update_time` - A `f64` value representing the last time the interaction was updated,
///   also expressed in seconds since the Unix epoch.
///
/// * `mapping` - A `HashMap<String, Node>` that contains the nodes of the conversation. Each
///   key is a unique identifier for a node, and the corresponding value is a `Node` struct
///   that encapsulates the specifics of that part of the conversation, including its message,
///   parent, and children relationships.
///
/// # Related Structures
///
/// * `Node` - Represents a single node in the conversation, which may hold a message, have a parent
///   node, and contain child nodes, allowing for a hierarchical structuring of messages.
///
/// * `Message` - Details the specifics of a message associated with a node, including its author,
///   content, and various metadata fields.
///
/// * `Author`, `Content`, `Part`, and `MessageMetadata` - Supporting structs used to further define
///   the specifics of each message, such as content parts, author information, and additional metadata.
#[derive(Debug, Deserialize, Serialize)]
pub struct GPTInteraction {
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub mapping: HashMap<String, Node>,
}

impl GPTInteraction {}

/// The `Node` struct represents a single node within the `GPTInteraction` mapping.
/// It potentially contains a message and relationships to parent and child nodes.
///
/// A `Node` can be thought of as an element in a tree-like data structure where
/// each node may have zero or one parent and zero or more children.
/// This structure supports the construction of complex conversation trees
/// or any hierarchical data representation where nodes have unique identifiers and
/// can optionally hold content (in the form of `Message`).
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    /// A unique identifier for the node.
    pub id: String,
    /// An optional `Message` associated with the node.
    ///
    /// This field is optional, so some nodes may not contain a message;
    /// they may exist only to support a hierarchical structure or represent
    /// intermediate steps in interactions.
    pub message: Option<Message>,
    /// An option representing the identifier of the parent node.
    ///
    /// This can be `None` if the node is a root node, i.e., it does not have a parent.
    pub parent: Option<String>,
    /// A vector containing the identifiers of child nodes.
    ///
    /// Children are nodes that directly descend from this node, allowing the creation
    /// of branch structures in the data hierarchy.
    pub children: Vec<String>,
}

/// Represents a message within a conversation node in a `GPTInteraction`.
///
/// The `Message` struct encapsulates all necessary details about a single message,
/// including its identifier, author, content, and relevant metadata. It is part of
/// the hierarchical structure of nodes in a conversation, each of which may contain
/// one or more messages.
///
/// The `Message` struct is designed to capture a variety of metadata attributes that
/// enhance the contextual understanding of a message within a conversation, such as
/// its creation and update times, status, and the recipient. Additionally, this struct
/// can represent the turn-taking nature of dialog through the `end_turn` field.
///
/// # Fields
///
/// * `id` - A `String` that uniquely identifies the message.
///
/// * `author` - An `Author` struct detailing the metadata of the individual
///   who authored the message, including their role and optional name.
///
/// * `create_time` - An optional `f64` representing the timestamp when the
///   message was created, usually given in seconds since the Unix epoch.
///
/// * `update_time` - An optional `f64` representing when the message was last
///   updated, also expressed in seconds since the Unix epoch. This field is useful
///   for tracking modifications to the message over time.
///
/// * `content` - A `Content` struct that houses the main content of the message,
///   including the content type and its parts.
///
/// * `status` - A `String` indicating the current status of the message, which
///   might reflect its position within a workflow or lifecycle in the conversation.
///
/// * `end_turn` - An optional `bool` that specifies if the message concludes a
///   turn in the conversation, allowing systems to manage dialogue flow effectively.
///
/// * `weight` - A `f64` expressing the significance or precedence of the message,
///   potentially used in the management or analysis of conversation priority levels.
///
/// * `metadata` - A `MessageMetadata` struct that encapsulates additional metadata,
///   providing flexibility for extending message information with dynamic attributes.
///
/// * `recipient` - A `String` that identifies the recipient of the message,
///   which can help in routing or targeting specific parts of a system or user base.
///
/// * `channel` - An optional `String` signifying the communication channel through
///   which the message was sent or received, allowing differentiation between platforms
///   or mediums in multi-channel scenarios.
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

/// Represents the author of a message within a `GPTInteraction`.
///
/// # Fields
///
/// * `role` - A `String` describing the role of the author, such as "user", "system",
///   or any custom role designation that identifies the author's function or position
///   within the interaction.
///
/// * `name` - An optional `String` representing the name of the author. This field
///   allows for a more personalized label if available, but it may be omitted if not
///   applicable or required.
///
/// * `metadata` - A `HashMap<String, String>` providing additional metadata about the
///   author. This can include any key-value pairs that extend the basic information of
///   the author, allowing for flexibility in storing metadata such as identifiers,
///   permissions, or other relevant attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub role: String,
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Represents the main content of a message within a `GPTInteraction`.
///
/// The `Content` struct is designed to encapsulate the core information of a message,
/// including the type of content being conveyed and its segmented parts.
///
/// # Fields
///
/// * `content_type` - A `String` specifying the type of content, such as "text" or "image",
///   which helps understand the format or nature of the message content.
///
/// * `parts` - An optional `Vec<Part>` representing the different parts of the content.
///   Each part may encapsulate a string or a serialized object
#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub content_type: String,
    pub parts: Option<Vec<Part>>,
}

/// Represents a part of the content within a message.
///
/// The `Part` enum is utilized to define versatile segments of message content that
/// can either be a simple string or a more complex serialized object.
///
/// # Variants
///
/// * `String` - A simple string representing a text-based part of the content.
///
/// * `Object` - A `serde_json::Value` that represents a JSON object, allowing for
///   structured and complex data forms to be part of the message content.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Part {
    String(String),
    Object(serde_json::Value),
}

/// Encapsulates additional metadata for a message within a `GPTInteraction`.
///
/// The `MessageMetadata` struct provides a flexible container for dynamic attributes
/// associated with a message. This flexibility is crucial for storing a wide range of
/// metadata attributes that can evolve over time or vary across different messages.
///
/// # Fields
///
/// * `additional_metadata` - A `HashMap<String, serde_json::Value>` that contains
///   key-value pairs of additional metadata, where each value can be a complex JSON
///   structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageMetadata {
    #[serde(flatten)]
    pub additional_metadata: HashMap<String, serde_json::Value>,
}
