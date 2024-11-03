use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Ident, PathArguments,
    PathSegment, Type, TypePath,
};

#[proc_macro_derive(UpdateStruct)]
pub fn update_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Original struct name and visibility
    let struct_name = &input.ident;
    let struct_vis = &input.vis;

    // Define the new struct name with "Update" prefixed
    let update_struct_name = Ident::new(&format!("Update{}", struct_name), struct_name.span());

    // Filter out the `id` field and make remaining fields optional by wrapping them in `Option`
    let fields = match &input.data {
        Data::Struct(data_struct) => data_struct.fields.iter().filter(|field| {
            if let Some(ident) = &field.ident {
                ident != "id"
            } else {
                true
            }
        }),
        _ => panic!("UpdateStruct can only be used on structs"),
    };

    // Transform fields to be of type `Option<T>`
    let optional_fields: Vec<Field> = fields
        .map(|field| {
            let mut new_field = field.clone();
            new_field.ty = Type::Path(TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: {
                        let mut segments = syn::punctuated::Punctuated::new();
                        segments.push(PathSegment {
                            ident: Ident::new("Option", new_field.ty.span()),
                            arguments: PathArguments::AngleBracketed(
                                syn::AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: std::iter::once(syn::GenericArgument::Type(
                                        new_field.ty.clone(),
                                    ))
                                    .collect(),
                                    gt_token: Default::default(),
                                },
                            ),
                        });
                        segments
                    },
                },
            });
            new_field
        })
        .collect();

    // Generate the new struct with "Update" prefix and optional fields
    let output = quote! {
        #struct_vis struct #update_struct_name {
            #(#optional_fields),*
        }
    };

    output.into()
}
