// Tests for the tree-sitter-c-proc macros
use tree_sitter_c_proc::{field, kind, kw};

#[test]
fn test_kind_macro() {
    // Test with valid node kinds
    let function_def_id = kind!("function_definition");
    assert!(
        function_def_id > 0,
        "Expected non-zero ID for function_definition"
    );

    let translation_unit_id = kind!("translation_unit");
    assert!(
        translation_unit_id > 0,
        "Expected non-zero ID for translation_unit"
    );

    let binary_expression_id = kind!("binary_expression");
    assert!(
        binary_expression_id > 0,
        "Expected non-zero ID for binary_expression"
    );

    // Verify that different node kinds have different IDs
    assert_ne!(
        function_def_id, translation_unit_id,
        "Different node kinds should have different IDs"
    );
    assert_ne!(
        function_def_id, binary_expression_id,
        "Different node kinds should have different IDs"
    );
    assert_ne!(
        translation_unit_id, binary_expression_id,
        "Different node kinds should have different IDs"
    );

    // Note: Testing with invalid node kinds would cause compile errors, so we can't test that here
    // Example of what would cause a compile error:
    // kind!("not_a_valid_node_kind") // This would fail at compile time
}

#[test]
fn test_kw_macro() {
    // Test with valid keywords
    let if_id = kw!("if");
    assert!(if_id > 0, "Expected non-zero ID for 'if' keyword");

    let for_id = kw!("for");
    assert!(for_id > 0, "Expected non-zero ID for 'for' keyword");

    let while_id = kw!("while");
    assert!(while_id > 0, "Expected non-zero ID for 'while' keyword");

    // Verify that different keywords have different IDs
    assert_ne!(
        if_id, for_id,
        "Different keywords should have different IDs"
    );
    assert_ne!(
        if_id, while_id,
        "Different keywords should have different IDs"
    );
    assert_ne!(
        for_id, while_id,
        "Different keywords should have different IDs"
    );

    // Note: Testing with invalid keywords would cause compile errors, so we can't test that here
    // Example of what would cause a compile error:
    // kw!("not_a_valid_keyword") // This would fail at compile time
}

#[test]
fn test_field_macro() {
    // Test with valid fields
    let declarator_id = field!("declarator");
    assert!(
        declarator_id.get() > 0,
        "Expected non-zero ID for 'declarator' field"
    );

    let name_id = field!("name");
    assert!(name_id.get() > 0, "Expected non-zero ID for 'name' field");

    let type_id = field!("type");
    assert!(type_id.get() > 0, "Expected non-zero ID for 'type' field");

    // Verify that different fields have different IDs
    assert_ne!(
        declarator_id, name_id,
        "Different fields should have different IDs"
    );
    assert_ne!(
        declarator_id, type_id,
        "Different fields should have different IDs"
    );
    assert_ne!(
        name_id, type_id,
        "Different fields should have different IDs"
    );

    // Note: Testing with invalid fields would cause compile errors, so we can't test that here
    // Example of what would cause a compile error:
    // field!("not_a_valid_field") // This would fail at compile time
}

#[test]
fn test_macro_integration() {
    // Test using the macros together in a realistic scenario
    let function_def_id = kind!("function_definition");
    let declarator_id = field!("declarator").get();
    let name_id = field!("name").get();

    // Verify that the IDs are valid
    assert!(
        function_def_id > 0,
        "Expected non-zero ID for function_definition"
    );
    assert!(
        declarator_id > 0,
        "Expected non-zero ID for declarator field"
    );
    assert!(name_id > 0, "Expected non-zero ID for name field");

    // In a real scenario, we would use these IDs to navigate a tree-sitter parse tree
    // For example, to find the name of a function definition:
    // node.child_by_field_id(declarator_id)
    //     .and_then(|n| n.child_by_field_id(declarator_id))
    //     .and_then(|n| n.child_by_field_id(name_id))
}
