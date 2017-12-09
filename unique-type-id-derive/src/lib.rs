#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate unique_type_id;

use proc_macro::TokenStream;

#[proc_macro_derive(SequentialTypeId)]
pub fn sequential_type_id(input: TokenStream) -> TokenStream {
    implement_type_id(input, sequential_implementor)
}

fn inc_id() -> u64 {
    unsafe {
        static mut ID: u64 = 0u64;

        let old_value = ID;
        ID += 1;
        old_value
    }
}


fn implement_type_id(input: TokenStream, implementor: fn(&syn::DeriveInput) -> quote::Tokens) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = implementor(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}


fn sequential_implementor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let id = inc_id();

    quote! {
        impl #impl_generics unique_type_id::SequentialTypeId for #name #ty_generics #where_clause {
            fn id() -> unique_type_id::TypeId {
                unique_type_id::TypeId(#id)
            }
        }
    }
}
