use crate::type_validation::{is_datetime, is_uuid};

use quote::quote;
use syn::{Ident, DeriveInput, FieldsNamed, Field};
use syn::spanned::Spanned;

pub struct DomainEventMeta<'a> {
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
                _ => Err(syn::Error::new(input.ident.span(), "`DomainEvent` does not contain named fields"))
            }
        }
        _ => Err(syn::Error::new(input.ident.span(), "`DomainEvent` has to be used with structs"))
    }
}

impl<'a> DomainEventMeta<'a> {
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
        self.check_timestamp_field()?;

        Ok(())
    }

    pub(crate) fn id_field(&self) -> Option<&Field> {
        self.fields.named
            .iter()
            .find(|field| field.ident.as_ref().unwrap().to_string().contains("id"))
    }

    pub(crate) fn timestamp_field(&self) -> Option<&Field> {
        self.fields.named
            .iter()
            .find(|field| field.ident.as_ref().unwrap().to_string().contains("timestamp"))
    }

    pub(crate) fn check_id_field(&self) -> Result<(), syn::Error> {
        if let Some(field) = self.id_field() {

            if !is_uuid(field) {
                return Err(syn::Error::new(self.ident.span(), "`DomainEvent` does not contain id field of type `Uuid`"));
            }

            Ok(())
        } else {
            Err(syn::Error::new(self.ident.span(), "`DomainEvent` does not contain id field of type `Uuid`"))
        }
    }

    pub(crate) fn check_timestamp_field(&self) -> Result<(), syn::Error> {
        if let Some(field) = self.timestamp_field() {

            if !is_datetime(field) {
                return Err(syn::Error::new(self.ident.span(), "`DomainEvent` does not contain timestamp field of type `DateTime<Utc>`"));
            }

            Ok(())
        } else {
            Err(syn::Error::new(self.ident.span(), "`DomainEvent` does not contain timestamp field of type `DateTime<Utc>`"))
        }
    }

    pub(crate) fn expand(&mut self) -> proc_macro2::TokenStream {
        let ident = self.ident;

        let id_field = self.id_field().unwrap();
        let id_field_ident = id_field.ident.as_ref().unwrap();

        let timestamp_field = self.timestamp_field().unwrap();
        let timestamp_field_ident = timestamp_field.ident.as_ref().unwrap();

        quote! {

            impl DomainEvent for #ident {
                fn id(&self) -> Uuid {
                    self.#id_field_ident
                }

                fn timestamp(&self) -> DateTime<Utc> {
                    self.#timestamp_field_ident
                }
            }
        }
    }
}