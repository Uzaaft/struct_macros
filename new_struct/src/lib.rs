use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident};

#[proc_macro_derive(NewStruct)]
pub fn remove_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Original struct name, visibility, and attributes
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let attrs = &input.attrs;

    // Define new struct name by prefixing "New" to the original struct name
    let new_struct_name = Ident::new(&format!("New{}", struct_name), struct_name.span());

    // Collect the derive attributes for traits from the original struct's attributes
    let derive_attrs: Vec<_> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("derive") {
                Some(attr)
            } else {
                None
            }
        })
        .collect();

    // Filter out the `id` field from the original struct fields
    let fields = match &input.data {
        Data::Struct(data_struct) => data_struct.fields.iter().filter(|field| {
            if let Some(ident) = &field.ident {
                ident != "id"
            } else {
                true
            }
        }),
        _ => panic!("RemoveId can only be used on structs"),
    };

    // Collect the remaining fields
    let new_fields: Vec<Field> = fields.cloned().collect();

    // Generate the new struct with the name `New<StructName>` without the `id` field
    let output = quote! {
        #(#derive_attrs)*
        #struct_vis struct #new_struct_name {
            #(#new_fields),*
        }
    };

    output.into()
}
