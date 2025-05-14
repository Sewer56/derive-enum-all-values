extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(AllValues)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| v.ident),
        _ => panic!("AllValues only works on enums"),
    };

    let enum_name = syn_item.ident;
    let expanded = quote! {
        impl #enum_name {
            pub const fn all_values() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants),* ]
            }
        }
    };

    expanded.into()
}
