
# Grust - A Grep in Rust

A simple grep clone implemented in rust.


## Installation

#### Enter project directory
```bash
cd /src
```
#### Compile the project in release mode
```bash
cargo build --release
```
    
## Usage/Examples

```bash
./grep_clone pattern path_to_file
```
if found
```bash
Pattern:        "pattern"
Path:           "path_to_file"
# "pattern" found at line [n]:
        "[...]pattern[...]"
```
else
```bash
Pattern:        "pattern"
Path:           "path_to_file"
# "pattern" not found in this document.
```

It automatically ensures if errors occur while executing and reports them in the stdout.

## Roadmap

- Implementing grep flags
- Adding test cases

