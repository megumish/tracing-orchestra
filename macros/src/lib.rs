use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Orchestra)]
pub fn derive_orchestra(tokens: TokenStream) -> TokenStream {
    let struct_ast = syn::parse_macro_input!(tokens as syn::ItemStruct);
    let struct_attrs = struct_ast.attrs;
    let struct_vis = struct_ast.vis;
    let struct_name = struct_ast.ident;
    let struct_generics = struct_ast.generics.params;
    let struct_where = struct_ast.generics.where_clause;

    let ret = quote!(
        #(#struct_attrs)*
        #struct_vis impl<#struct_generics> tracing_orchestra::Orchestra for #struct_name<#struct_generics> #struct_where {}
    );
    eprintln!("{}", ret.to_string());
    ret.into()
}

#[proc_macro_attribute]
pub fn orchestra(attrs: TokenStream, implementation: TokenStream) -> TokenStream {
    let implementation_ast = syn::parse_macro_input!(implementation as syn::ItemImpl);
    let on_attrs: proc_macro2::TokenStream = attrs.into();
    let impl_attrs = implementation_ast.attrs;
    let impl_defaultness = implementation_ast.defaultness;
    let impl_unsafety = implementation_ast.unsafety;
    let impl_generics = implementation_ast.generics.params;
    let impl_trait = if let Some(t) = implementation_ast.trait_ {
        if let Some(_) = t.0 {
            let p = t.1;
            quote!(! #p for)
        } else {
            let p = t.1;
            if p.segments.is_empty() {
                quote!()
            } else {
                quote!(#p for)
            }
        }
    } else {
        TokenStream::new().into()
    };
    let impl_type = implementation_ast.self_ty;
    let impl_items = implementation_ast.items.into_iter().map(|item| match item {
        syn::ImplItem::Fn(impl_fn) => {
            let attrs = impl_fn
                .attrs
                .iter()
                .filter(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map(|x| x.ident == "instrument")
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();
            if attrs.is_empty() {
                quote!(
                    #[tracing::instrument(#on_attrs)]
                    #impl_fn
                )
            } else {
                let attrs_others = impl_fn
                    .attrs
                    .iter()
                    .filter(|attr2| {
                        attr2
                            .path()
                            .segments
                            .iter()
                            .last()
                            .map(|x| x.ident != "instrument")
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();
                let vis = impl_fn.vis;
                let defaultness = impl_fn.defaultness;
                let sig = impl_fn.sig;
                let block = impl_fn.block;
                quote!(
                    #(#attrs)*
                    #(#attrs_others)*
                    #vis #defaultness #sig #block
                )
            }
        }
        _ => {
            eprintln!("support only fn");
            quote!(#item)
        }
    });
    let ret = quote!(
        #(#impl_attrs)*
        #impl_defaultness #impl_unsafety impl #impl_generics #impl_trait #impl_type {
            #(#impl_items)*
        }
    );

    eprintln!("{}", ret.to_string());
    ret.into()
}
