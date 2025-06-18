use std::io::{self, Read};
use tree_sitter::{Parser, TreeCursor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the parser
    let mut parser = Parser::new();

    // Set the language to C
    let language = tree_sitter_c::language();
    parser.set_language(language)?;

    // Read input from stdin or use a default example
    let code = if atty::is(atty::Stream::Stdin) {
        // No stdin input, use a default example with more complex C code
        r#"
        #include <stdio.h>
        #include <stdlib.h>

        // A simple structure
        typedef struct {
            int x;
            int y;
        } Point;

        // Function to create a new point
        Point* create_point(int x, int y) {
            Point* p = (Point*)malloc(sizeof(Point));
            if (p == NULL) {
                return NULL;
            }
            p->x = x;
            p->y = y;
            return p;
        }

        // Function to calculate distance between points
        double distance(Point* p1, Point* p2) {
            int dx = p2->x - p1->x;
            int dy = p2->y - p1->y;
            return sqrt(dx*dx + dy*dy);
        }

        int main() {
            Point* p1 = create_point(0, 0);
            Point* p2 = create_point(3, 4);

            if (p1 != NULL && p2 != NULL) {
                printf("Distance: %f\n", distance(p1, p2));
                free(p1);
                free(p2);
            } else {
                printf("Memory allocation failed\n");
                return 1;
            }

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

    // Demonstrate using the proc macros
    #[cfg(feature = "proc_macros")]
    {
        use std::collections::HashMap;
        use tree_sitter_c_proc::{field, kind, kw};

        println!("Advanced usage of tree-sitter-c-proc macros\n");

        // Example 1: Find all struct definitions and their fields
        println!("Example 1: Finding struct definitions and their fields");
        let struct_specifier_id = kind!("struct_specifier");
        let field_declaration_id = kind!("field_declaration");
        let mut cursor = tree.walk();

        fn find_structs(
            node: &tree_sitter::Node,
            struct_specifier_id: u16,
            field_declaration_id: u16,
            code: &str,
            cursor: &mut TreeCursor,
        ) {
            if node.kind_id() == struct_specifier_id {
                // Get the name of the struct if it has one
                let name_field = field!("name");
                let name = if let Some(name_node) = node.child_by_field_id(name_field.get()) {
                    name_node.utf8_text(code.as_bytes()).unwrap().to_string()
                } else {
                    "anonymous".to_string()
                };

                println!("Found struct: {}", name);

                // Get the body field to find field declarations
                let body_field = field!("body");
                if let Some(body) = node.child_by_field_id(body_field.get()) {
                    // Iterate through the body to find field declarations
                    let mut field_cursor = body.walk();
                    if field_cursor.goto_first_child() {
                        loop {
                            let field_node = field_cursor.node();
                            if field_node.kind_id() == field_declaration_id {
                                // Get the type and declarator
                                let type_field = field!("type");
                                let declarator_field = field!("declarator");

                                if let Some(type_node) =
                                    field_node.child_by_field_id(type_field.get())
                                {
                                    if let Some(declarator) =
                                        field_node.child_by_field_id(declarator_field.get())
                                    {
                                        // Get the name of the field
                                        let name_field = field!("name");
                                        if let Some(name_node) =
                                            declarator.child_by_field_id(name_field.get())
                                        {
                                            println!(
                                                "  Field: {} of type {}",
                                                name_node.utf8_text(code.as_bytes()).unwrap(),
                                                type_node.utf8_text(code.as_bytes()).unwrap()
                                            );
                                        }
                                    }
                                }
                            }

                            if !field_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                }
            }

            if cursor.goto_first_child() {
                loop {
                    find_structs(
                        &cursor.node(),
                        struct_specifier_id,
                        field_declaration_id,
                        code,
                        cursor,
                    );
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        find_structs(
            &root_node,
            struct_specifier_id,
            field_declaration_id,
            &code,
            &mut cursor,
        );

        // Example 2: Analyze function calls and their arguments
        println!("\nExample 2: Analyzing function calls and their arguments");
        let call_expression_id = kind!("call_expression");
        let mut cursor = tree.walk();

        fn analyze_function_calls(
            node: &tree_sitter::Node,
            call_expression_id: u16,
            code: &str,
            cursor: &mut TreeCursor,
        ) {
            if node.kind_id() == call_expression_id {
                // Get the function name
                let function_field = field!("function");
                if let Some(function) = node.child_by_field_id(function_field.get()) {
                    let function_name = function.utf8_text(code.as_bytes()).unwrap();

                    // Get the arguments
                    let arguments_field = field!("arguments");
                    if let Some(arguments) = node.child_by_field_id(arguments_field.get()) {
                        let mut arg_count = 0;
                        let mut arg_cursor = arguments.walk();

                        if arg_cursor.goto_first_child() {
                            loop {
                                // Skip commas and other non-argument nodes
                                if arg_cursor.node().kind() != "," {
                                    arg_count += 1;
                                }

                                if !arg_cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                        }

                        println!(
                            "Found call to function '{}' with {} argument(s)",
                            function_name, arg_count
                        );
                    }
                }
            }

            if cursor.goto_first_child() {
                loop {
                    analyze_function_calls(&cursor.node(), call_expression_id, code, cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        analyze_function_calls(&root_node, call_expression_id, &code, &mut cursor);

        // Example 3: Find all variable declarations and their types
        println!("\nExample 3: Finding variable declarations and their types");
        let declaration_id = kind!("declaration");
        let mut cursor = tree.walk();

        // Keep track of variable types
        let mut variable_types: HashMap<String, String> = HashMap::new();

        fn find_variable_declarations(
            node: &tree_sitter::Node,
            declaration_id: u16,
            code: &str,
            cursor: &mut TreeCursor,
            variable_types: &mut HashMap<String, String>,
        ) {
            if node.kind_id() == declaration_id {
                // Get the type
                let type_field = field!("type");
                if let Some(type_node) = node.child_by_field_id(type_field.get()) {
                    let type_str = type_node.utf8_text(code.as_bytes()).unwrap().to_string();

                    // Get the declarator
                    let declarator_field = field!("declarator");
                    if let Some(declarator) = node.child_by_field_id(declarator_field.get()) {
                        // Get the name of the variable
                        let name_field = field!("name");
                        if let Some(name_node) = declarator.child_by_field_id(name_field.get()) {
                            let var_name =
                                name_node.utf8_text(code.as_bytes()).unwrap().to_string();
                            println!(
                                "Found variable declaration: {} of type {}",
                                var_name, type_str
                            );

                            // Store the variable type
                            variable_types.insert(var_name, type_str);
                        }
                    }
                }
            }

            if cursor.goto_first_child() {
                loop {
                    find_variable_declarations(
                        &cursor.node(),
                        declaration_id,
                        code,
                        cursor,
                        variable_types,
                    );
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        find_variable_declarations(
            &root_node,
            declaration_id,
            &code,
            &mut cursor,
            &mut variable_types,
        );

        // Print the variable types we've collected
        println!("\nVariable types collected:");
        for (var, type_str) in &variable_types {
            println!("  {} is a {}", var, type_str);
        }

        // Example 4: Using macros with variables
        println!("\nExample 4: Using macros with variables");

        // Using kind! macro with variables
        let function_def_const = kind!("function_definition");
        let if_statement_const = kind!("if_statement");

        // Using kw! macro with variables
        let if_kw_const = kw!("if");
        let return_kw_const = kw!("return");

        // Using field! macro with variables
        let body_field_const = field!("body");
        let condition_field_const = field!("condition");

        println!("Values from macros stored in variables:");
        println!("  kind!(\"function_definition\") = {}", function_def_const);
        println!("  kind!(\"if_statement\") = {}", if_statement_const);
        println!("  kw!(\"if\") = {}", if_kw_const);
        println!("  kw!(\"return\") = {}", return_kw_const);
        println!("  field!(\"body\") = {}", body_field_const);
        println!("  field!(\"condition\") = {}", condition_field_const);
    }

    Ok(())
}
