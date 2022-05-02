use proc_macro::TokenStream;

#[proc_macro_derive(Dump)]
pub fn aaaaaa(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &input.ident;
    let typename = match input.data {
        syn::Data::Enum(..) => "enum",
        syn::Data::Struct(..) => "struct",
        syn::Data::Union(..) => "union",
    };
    let mems = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(named) => named.named.into_iter().map(|var| {
                var.ident
            }).collect::<Vec<_>>(),
            _ => todo!(),
        },
        _ => todo!(),
    };
    quote::quote!(
        impl Dump for #ident {
            fn dump(&self) {
                println!("{} {} {{", #typename, stringify!(#ident));
                #(println!("  {}: {:?},", stringify!(#mems), self.#mems);)*
                println!("}}");
            }
        }
    )
    .into()
    // Note: `format!().parse();` for string
}

// // Use ItemStruct
// #[proc_macro_derive(Dump)]
// pub fn derive_dump(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as syn::ItemStruct);
//     let ident = &input.ident;
//     let field_idents = input.fields.iter().map(|field| &field.ident);
//     quote::quote!(
//         impl Dump for #ident {
//             fn dump(&self) {
//                 struct D<'a>(&'a #ident);
//                 impl std::fmt::Debug for D<'_> {
//                     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                         f.debug_struct(stringify!(#ident))
//                          #(.field(stringify!(#field_idents), &self.0.#field_idents))*
//                          .finish()
//                     }
//                 }

//                 println!("struct {:#?}", D(self));
//             }
//         }
//     )
//     .into()
// }
