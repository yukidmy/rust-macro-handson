use proc_macro::TokenStream;
use quote::format_ident;

#[proc_macro]
pub fn greet(input: TokenStream) -> TokenStream {
    // r#"fn greet() { println!("Hello world!") }"#.parse().unwrap()
    match input.to_string().as_str() {
        "hello" => r#"fn greet() { println!("Hello world!") }"#.parse().unwrap(),
        "goodbye" => r#"fn greet() { println!("Bye bye!") }"#.parse().unwrap(),
        _ => unreachable!(),
    }
}

#[proc_macro]
pub fn printable(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::ItemEnum);
    let name = &item.ident;
    let variants = item.variants.iter().map(|variant| &variant.ident);
    quote::quote!(
        #item
        impl #name {
            fn print(&self) {
                match self { #(Self::#variants => println!("{}::{}", stringify!(#name), stringify!(#variants)),)* }
            }
        }
    )
    .into()
}

#[proc_macro]
pub fn make_fieldless(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::ItemEnum);
    let name = &item.ident;
    let another_name = format_ident!("{}WithoutFields", name);
    let variants: Vec<&syn::Ident> = item.variants.iter().map(|variant| &variant.ident).collect();
    // let fields = item.variants.iter().map(|variant| &variant.fields);
    let variants_with_matcher = item.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match variant.fields {
            syn::Fields::Unit => quote::quote!(#ident),
            syn::Fields::Named(..) => quote::quote!(#ident {..}),
            syn::Fields::Unnamed(..) => quote::quote!(#ident (..)),
        }
    });
    quote::quote!(
        #item
        enum #another_name {
            #(#variants,)*
        }
        impl #name {
            fn convert(self) -> #another_name {
                match self {
                    // Self::A(..) => return ParamWithoutFields::A,
                    // Self::B{..} => return ParamWithoutFields::B,
                    // Self::C => return ParamWithoutFields::C,
                    #(Self::#variants_with_matcher => ParamWithoutFields::#variants,)*
                }
            }
        }
    )
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
