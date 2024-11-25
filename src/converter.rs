use crate::{
    model::{Conversation, ConversationItem, GPTInteraction, Node, Part},
    utils::date_from_epoch_time,
};

pub fn create_conversation_from(gpt_interaction: GPTInteraction) -> Conversation {
    let mut conversation_items: Vec<ConversationItem> = Vec::new();

    for object in gpt_interaction.mapping {
        if let Some(item) = process_interaction_node(object.1) {
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

#[cfg(test)]
mod tests {
    use crate::{
        converter::create_conversation_from,
        model::{Author, Content, GPTInteraction, Message, MessageMetadata, Node, Part},
    };
    use std::collections::HashMap;

    #[test]
    fn test_create_conversation_from() {
        let interaction = GPTInteraction {
            title: "Test Conversation".to_string(),
            create_time: 0.0,
            update_time: 1672531200.0,
            mapping: HashMap::from([
                (
                    "1".to_string(),
                    Node {
                        id: "1".to_string(),
                        message: Some(Message {
                            id: "1".to_string(),
                            author: Author {
                                role: "user".to_string(),
                                name: None,
                                metadata: HashMap::new(),
                            },
                            create_time: Some(1672531200.0),
                            update_time: None,
                            content: Content {
                                content_type: "text".to_string(),
                                parts: Some(vec![Part::String("Hello!".to_string())]),
                            },
                            status: "complete".to_string(),
                            end_turn: None,
                            weight: 1.0,
                            metadata: MessageMetadata {
                                additional_metadata: HashMap::new(),
                            },
                            recipient: "assistant".to_string(),
                            channel: None,
                        }),
                        parent: None,
                        children: vec![],
                    },
                ),
                (
                    "2".to_string(),
                    Node {
                        id: "2".to_string(),
                        message: Some(Message {
                            id: "2".to_string(),
                            author: Author {
                                role: "assistant".to_string(),
                                name: None,
                                metadata: HashMap::new(),
                            },
                            create_time: Some(1672531210.0),
                            update_time: None,
                            content: Content {
                                content_type: "text".to_string(),
                                parts: Some(vec![Part::String("Hi!".to_string())]),
                            },
                            status: "complete".to_string(),
                            end_turn: None,
                            weight: 1.0,
                            metadata: MessageMetadata {
                                additional_metadata: HashMap::new(),
                            },
                            recipient: "user".to_string(),
                            channel: None,
                        }),
                        parent: Some("1".to_string()),
                        children: vec![],
                    },
                ),
            ]),
        };

        let conversation = create_conversation_from(interaction);

        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.date, "2023-01-01");
        assert_eq!(conversation.items.len(), 2);
        assert_eq!(conversation.items[0].text, "Hello!");
        assert_eq!(conversation.items[0].author, "user");
        assert_eq!(conversation.items[1].text, "Hi!");
        assert_eq!(conversation.items[1].author, "assistant");
    }
}
