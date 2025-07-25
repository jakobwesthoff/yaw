use std::fs;
use std::io::Write;
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
    JsonSerialization(#[from] serde_json::Error),
    #[error("Editor execution failed: {0}")]
    EditorExecution(std::io::Error),
    #[error("File I/O error: {0}")]
    FileIO(std::io::Error),
    #[error("No changes detected")]
    NoChanges,
}

pub struct YamlEditor {
    file_path: String,
    original_content: Value,
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
        // Convert YAML to pretty JSON
        let json_content = serde_json::to_string_pretty(&self.original_content)?;

        // Create temporary file with .json extension for syntax highlighting
        let mut temp_file =
            NamedTempFile::with_suffix(".json").map_err(YamlEditorError::TempFile)?;
        temp_file
            .write_all(json_content.as_bytes())
            .map_err(YamlEditorError::TempFile)?;
        temp_file.flush().map_err(YamlEditorError::TempFile)?;

        // Open the temporary file in editor
        edit::edit_file(temp_file.path()).map_err(YamlEditorError::EditorExecution)?;

        // Read the edited content back from the filesystem
        let edited_content =
            std::fs::read_to_string(temp_file.path()).map_err(YamlEditorError::TempFile)?;

        // Parse the edited JSON content
        let edited_value: Value = serde_json::from_str(&edited_content)?;

        // Check if anything changed
        if edited_value == self.original_content {
            return Err(YamlEditorError::NoChanges);
        }

        // Convert back to YAML and write to original file
        let yaml_output = serde_yaml::to_string(&edited_value)?;
        fs::write(&self.file_path, yaml_output).map_err(YamlEditorError::FileIO)?;

        Ok(())
    }
}
