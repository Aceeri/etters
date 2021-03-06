extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate synstructure;

use proc_macro::TokenStream;

mod getters;
mod setters;

#[proc_macro_derive(Getters, attributes(get))]
pub fn getters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for getters");

    // Build the impl
    let gen = getters::implement(&ast);
    
    println!("{}", gen.as_str());

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(Setters, attributes(set))]
pub fn setters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for setters");

    // Build the impl
    let gen = setters::implement(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}