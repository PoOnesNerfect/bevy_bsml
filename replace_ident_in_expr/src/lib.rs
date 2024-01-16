#![allow(
    clippy::blocks_in_if_conditions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::manual_find,
    clippy::manual_let_else,
    clippy::manual_map,
    clippy::map_unwrap_or,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::range_plus_one,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::wrong_self_convention
)]

extern crate proc_macro;

use proc_macro::{token_stream, Group, Ident, TokenStream, TokenTree};

#[proc_macro]
pub fn replace_ident_in_expr(input: TokenStream) -> TokenStream {
    let mut it = input.into_iter();

    // Get first parameters
    let target = get_ident(&mut it);
    let _comma = it.next().unwrap();
    let replace = get_ident(&mut it);
    let _comma = it.next().unwrap();

    // Return the remaining tokens, but replace identifiers.
    replace_ident(target, replace, it)
}

fn replace_ident(target: Ident, replace: Ident, input: token_stream::IntoIter) -> TokenStream {
    // Return the remaining tokens, but replace identifiers.
    input
        .map(|tt| {
            // println!("{:?}", tt);
            match tt {
                // Comparing `Ident`s can only be done via string comparison right
                // now. Note that this ignores syntax contexts which can be a
                // problem in some situation.
                TokenTree::Ident(ref i) if i.to_string() == target.to_string() => {
                    TokenTree::Ident(replace.clone())
                }
                TokenTree::Group(group) => TokenTree::Group(Group::new(
                    group.delimiter(),
                    replace_ident(target.clone(), replace.clone(), group.stream().into_iter()),
                )),
                // All other tokens are just forwarded
                other => other,
            }
        })
        .collect()
}

/// Extract an identifier from the iterator.
fn get_ident(it: &mut token_stream::IntoIter) -> Ident {
    match it.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => panic!("Expected identifier"),
    }
}
