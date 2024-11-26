use crate::{model::Conversation, utils::normalized_filename_string};
use rayon::prelude::*;
use std::{fs, path::Path};

/// Converts a `Conversation` object into a Markdown formatted string.
///
/// This function takes a `Conversation` struct, iterates over its items, and constructs
/// a Markdown representation of the conversation. Each item in the conversation is prefixed
/// with a section title based on the author of the message, distinguishing between user
/// input and system responses.
///
/// # Arguments
///
/// * `conversation` - A `Conversation` object that contains a title and a collection of
///   conversation items, each with an author and text content.
///
/// # Returns
///
/// A `String` containing the entire conversation formatted as Markdown. The output begins
/// with an H1 title derived from the conversation's title, followed by each item formatted
/// as an H2 section. Items authored by "user" are labeled "Question" and other items are
/// labeled "Answer".
///
/// # Example
///
/// ```
/// let conversation = Conversation {
///     title: String::from("Sample Conversation"),
///     items: vec![
///         ConversationItem { author: String::from("user"), text: String::from("What is the weather today?") },
///         ConversationItem { author: String::from("assistant"), text: String::from("The weather is sunny today.") },
///     ],
/// };
/// let markdown = conversation_to_md(conversation);
/// println!("{}", markdown);
/// // Output:
/// // # Sample Conversation
/// //
/// // ## Question
/// // What is the weather today?
/// //
/// // ## Answer
/// // The weather is sunny today.
/// //
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

/// Writes a collection of `Conversation` objects to markdown files in a specified output folder.
///
/// This function processes each `Conversation` object in the provided vector, converts it to a
/// Markdown string using the `conversation_to_md` function, and saves it as a file in the specified
/// output directory. Each file is named using a combination of the conversation's date and a
/// normalized version of the title. If the output directory does not exist, it attempts to create it.
///
/// # Arguments
///
/// * `conversations` - A `Vec<Conversation>` containing the conversations to be written to files. Each
///   `Conversation` includes a title and date that contribute to the naming of the output files.
///
/// * `output_folder` - A path that specifies the directory where the markdown files will be saved. The
///   path is generic and can be converted into a `Path`.
///
/// # Errors
///
/// Errors during directory creation or file writing are logged to the standard error output.
/// This includes failures such as inability to create the directory or to write a file, along
/// with associated error messages.
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
