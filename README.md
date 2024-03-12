
# Grust - A Grep in Rust

A simple grep clone implemented in rust.


## Installation

#### Install Rustc/Cargo
##### Linux or Wsl
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
##### Other OS
Visit [Rust Official Website](https://forge.rust-lang.org/infra/other-installation-methods.html)

#### Enter project directory
```bash
cd /src
```
#### Compile the project in release mode
```bash
cargo build --release
```
The executable will be found in:
```bash
target/release
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

For searching multiple patterns use '\|' to separate them inside the same argument passed to the function, as in grep:
```bash
./grep_clone "pattern1\|pattern2\|..." path_to_file
```
The output it's slightly different, more grep-y, since the single pattern output could result confusing for multiple outputs:
```bash
Pattern:        "pattern"
Path:           "path_to_file"

        "[...]pattern1[...]pattern2[...]"
```


It automatically ensures if errors occur while executing and reports them in the stdout.

## Roadmap

- Make it work for one pattern ✅
- Add colouring of the pattern searched inside the line printed ✅
- Multiple patterns search ✅
- Implementing grep flags ❌
- Adding test cases ❌
