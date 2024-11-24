# Herodote
This Rust project is a command-line tool designed to process GPT-generated conversation data (in JSON format) and convert it into structured Markdown files. It is optimized for performance, modular, and easy to use, making it a great tool for archiving, publishing, or analyzing GPT conversations.

### Features
	•	JSON Parsing: Reads GPT conversation data stored in JSON format.
	•	Markdown Export: Converts conversations into clean, human-readable Markdown files.
	•	Parallel Processing: Uses multi-threading (via rayon) for efficient file writing, even with large datasets.
	•	Customizable Output: Normalizes filenames and ensures compatibility with Markdown editors.
	•	Error Handling: Handles file system and parsing errors gracefully.

### Purpose
GPT tools often generate structured JSON data containing user interactions and assistant responses. This project provides a simple way to transform that raw data into Markdown files, which can then be:
	•	Archived for later reference.
	•	Published in blogs or documentation.
	•	Analyzed for research or development purposes.

This project is well-suited for developers, researchers, or writers working with GPT-generated data who want a streamlined solution for managing and exporting conversations.

### Usage

Prerequisites
	•	Rust installed on your machine (use Rustup to install).
	•	A valid JSON file containing GPT interaction data.

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

