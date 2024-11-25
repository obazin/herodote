# Herodote
This Rust project is a command-line tool designed to process GPT-generated conversation data (in JSON format) and convert it into structured Markdown files. It is optimized for performance, modular, and easy to use, making it a reliable tool for archiving, publishing, or analyzing GPT conversations.

### Features
- JSON Parsing: Reads GPT conversation data stored in JSON format.
- Markdown Export: Converts conversations into clean, human-readable Markdown files.
- Parallel Processing: Uses multi-threading (via rayon) for efficient file writing, even with large datasets.
- Customizable Output: Normalizes filenames and ensures compatibility with Markdown editors.
- Error Handling: Handles file system and parsing errors gracefully.

### Purpose
GPT tools often generate structured JSON data containing user interactions and assistant responses. This project provides a simple way to transform that raw data into Markdown files, which can then be:
- Archived for later reference.
- Published in blogs or documentation.
- Analyzed for research or development purposes.

This project is well-suited for developers, researchers, or writers working with GPT-generated data who want a streamlined solution for managing and exporting conversations.

### Build & Run
Build the project: `cargo build`
Run the tool: `cargo run -- -i path/to/input.json -o path/to/output/`
Testing : `cargo test`

### Usage
Prerequisites
- Rust installed on your machine (use Rustup to install).
- A valid JSON file containing GPT interaction data.

Command-line Interface
This tool is designed to be used via the command line. Below are the supported options:
```sh
USAGE:
    herodote [OPTIONS]

OPTIONS:
    -i, --input <FILE>          Path to the input JSON file containing GPT conversations
    -o, --output-folder <DIR>   Path to the folder where Markdown files will be saved
    -h, --help                  Show this help message
    -V, --version               Show version information
```
### Example Usage
Convert a JSON file of GPT conversations into Markdown files:

```bash
./target/release/gpt-converter -i conversations.json -o output/
```
This will:

Parse conversations.json.
Create Markdown files in the output/ directory, one file per conversation.

### Input File Format
The tool expects a JSON file with the following structure:

```json
{
  "title": "Conversation Title",
  "update_time": 1672531200,
  "mapping": {
    "1": {
      "message": {
        "author": {
          "role": "user"
        },
        "content": {
          "parts": ["Hello!"]
        },
        "create_time": 1672531200
      }
    },
    "2": {
      "message": {
        "author": {
          "role": "assistant"
        },
        "content": {
          "parts": ["Hi! How can I assist you?"]
        },
        "create_time": 1672531220
      }
    }
  }
}
```
Each conversation is represented as a mapping of interaction nodes, where:

- title is the conversation title.
- mapping contains the individual user and assistant messages.
- update_time specifies the last update timestamp of the conversation.

### Output Example
For the above JSON, the tool generates a Markdown file like this:

File: 2023-01-01-Conversation_Title.md
```markdown
# Conversation Title

## Question
Hello!

## Answer
Hi! How can I assist you?
```
### Key Points
- Efficient Multi-threading: Uses rayon for concurrent file writing, ensuring scalability for large datasets.
- Data Validation: Ensures only valid interactions (e.g., non-empty text) are processed.
- Filename Normalization: Converts titles into safe, human-readable filenames.
- Extensibility: The modular design allows easy integration with other tools or formats (e.g., HTML export).

### Development
Directory Structure
```shell
src/
├── conversation_writer.rs  # Handles Markdown file writing
├── main.rs                 # CLI entry point
├── model.rs                # Data structures mapping GPT export and target Model
├── utils.rs                # Helper functions for filenames and dates
```


### Contributing
Contributions are welcome! If you find a bug, want to suggest a feature, or improve documentation, feel free to open an issue or pull request.

### To-Do List
- Add support for HTML export.
- Implement logging instead of eprintln!.

### License
This project is licensed under the MIT License. See the LICENSE file for details.

### Acknowledgments
- [Rayon](https://github.com/rayon-rs/rayon) for parallel processing.
- [Serde](https://github.com/serde-rs/serde) for JSON parsing.
- Rust community for its excellent libraries and tools.
