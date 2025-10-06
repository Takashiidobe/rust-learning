use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(IsEnum)]
pub fn derive_enum_is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(name, "#[derive(IsEnum)] can only be used on enums")
            .to_compile_error()
            .into();
    };

    let methods = data_enum.variants.iter().map(|variant| {
        let v_ident = &variant.ident;
        let fn_name = Ident::new(
            &format!("is_{}", v_ident.to_string().to_lowercase()),
            v_ident.span(),
        );

        let pattern = if variant.fields.is_empty() {
            quote! { #name::#v_ident }
        } else {
            quote! { #name::#v_ident(..) }
        };

        quote! {
            pub fn #fn_name(&self) -> bool {
                matches!(self, #pattern)
            }
        }
    });

    quote! {
        impl #name {
            #(#methods)*
        }
    }
    .into()
}
