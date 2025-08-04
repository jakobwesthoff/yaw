use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde_json::Value;
use tempfile::NamedTempFile;
use thiserror::Error;

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
}

pub struct EditSession {
    temp_file: NamedTempFile,
    file_path: String,
}

impl EditSession {
    pub fn new(file_path: String, content: &Value) -> Result<Self, YamlEditorError> {
        // Convert YAML to pretty JSON
        let json_content =
            serde_json::to_string_pretty(content).map_err(YamlEditorError::JsonSerialization)?;

        // Create temporary file with .json extension for syntax highlighting
        let mut temp_file =
            NamedTempFile::with_suffix(".json").map_err(YamlEditorError::TempFile)?;
        temp_file
            .write_all(json_content.as_bytes())
            .map_err(YamlEditorError::TempFile)?;
        temp_file.flush().map_err(YamlEditorError::TempFile)?;

        Ok(Self {
            temp_file,
            file_path,
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
        fs::write(&self.file_path, yaml_output).map_err(YamlEditorError::FileIO)
    }
}

pub struct YamlEditor {
    file_path: String,
    original_content: Value,
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
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, YamlEditorError> {
        let file_path = file_path.as_ref().to_string_lossy().to_string();
        let yaml_content = fs::read_to_string(&file_path).map_err(YamlEditorError::FileIO)?;
        let original_content: Value = serde_yaml::from_str(&yaml_content)?;

        Ok(Self {
            file_path,
            original_content,
        })
    }

    pub fn edit(&self) -> Result<(), YamlEditorError> {
        let mut session = EditSession::new(self.file_path.clone(), &self.original_content)?;

        loop {
            session.edit()?;

            match session.try_parse_json() {
                Ok(edited_value) => {
                    if edited_value == self.original_content {
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
