# yaw - **YA**ML **w**ithout YAML Editor

A lightweight command-line utility that transforms YAML editing by converting files to JSON format for easier editing, then converting them back to YAML. Key ordering is preserved throughout the process.

## Why

YAML's strict indentation requirements can be frustrating, particularly when working with complex configurations. This tool offers a simple solution: edit your YAML content as JSON in your preferred editor, eliminating indentation concerns while maintaining the final YAML structure.

## Usage

```bash
yaw config.yaml
```

This will:
1. Convert your YAML file to JSON format
2. Open it in your configured editor (`$EDITOR`)
3. Convert your edited JSON back to YAML preserving key order
4. Save the result to the original file

### Example

Given a YAML file like:
```yaml
database:
  host: localhost
  port: 5432
```

Running `yaw database.yaml` opens this JSON in your editor:
```json
{
  "database": {
    "host": "localhost",
    "port": 5432
  }
}
```

After editing and saving, it's converted back to YAML with preserved ordering.

## Build

```bash
cargo build --release
```

The binary will be available at `target/release/yaw`.

