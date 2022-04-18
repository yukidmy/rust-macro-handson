use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn greet2(_: TokenStream, _: TokenStream) -> TokenStream {
    // r#"fn greet2() { println!("Hello world!") }"#.parse().unwrap()
    quote::quote!(
        fn greet2() { println!("Hello world!") }
    ).into()
}

#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::LitStr);
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    let sig = &item.sig;
    let block = &item.block;

    let args = sig.inputs.iter().filter(|variant| {
        match variant {
                    syn::FnArg::Receiver(_) => false,
                    syn::FnArg::Typed(_) => true,
                }
    }).map(|variant| {
        match variant {
            syn::FnArg::Receiver(_) => unreachable!(),
            syn::FnArg::Typed(syn::PatType { pat, .. }) => quote::quote!(stringify!(#pat), #pat),
        }
    });
    let format = format!(
        "{{}}: {}({}) -> {{}}",
        sig.ident,
        sig.inputs.iter().filter(|variant| {
            match variant {
                        syn::FnArg::Receiver(_) => false,
                        syn::FnArg::Typed(_) => true,
                    }
        }).map(|variant| {
            match variant {
                syn::FnArg::Receiver(_) => unreachable!(),
                syn::FnArg::Typed(..) => "{}: {}",
            }
        }).collect::<Vec<_>>().join(", ")
    );

    quote::quote!(
        #sig {
            let result = #block;
            println!(#format, #attr, #(#args,)* result);
            result
        }
    )
    .into()
}
