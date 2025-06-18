use std::io::{self, Read};
use tree_sitter::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the parser
    let mut parser = Parser::new();

    // Set the language to C
    let language = tree_sitter_c::language();
    parser.set_language(language)?;

    // Read input from stdin or use a default example
    let code = if atty::is(atty::Stream::Stdin) {
        // No stdin input, use a default example
        r#"
        #include <stdio.h>

        int main() {
            printf("Hello, world!\n");
            return 0;
        }
        "#
        .to_string()
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Parse the code
    let tree = parser.parse(&code, None).unwrap();
    let root_node = tree.root_node();

    // Print the syntax tree
    println!("Syntax tree:");
    print_node(&root_node, &code, 0);

    // Demonstrate using the proc macros
    #[cfg(feature = "proc_macros")]
    {
        use tree_sitter_c_proc::{field, kind, kw};

        println!("\nDemonstrating proc macros:");

        // Example 1: Find all function definitions using kind! macro
        println!("\nExample 1: Finding function definitions using kind! macro");
        let function_def_id = kind!("function_definition");
        let mut cursor = tree.walk();

        fn visit_function_nodes(
            node: &tree_sitter::Node,
            function_def_id: u16,
            code: &str,
            cursor: &mut tree_sitter::TreeCursor,
        ) {
            if node.kind_id() == function_def_id {
                // Get the declarator field using field! macro
                let declarator_field = field!("declarator");
                if let Some(declarator) = node.child_by_field_id(declarator_field.get()) {
                    // Look for the identifier (name) field
                    let name_field = field!("name");
                    if let Some(name_node) = declarator.child_by_field_id(name_field.get()) {
                        println!(
                            "Found function: {}",
                            name_node.utf8_text(code.as_bytes()).unwrap()
                        );
                    }
                }
            }

            if cursor.goto_first_child() {
                loop {
                    visit_function_nodes(&cursor.node(), function_def_id, code, cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        visit_function_nodes(&root_node, function_def_id, &code, &mut cursor);

        // Example 2: Find all if statements using kw! macro
        println!("\nExample 2: Finding if statements using kw! macro");
        let if_keyword_id = kw!("if");
        let mut cursor = tree.walk();

        fn visit_if_nodes(
            node: &tree_sitter::Node,
            if_keyword_id: u16,
            code: &str,
            cursor: &mut tree_sitter::TreeCursor,
        ) {
            // Check if this is an if statement by its kind
            if node.kind() == "if_statement" {
                // Get the condition field using field! macro
                let condition_field = field!("condition");
                if let Some(condition) = node.child_by_field_id(condition_field.get()) {
                    println!(
                        "Found if statement with condition: {}",
                        condition.utf8_text(code.as_bytes()).unwrap()
                    );
                }
            }

            if cursor.goto_first_child() {
                loop {
                    visit_if_nodes(&cursor.node(), if_keyword_id, code, cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        visit_if_nodes(&root_node, if_keyword_id, &code, &mut cursor);

        // Example 3: Find all return statements and their values
        println!("\nExample 3: Finding return statements and their values");
        let return_keyword_id = kw!("return");
        let mut cursor = tree.walk();

        fn visit_return_nodes(
            node: &tree_sitter::Node,
            return_keyword_id: u16,
            code: &str,
            cursor: &mut tree_sitter::TreeCursor,
        ) {
            if node.kind() == "return_statement" {
                // Get the value field using field! macro
                let value_field = field!("value");
                if let Some(value) = node.child_by_field_id(value_field.get()) {
                    println!(
                        "Found return statement with value: {}",
                        value.utf8_text(code.as_bytes()).unwrap()
                    );
                } else {
                    println!("Found return statement without value");
                }
            }

            if cursor.goto_first_child() {
                loop {
                    visit_return_nodes(&cursor.node(), return_keyword_id, code, cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        visit_return_nodes(&root_node, return_keyword_id, &code, &mut cursor);
    }

    Ok(())
}

fn print_node(node: &tree_sitter::Node, source: &str, indent: usize) {
    let indent_str = " ".repeat(indent);
    let node_text = if node.child_count() == 0 {
        format!(" \"{}\"", node.utf8_text(source.as_bytes()).unwrap())
    } else {
        String::new()
    };

    println!(
        "{}{} ({}){}",
        indent_str,
        node.kind(),
        node.start_position().row,
        node_text
    );

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            print_node(&cursor.node(), source, indent + 2);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
