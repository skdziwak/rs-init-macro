//! This crate provides the `#[init]` attribute macro.
//! It does not do anything, but it is used by the `rs-init` crate to generate the `generated_init` function.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}