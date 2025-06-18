use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

/// Returns the node kind ID for a given node kind name.
///
/// This macro is useful for matching against node kinds in pattern matching contexts.
///
/// # Arguments
///
/// * `kind_name` - A string literal representing the node kind name.
///
/// # Returns
///
/// The node kind ID as a `u16`.
///
/// # Errors
///
/// Generates a compile-time error if the provided node kind name is not valid
/// in the tree-sitter-c grammar.
///
/// # Examples
///
/// ```
/// use tree_sitter_c_proc::kind;
///
/// let function_def_id = kind!("function_definition");
/// ```
#[proc_macro]
pub fn kind(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    // Get the string value
    let requested_kind = string_literal.value();

    let language = tree_sitter_c::language();
    let found_id = language.id_for_node_kind(&requested_kind, true);

    if found_id != 0 {
        quote! {
            #found_id
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid node kind in the tree-sitter-c grammar")
        )
    }
    .into()
}

/// Returns the node kind ID for a given keyword.
///
/// This macro is similar to `kind!` but specifically for keywords.
///
/// # Arguments
///
/// * `keyword` - A string literal representing the keyword.
///
/// # Returns
///
/// The keyword ID as a `u16`.
///
/// # Errors
///
/// Generates a compile-time error if the provided keyword is not valid
/// in the tree-sitter-c grammar.
///
/// # Examples
///
/// ```
/// use tree_sitter_c_proc::kw;
///
/// let if_keyword_id = kw!("if");
/// ```
#[proc_macro]
pub fn kw(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    // Get the string value
    let requested_keyword = string_literal.value();

    let language = tree_sitter_c::language();
    let found_id = language.id_for_node_kind(&requested_keyword, false);

    if found_id != 0 {
        quote! {
            #found_id
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid keyword in the tree-sitter-c grammar")
        )
    }
    .into()
}

/// Returns the field ID for a given field name.
///
/// This macro is useful for checking if a node has a specific field.
///
/// # Arguments
///
/// * `field_name` - A string literal representing the field name.
///
/// # Returns
///
/// The field ID as a `std::num::NonZeroU16`.
///
/// # Errors
///
/// Generates a compile-time error if the provided field name is not valid
/// in the tree-sitter-c grammar.
///
/// # Examples
///
/// ```
/// use tree_sitter_c_proc::field;
///
/// let declarator_field_id = field!("declarator");
/// ```
#[proc_macro]
pub fn field(token_stream: TokenStream) -> TokenStream {
    let string_literal: LitStr = parse_macro_input!(token_stream);

    // Get the string value
    let requested_field = string_literal.value();

    let language = tree_sitter_c::language();
    let found_id = language.field_id_for_name(&requested_field);

    if let Some(found_id) = found_id {
        let id_number: u16 = found_id;
        quote! {
            std::num::NonZeroU16::new(#id_number).unwrap()
        }
    } else {
        quote_spanned!(
            string_literal.span() =>
            compile_error!("This is not a valid field in the tree-sitter-c grammar")
        )
    }
    .into()
}
