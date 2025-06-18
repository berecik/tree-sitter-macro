# Template for Tree-Sitter Language Proc Macros

This directory contains template files for creating proc macro crates for tree-sitter languages. These proc macros allow for compile-time access to node kinds, keywords, and fields, which can be used in pattern matching contexts.

## Usage

1. Copy this template directory to your tree-sitter language project
2. Rename the directory to `macros` and place it in the `bindings/rust` directory of your language project
3. Update the `Cargo.toml` file with your language-specific information
4. Update the `src/lib.rs` file with your language-specific information
5. Add the proc macro crate as a dependency in your language's Rust binding crate

## Customizing the Template

### Cargo.toml

Update the following fields in `Cargo.toml`:

- `name`: Change to `tree-sitter-<your-language>-proc`
- `description`: Update with your language name
- `repository`: Update with your language's repository URL
- `dependencies.tree-sitter-<your-language>`: Update with your language's crate name and version

### src/lib.rs

Update the following in `src/lib.rs`:

- Replace `tree_sitter_c::language()` with `tree_sitter_<your_language>::language()`
- Update the error messages with your language name

## Project Structure

The recommended structure for tree-sitter language proc macro crates is:

```
tree-sitter-<your-language>/
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

## Publishing

Once you've customized the template for your language, you can publish the proc macro crate to crates.io:

```bash
cd tree-sitter-<your-language>/bindings/rust/macros
cargo publish
```

Then users can add it as a dependency in their `Cargo.toml`:

```toml
[dependencies]
tree-sitter-<your-language> = "<version>"
tree-sitter-<your-language>-proc = "<version>"
```

And use the proc macros in their code:

```rust
use tree_sitter_<your_language>_proc::{kind, kw, field};

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