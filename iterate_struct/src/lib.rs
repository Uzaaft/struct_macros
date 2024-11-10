extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(IterateFields)]
pub fn iterate_fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let gen = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn iterate_fields(&self) -> impl Iterator<Item = (&str, &dyn std::any::Any)> {
                let fields: Vec<(&str, &dyn std::any::Any)> = vec![
                    #(
                        (
                            stringify!(#field_names),
                            &(self.#field_names) as &dyn std::any::Any
                        ),
                    )*
                ];
                fields.into_iter()
            }
        }
    };

    TokenStream::from(gen)
}
