mod editor;

use std::path::PathBuf;
use std::process;

use clap::Parser;
use editor::YamlEditor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The YAML file to edit (reads from stdin if not provided)
    #[arg(value_name = "FILE")]
    yaml_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let is_file_mode = cli.yaml_file.is_some();
    let editor = match cli.yaml_file {
        Some(file_path) => YamlEditor::from_file(file_path),
        None => YamlEditor::from_stdin(),
    };

    match editor.edit() {
        Ok(()) => {
            if is_file_mode {
                println!("YAML file updated successfully");
            }
        }
        Err(editor::YamlEditorError::NoChanges) => {
            if is_file_mode {
                println!("No changes made");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
