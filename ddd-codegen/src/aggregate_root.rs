use crate::type_validation::{is_events_vec, is_uuid};

use quote::quote;
use syn::{Ident, DeriveInput, FieldsNamed, Field};
use syn::spanned::Spanned;

pub struct AggregateRootMeta<'a> {
    ident: &'a Ident,
    input: &'a DeriveInput,
    fields: &'a FieldsNamed,
}

pub(crate) fn extract_fields(input: &DeriveInput) -> Result<&FieldsNamed, syn::Error> {
    match &input.data {
        syn::Data::Struct(ref struct_data) => {
            return match &struct_data.fields {
                syn::Fields::Named(fields) => {
                    Ok(fields)
                }
                _ => Err(syn::Error::new(input.ident.span(), "`AggregateRoot` does not contain named fields"))
            }
        }
        _ => Err(syn::Error::new(input.ident.span(), "`AggregateRoot` has to be used with structs"))
    }
}

impl<'a> AggregateRootMeta<'a> {
    pub(crate) fn new(input: &'a DeriveInput) -> Result<Self, syn::Error> {
        let ident = &input.ident;

        let fields = extract_fields(input)?;

        let meta = Self {
            ident,
            input,
            fields,
        };

        meta.check_preconditions()?;

        Ok(meta)
    }

    pub(crate) fn check_preconditions(&self) -> Result<(), syn::Error> {
        self.check_id_field()?;
        self.check_events_field()?;

        Ok(())
    }

    pub(crate) fn id_field(&self) -> Option<&Field> {
        self.fields.named
            .iter()
            .find(|field| field.ident.as_ref().unwrap().to_string().contains("id"))
    }

    pub(crate) fn events_field(&self) -> Option<&Field> {
        self.fields.named
            .iter()
            .find(|field| field.ident.as_ref().unwrap().to_string().contains("events"))
    }

    pub(crate) fn check_id_field(&self) -> Result<(), syn::Error> {
        if let Some(field) = self.id_field() {

            if !is_uuid(field) {
                return Err(syn::Error::new(self.ident.span(), "`AggregateRoot` does not contain id field of type `Uuid`"));
            }

            Ok(())
        } else {
            Err(syn::Error::new(self.ident.span(), "`AggregateRoot` does not contain id field of type `Uuid`"))
        }
    }

    pub(crate) fn check_events_field(&self) -> Result<(), syn::Error> {
        if let Some(field) = self.events_field() {

            if !is_events_vec(field) {
                return Err(syn::Error::new(self.ident.span(), "`AggregateRoot` does not contain events field of type `Vec<Box<dyn DomainEvent>>`"));
            }

            Ok(())
        } else {
            Err(syn::Error::new(self.ident.span(), "`AggregateRoot` does not contain id field of type `Uuid`"))
        }
    }

    pub(crate) fn expand(&mut self) -> proc_macro2::TokenStream {
        let ident = self.ident;

        let id_field = self.id_field().unwrap();
        let id_field_ident = id_field.ident.as_ref().unwrap();

        let events_field = self.events_field().unwrap();
        let events_field_ident = events_field.ident.as_ref().unwrap();

        quote! {

            impl AggregateRoot for #ident {
                fn aggregate_id(&self) -> Uuid {
                    self.#id_field_ident
                }

                fn events(&self) -> &Vec<Box<dyn DomainEvent>> {
                    &self.#events_field_ident
                }

                fn add_event(&mut self, event: Box<dyn DomainEvent>) {
                    self.#events_field_ident.push(event);
                }

                fn clear_events(&mut self) {
                    self.#events_field_ident.clear();
                }
            }
        }
    }
}