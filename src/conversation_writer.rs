use crate::{model::Conversation, utils::normalized_filename_string};
use rayon::prelude::*;
use std::{fs, path::Path};

fn conversation_to_md(conversation: Conversation) -> String {
    let mut content = format!("# {}\n\n", conversation.title);

    for item in conversation.items {
        let section_title = if item.author == "user" {
            "Question"
        } else {
            "Answer"
        };
        content.push_str(&format!("## {}\n{}\n\n", section_title, item.text));
    }
    content
}

pub fn write<P>(conversations: Vec<Conversation>, output_folder: P)
where
    P: AsRef<Path>,
{
    let folder = output_folder.as_ref();
    if let Err(err) = fs::create_dir_all(folder) {
        eprintln!("Failed to create directory '{}': {}", folder.display(), err);
        return;
    }
    conversations.into_par_iter().for_each(|conversation| {
        let filename = format!(
            "{}-{}.md",
            conversation.date,
            normalized_filename_string(&conversation.title, 40)
        );
        let path = folder.join(filename);
        let content = conversation_to_md(conversation);

        if let Err(err) = fs::write(&path, content) {
            eprintln!("Failed to write file '{}': {}", path.display(), err);
        }
    });
}
