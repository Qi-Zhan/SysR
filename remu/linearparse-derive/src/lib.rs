use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LinearParse)]
pub fn linear_parse_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_linear_parse(&ast)
}

fn impl_linear_parse(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        fields
    } else {
        panic!("LinearParses can only be derived for structs");
    };

    let mut parse_code = quote!();

    for field in fields {
        let field_ty = &field.ty;
        let construct = match field_ty {
            syn::Type::Array(_) => {
                quote! {
                        data[start..end].try_into().expect("Not enough bytes for the current field")
                }
            }
            _ => {
                quote! {
                    <#field_ty>::from_le_bytes(
                        data[start..end].try_into().expect("Not enough bytes for the current field")
                    )
                }
            }
        };
        let field_size = quote! {
            std::mem::size_of::<#field_ty>() / std::mem::size_of::<u8>()
        };

        let field_ident = field
            .ident
            .as_ref()
            .expect("Expected the struct to have named fields");

        parse_code.extend(quote! {
            #field_ident: {
                let start = offset;
                let end = start + #field_size;
                offset = end;
                #construct
            },
        });
    }

    let gen = quote! {
        impl LinearParse for #name {
            fn linearparse(data: &[u8]) -> Self {
                let mut offset = 0;
                Self {
                    #parse_code
                }
            }
        }
    };
    gen.into()
}
