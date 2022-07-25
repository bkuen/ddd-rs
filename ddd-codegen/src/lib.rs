use crate::aggregate_root::AggregateRootMeta;
use crate::event::DomainEventMeta;

mod aggregate_root;
mod event;
use syn::{DeriveInput, parse_macro_input};
mod type_validation;

#[proc_macro_derive(AggregateRoot)]
pub fn aggregate_root(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let mut meta = match AggregateRootMeta::new(&mut input) {
        Err(err) => return err.to_compile_error().into(),
        Ok(meta) => meta,
    };

    meta.expand().into()
}

#[proc_macro_derive(DomainEvent)]
pub fn domain_event(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let mut meta = match DomainEventMeta::new(&mut input) {
        Err(err) => return err.to_compile_error().into(),
        Ok(meta) => meta,
    };

    meta.expand().into()
}