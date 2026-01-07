# lat

cat for LLMs.

`lat` displays file contents in an LLM-friendly way. It dispatches to specialized handlers based on file patterns, configured via `.lat.kdl`. Use it in `AGENTS.md` or similar to help AI agents read files more effectively.

## Installation

```bash
cargo install lat
```

## Usage

```bash
lat <FILE> [OPTIONS]
```

### Options

- `-u, --upto <UPTO>` - Maximum characters to read
- `-f, --focus <FOCUS>` - Paths within the file to focus on, comma-separated

### Examples

```bash
lat src/main.rs
lat config.json -u 1000
lat data.json -f users,posts
lat big.json -u 5000 -f data.items,data.metadata
```

## Configuration

`lat` looks for `.lat.kdl` in the current directory and parent directories.

```kdl
rule "*.json" {
    command "json-lat"
    args "$FILE" "$FOCUS"
    defaults upto=5000
}

rule "*.js" "*.ts" "*.jsx" "*.tsx" {
    command "js-lat"
    args "$FILE"
    defaults upto=10000
}

rule "*.md" {
    command "cat"
    args "$FILE"
    defaults upto=3000
}

rule "*" {
    command "cat"
    args "$FILE"
    defaults upto=2000
}
```

### Rule structure

- **patterns** - Glob patterns as arguments to `rule` (e.g., `"*.json"`, `"*.js" "*.ts"`)
- **command** - The program to execute
- **args** - Arguments passed to the command, with variable substitution:
  - `$FILE` - The file path
  - `$UPTO` - The upto limit (omitted if not set)
  - `$FOCUS` - Comma-separated focus paths (omitted if not set)
- **defaults** - Default values when not specified via CLI:
  - `upto` - Default read limit

Rules are matched in order; the first matching pattern wins.

## Why lat?

LLMs work better when file contents are:
- **Appropriately sized** - Not too much context at once
- **Well-formatted** - Proper indentation aids comprehension
- **Focused** - Relevant sections highlighted, large arrays/objects collapsed

Instead of building all this into one tool, `lat` delegates to specialized handlers per file type, letting you customize the behavior for your project.
