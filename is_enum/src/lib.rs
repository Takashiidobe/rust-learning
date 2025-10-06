use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, ItemFn, parse_macro_input};

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

#[proc_macro_attribute]
pub fn log_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let name = &sig.ident;
    let body = &input.block;

    let args: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(pat_ident.ident.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let fmt = args
        .iter()
        .map(|id| format!("{} = {{:?}}", id))
        .collect::<Vec<_>>()
        .join(", ");

    let expanded = quote! {
        #vis #sig {
            println!(
                concat!("calling ", stringify!(#name), "(", #fmt, ")"),
                #(#args),*
            );
            #body
        }
    };

    expanded.into()
}
