mod editor;

use std::path::PathBuf;
use std::process;

use clap::Parser;
use editor::{YamlEditor, YamlEditorError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The YAML file to edit
    #[arg(value_name = "FILE")]
    yaml_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match YamlEditor::new(&cli.yaml_file) {
        Ok(editor) => match editor.edit() {
            Ok(()) => println!("YAML file updated successfully"),
            Err(YamlEditorError::NoChanges) => println!("No changes made"),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to open YAML file: {}", e);
            process::exit(1);
        }
    }
}
