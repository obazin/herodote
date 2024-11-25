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
