//! Generates methods to match enum variant.
//!
//! # Example
//!
//! ```rust
//! # macro_rules! dont_test { () => {
//! use is_variant::IsVariant;
//! 
//! #[derive(IsVariant)]
//! enum TestEnum {
//!     A,
//!     B(),
//!     C(i32, i32),
//!     D { _name: String, _age: i32 },
//!     VariantTest,
//! }
//! 
//! fn main() {
//!     let x = TestEnum::C(1, 2);
//!     assert!(x.is_c());
//! 
//!     let x = TestEnum::A;
//!     assert!(x.is_a());
//! 
//!     let x = TestEnum::B();
//!     assert!(x.is_b());
//! 
//!     let x = TestEnum::D {_name: "Jane Doe".into(), _age: 30 };
//!     assert!(x.is_d());
//!
//!     let x = TestEnum::VariantTest;
//!     assert!(x.is_variant_test());
//! }
//! # }}
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

use convert_case::{Case, Casing};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into();
    };
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    match data {
        Data::Enum(data_enum) => {
            variant_checker_functions = TokenStream2::new();

            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;

                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                let mut is_variant_func_name =
                    format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));
                is_variant_func_name.set_span(variant_name.span());

                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    fn #is_variant_func_name(&self) -> bool {
                        match self {
                            #name::#variant_name #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
}
