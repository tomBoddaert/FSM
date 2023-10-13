//! This crate defines derive macros for the [`fsm`](https://github.com/tomBoddaert/fsm) library.
//! This should not be used directly, rather through the reexports in [`fsm`](https://github.com/tomBoddaert/fsm) with the `derive` feature enabled.

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo,
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::get_unwrap,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::undocumented_unsafe_blocks
)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident};

#[proc_macro_derive(AcceptStates, attributes(accept))]
/// Auto-derives the `fsm::HasAcceptState` trait on an enum, given at least one variant is marked `#[accept]`.
///
/// # Panics
/// This will panic on any of the following conditions:
/// - This derive is run on anything but an enum
/// - The accept attribute is not just the path
/// - The accept attribute is used on a variant with fields
/// - The accept attribute is used more than once for one variant
pub fn accept_state_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let Data::Enum(data) = input.data else {
        panic!("#[derive(AcceptStates)] is only defined for enums!");
    };

    let accepted_variants = get_accepted_variants(data);

    proc_macro::TokenStream::from(produce_impl(&name, &accepted_variants))
}

fn get_accepted_variants(data: DataEnum) -> Vec<Ident> {
    let mut accepted_variants = Vec::new();

    for variant in data.variants {
        for attribute in variant.attrs {
            let path = attribute.meta.path();

            if !path.is_ident("accept") {
                continue;
            }

            assert!(
                attribute.meta.require_path_only().is_ok(),
                "#[accept] should only contain a path"
            );

            assert!(
                variant.fields.is_empty(),
                "#[accept] can only be used on variants with no fields."
            );

            assert!(
                accepted_variants.last() != Some(&variant.ident),
                "#[accept] can only be used once per variant"
            );

            accepted_variants.push(variant.ident.clone());
        }
    }

    accepted_variants
}

fn produce_impl(name: &Ident, accepted_variants: &[Ident]) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl fsm::AcceptStates for #name {
            fn is_accepted(&self) -> bool {
                match self {
                    #(Self::#accepted_variants => true,)*
                    _ => false,
                }
            }
        }
    }
}
