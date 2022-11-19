#[proc_macro_derive(ImplDisplayForError)]
pub fn derive_impl_display_for_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplDisplayForError syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplDisplayForError only work on structs and enums!")
        }
        syn::Data::Struct(_) => {
            let gen = quote::quote! {
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.source)
                    }
                }
            };
            gen.into()
        }
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                match v.fields {
                    syn::Fields::Unit => {
                        panic!("ImplDisplayForError still not work with syn::Fields::Unit")
                    }
                    syn::Fields::Named(fields_named) => {
                        let one = fields_named.clone();
                        let two = fields_named.clone();
                        let mut scopes = one.named.iter().map(|_| String::from("{}\n")).fold(
                            String::from(""),
                            |mut acc, elem| {
                                acc.push_str(&elem);
                                acc
                            },
                        );
                        if !scopes.is_empty() {
                            scopes.pop();
                        }
                        let fields_idents = two.named.iter().map(|field| {
                            let field_ident = field
                                .ident
                                .clone()
                                .expect("some of named fields doesnt have ident");
                            quote::quote! { #field_ident }
                        });
                        let fields_idents_map = fields_named.named.iter().map(|field| {
                            let field_ident = field
                                .ident
                                .clone()
                                .expect("some of named fields doesnt have ident");
                            quote::quote! { #field_ident }
                        });
                        quote::quote! {
                            #ident::#variant_ident{
                                #(#fields_idents,)*
                            } => {
                                write!(
                                    f,
                                    #scopes,
                                    #(#fields_idents_map,)*
                                )
                            }
                        }
                    }
                    syn::Fields::Unnamed(_) => quote::quote! {
                        #ident::#variant_ident(e) => write!(f, "{}", e)
                    },
                }
            });
            let gen = quote::quote! {
                impl std::fmt::Display for #ident {
                     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variants,)*
                        }
                    }
                }
            };
            gen.into()
        }
    }
}
