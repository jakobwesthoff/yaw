mod editor;

use std::env;
use std::process;

use editor::{YamlEditor, YamlEditorError};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <yaml-file>", args[0]);
        process::exit(1);
    }

    let yaml_file = &args[1];

    match YamlEditor::new(yaml_file) {
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
