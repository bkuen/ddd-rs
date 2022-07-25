use syn::{AttributeArgs, parse_macro_input};
use syn::token::Struct;
use crate::aggregate_root::AggregateRootMeta;

pub mod aggregate_root;

#[proc_macro_attribute]
pub fn test(metadata: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as Struct);
    let args = parse_macro_input!(metadata as AttributeArgs);

    // let meta = AggregateRootMeta::new(&input, &args);
    // let output = meta.expand();

    output.into()
}
