use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde_json::Value;
use tempfile::NamedTempFile;
use thiserror::Error;

// InputSource cannot be cloned - stdin can only be read once
#[derive(Debug)]
pub enum InputSource {
    File(PathBuf),
    Stdin,
}

// OutputDestination can be cloned - files and stdout can be written to multiple times
#[derive(Debug, Clone)]
pub enum OutputDestination {
    File(String),
    Stdout,
}

#[derive(Error, Debug)]
pub enum YamlEditorError {
    #[error("Failed to read YAML file: {0}")]
    YamlRead(#[from] serde_yaml::Error),
    #[error("Failed to create temporary file: {0}")]
    TempFile(std::io::Error),
    #[error("JSON serialization error: {0}")]
    JsonSerialization(serde_json::Error),
    #[error("JSON deserialization error: {0}")]
    JsonDeserialization(serde_json::Error),
    #[error("Editor execution failed: {0}")]
    EditorExecution(std::io::Error),
    #[error("File I/O error: {0}")]
    FileIO(std::io::Error),
    #[error("No changes detected")]
    NoChanges,
    #[error("Failed to read from stdin: {0}")]
    StdinRead(std::io::Error),
    #[error("Failed to write to stdout: {0}")]
    StdoutWrite(std::io::Error),
}

fn read_input(source: &InputSource) -> Result<String, YamlEditorError> {
    match source {
        InputSource::File(path) => fs::read_to_string(path).map_err(YamlEditorError::FileIO),
        InputSource::Stdin => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(YamlEditorError::StdinRead)?;
            Ok(buffer)
        }
    }
}

fn write_output(destination: &OutputDestination, content: &str) -> Result<(), YamlEditorError> {
    match destination {
        OutputDestination::File(path) => fs::write(path, content).map_err(YamlEditorError::FileIO),
        OutputDestination::Stdout => {
            print!("{}", content);
            io::stdout().flush().map_err(YamlEditorError::StdoutWrite)?;
            Ok(())
        }
    }
}

pub struct EditSession {
    temp_file: NamedTempFile,
    output_destination: OutputDestination,
    original_content: Value,
}

impl EditSession {
    pub fn new(
        input_source: InputSource,
        output_destination: OutputDestination,
    ) -> Result<Self, YamlEditorError> {
        // Read and parse YAML from input source
        let yaml_content = read_input(&input_source)?;
        let original_content: Value = serde_yaml::from_str(&yaml_content)?;
        // Convert YAML to pretty JSON
        let json_content = serde_json::to_string_pretty(&original_content)
            .map_err(YamlEditorError::JsonSerialization)?;

        // Create temporary file with .json extension for syntax highlighting
        let mut temp_file =
            NamedTempFile::with_suffix(".json").map_err(YamlEditorError::TempFile)?;
        temp_file
            .write_all(json_content.as_bytes())
            .map_err(YamlEditorError::TempFile)?;
        temp_file.flush().map_err(YamlEditorError::TempFile)?;

        Ok(Self {
            temp_file,
            output_destination,
            original_content,
        })
    }

    pub fn edit(&mut self) -> Result<(), YamlEditorError> {
        edit::edit_file(self.temp_file.path()).map_err(YamlEditorError::EditorExecution)
    }

    pub fn try_parse_json(&self) -> Result<Value, YamlEditorError> {
        let content =
            fs::read_to_string(self.temp_file.path()).map_err(YamlEditorError::TempFile)?;
        serde_json::from_str(&content).map_err(YamlEditorError::JsonDeserialization)
    }

    pub fn save_yaml(&self, edited_value: &Value) -> Result<(), YamlEditorError> {
        let yaml_output = serde_yaml::to_string(edited_value)?;
        write_output(&self.output_destination, &yaml_output)
    }

    pub fn has_changes(&self, edited_value: &Value) -> bool {
        edited_value != &self.original_content
    }
}

pub struct YamlEditor {
    input_source: InputSource,
    output_destination: OutputDestination,
}

fn display_json_error(err: &serde_json::Error) {
    eprintln!("There was an Error in your edited JSON document:");
    eprintln!();
    eprintln!("{}", err);
    eprintln!();
    eprintln!("Press any key to return to the editor and fix the issue...");
}

fn wait_for_keypress() -> Result<(), YamlEditorError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(YamlEditorError::FileIO)?;
    Ok(())
}

impl YamlEditor {
    pub fn new(input_source: InputSource, output_destination: OutputDestination) -> Self {
        Self {
            input_source,
            output_destination,
        }
    }

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Self {
        let path = file_path.as_ref().to_path_buf();
        let file_path_str = path.to_string_lossy().to_string();
        Self::new(
            InputSource::File(path),
            OutputDestination::File(file_path_str),
        )
    }

    pub fn from_stdin() -> Self {
        Self::new(InputSource::Stdin, OutputDestination::Stdout)
    }

    pub fn edit(self) -> Result<(), YamlEditorError> {
        let mut session = EditSession::new(self.input_source, self.output_destination)?;

        loop {
            session.edit()?;

            match session.try_parse_json() {
                Ok(edited_value) => {
                    if !session.has_changes(&edited_value) {
                        return Err(YamlEditorError::NoChanges);
                    }
                    return session.save_yaml(&edited_value);
                }
                Err(YamlEditorError::JsonDeserialization(json_err)) => {
                    display_json_error(&json_err);
                    wait_for_keypress()?;
                    // Continue loop to retry
                }
                Err(other) => return Err(other),
            }
        }
    }
}
