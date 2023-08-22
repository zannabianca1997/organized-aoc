//! Crate to create neat aoc libraries

use aoc_runtime::__private::macros;

#[proc_macro]
pub fn library(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::library(input.into()).into()
}

#[proc_macro]
pub fn year(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::year(input.into()).into()
}

#[proc_macro]
pub fn day(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::day(input.into()).into()
}

#[proc_macro_attribute]
pub fn solution(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::solution(attr.into(), item.into()).into()
}
