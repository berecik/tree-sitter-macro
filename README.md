# tree-sitter-c-proc

Proc macros for compile-time access to tree-sitter-c node kinds, keywords, and fields.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tree-sitter-c = "0.20.7"
tree-sitter-c-proc = "0.1.0"
```

## Usage

```rust
use tree_sitter_c_proc::{kind, kw, field};

fn test_fn(cursor: &TreeCursor) {
    if cursor.field() == field!("sub_expr") {
        let node = cursor.node();
        match node.kind_id() {
            kind!("array_expr") => {},
            kind!("binary_expr") => {},
            kind!("unary_expr") => {},
            // etc
        }
    }
}
```

## Macros

### `kind!`

The `kind!` macro returns the node kind ID for a given node kind name. This is useful for matching against node kinds in pattern matching contexts.

```rust
let kind_id = kind!("binary_expr");
```

### `kw!`

The `kw!` macro returns the node kind ID for a given keyword. This is similar to `kind!` but specifically for keywords.

```rust
let keyword_id = kw!("if");
```

### `field!`

The `field!` macro returns the field ID for a given field name. This is useful for checking if a node has a specific field.

```rust
let field_id = field!("sub_expr");
```

## Examples

The repository includes examples that demonstrate how to use tree-sitter-c-proc macros for parsing and analyzing C code:

### Basic Example: Parse C Code

This example demonstrates how to use tree-sitter to parse C code and output the parse tree to stdout:

```bash
# Run the example with the default C code
cargo run --example parse_c --features proc_macros

# Parse a C file and output the parse tree
cat your_file.c | cargo run --example parse_c --features proc_macros
```

The basic example also demonstrates:
- Finding function definitions using the `kind!` macro
- Finding if statements using the `kw!` macro
- Finding return statements and their values using the `field!` macro

### Advanced Example: Analyze C Code

This example demonstrates more advanced usage of the macros for analyzing C code:

```bash
# Run the advanced example with the default C code
cargo run --example advanced_usage --features proc_macros

# Parse and analyze a C file
cat your_file.c | cargo run --example advanced_usage --features proc_macros
```

The advanced example demonstrates:
- Finding struct definitions and their fields
- Analyzing function calls and their arguments
- Finding variable declarations and their types
- Using const values from modules instead of macros

These examples show how the tree-sitter-c-proc macros can be used to efficiently navigate and analyze C code syntax trees at compile time.

## Development

### Building and Testing

This project uses a Makefile to simplify common development tasks:

```bash
# Build the project
make build

# Run tests and examples
make test

# Check code quality (rustfmt and clippy)
make check

# Fix code quality issues
make fix

# Run the example
make example

# Clean build artifacts
make clean

# Show help
make help
```

### Testing

The project includes tests for all three macros (`kind!`, `kw!`, and `field!`). These tests verify that the macros correctly return the expected IDs for valid inputs. You can run the tests with:

```bash
cargo test
```

Note that since these are proc macros that run at compile time, the tests can only verify behavior with valid inputs. Invalid inputs would cause compile errors, which can't be caught in runtime tests.

The tests include:
- Testing each macro with multiple valid inputs
- Verifying that different inputs produce different IDs
- Testing the macros together in a realistic scenario

If you add new macros or modify existing ones, make sure to update the tests accordingly.

### Scripts

The `scripts` directory contains shell scripts for various development tasks:

- `check_code_quality.sh`: Checks code formatting and runs clippy
- `fix_code_quality.sh`: Formats code and fixes clippy lints where possible
- `run_tests.sh`: Runs tests and examples

## Project Structure

This crate is part of the tree-sitter-c project and follows the recommended structure for tree-sitter language proc macro crates:

```
tree-sitter-c/
├── ...
├── bindings/
│   └── rust/
│       ├── ...
│       └── macros/
│           ├── Cargo.toml
│           └── src/
│               └── lib.rs
└── ...
```

By placing the proc macro crate in a `macros` subdirectory of the language's Rust bindings, we maintain a clean separation between the main language binding and the proc macros, while keeping them logically grouped together.

## Template for Other Languages

This repository includes a template for creating similar proc macro crates for other tree-sitter languages. The template is located in the `template` directory and includes:

- `README.md`: Instructions for customizing the template
- `Cargo.toml`: Template Cargo.toml file with placeholders
- `src/lib.rs`: Template implementation with placeholders

To use the template for your own tree-sitter language:

1. Copy the `template` directory to your tree-sitter language project
2. Rename the directory to `macros` and place it in the `bindings/rust` directory of your language project
3. Replace all occurrences of `LANG` in the template files with your language name
4. Update the version number in `Cargo.toml` to match your language's version
5. Add the proc macro crate as a dependency in your language's Rust binding crate
