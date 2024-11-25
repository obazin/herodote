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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Conversation, ConversationItem};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_conversation_to_md() {
        let conversation = Conversation {
            title: "Test Conversation".to_string(),
            date: "2023-01-01".to_string(),
            items: vec![
                ConversationItem {
                    text: "Hello!".to_string(),
                    author: "user".to_string(),
                    time: 1672531200.0,
                },
                ConversationItem {
                    text: "Hi!".to_string(),
                    author: "assistant".to_string(),
                    time: 1672531210.0,
                },
            ],
        };

        let markdown = conversation_to_md(conversation);
        let expected = r#"# Test Conversation

## Question
Hello!

## Answer
Hi!

"#;
        assert_eq!(markdown, expected);
    }

    #[test]
    fn test_write() {
        let conversations = vec![Conversation {
            title: "Test Conversation".to_string(),
            date: "2023-01-01".to_string(),
            items: vec![ConversationItem {
                text: "Hello!".to_string(),
                author: "user".to_string(),
                time: 1672531200.0,
            }],
        }];

        let output_folder = PathBuf::from("./test_output");
        write(conversations, &output_folder);

        let output_path = output_folder.join("2023-01-01-Test_Conversation.md");
        assert!(output_path.exists());

        let content = fs::read_to_string(output_path.clone()).unwrap();
        assert!(content.contains("# Test Conversation"));

        // Clean up
        fs::remove_file(output_path).unwrap();
        fs::remove_dir(output_folder).unwrap();
    }
}
