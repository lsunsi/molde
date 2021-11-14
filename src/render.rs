use crate::{
    case::pascal,
    types::{Stmt, Template},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn structs(t: &Template) -> TokenStream {
    let root_ident = quote::format_ident!("{}", pascal(&t.ident.to_string()));

    let mut list = vec![];
    let mut props = vec![];

    for stmt in &t.pos_stmts {
        match stmt {
            Stmt::Let(t) => {
                let (ident, items) = structs2(&root_ident, t);
                list.extend(items);

                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: Option<#ident> });
            }
            Stmt::For(t) => {
                let (ident, items) = structs2(&root_ident, t);
                list.extend(items);

                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: Vec<#ident> });
            }
            Stmt::Show(prop_ident) => {
                props.push(quote! { #prop_ident: String });
            }
            Stmt::If(t) => {
                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: bool });
            }
            Stmt::Lit(_) => {}
        }
    }

    list.push(quote! { struct #root_ident { #(#props),* } });
    quote! { #(#list)* }
}

fn structs2(parent_ident: &Ident, t: &Template) -> (Ident, Vec<TokenStream>) {
    let ident = quote::format_ident!("{}{}", parent_ident, pascal(&t.ident.to_string()));

    let mut list = vec![];
    let mut props = vec![];

    for stmt in &t.pos_stmts {
        match stmt {
            Stmt::Let(t) => {
                let (inner_ident, items) = structs2(&ident, t);
                list.extend(items);

                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: Option<#inner_ident> });
            }
            Stmt::For(t) => {
                let (inner_ident, items) = structs2(&ident, t);
                list.extend(items);

                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: Vec<#inner_ident> });
            }
            Stmt::Show(prop_ident) => {
                props.push(quote! { #prop_ident: String });
            }
            Stmt::If(t) => {
                let prop_ident = &t.ident;
                props.push(quote! { #prop_ident: bool });
            }
            Stmt::Lit(_) => {}
        }
    }

    list.push(quote! { struct #ident { #(#props),* } });
    (ident, list)
}

pub fn method(t: Template) -> TokenStream {
    let root_ident = quote::format_ident!("{}", pascal(&t.ident.to_string()));

    let parent_ident = quote::format_ident!("{}", "self");
    let renders = render_stmts(&parent_ident, t.pos_stmts);

    quote! {
        impl #root_ident {
            fn render(self) -> String {
                [#(#renders),*].concat()
            }
        }
    }
}

fn render_stmts(parent_ident: &Ident, stmts: Vec<Stmt>) -> Vec<TokenStream> {
    let mut renders = vec![];

    for stmt in stmts {
        let rendered = match stmt {
            Stmt::Lit(a) => quote! { #a.to_owned() },
            Stmt::Show(ident) => quote! { #parent_ident.#ident },
            Stmt::If(Template {
                ident,
                pos_stmts,
                neg_stmts,
            }) => {
                let pos_stmts = render_stmts(parent_ident, pos_stmts);
                let neg_stmts = render_stmts(parent_ident, neg_stmts);

                let pos_stmts_len = pos_stmts.len();
                let neg_stmts_len = neg_stmts.len();

                quote! {
                    #parent_ident.#ident
                        .then(|| ([#(#pos_stmts),*] as [String; #pos_stmts_len]).concat())
                        .unwrap_or_else(|| ([#(#neg_stmts),*] as [String; #neg_stmts_len]).concat())
                }
            }
            Stmt::Let(Template {
                ident,
                pos_stmts,
                neg_stmts,
            }) => {
                let pos_stmts = render_stmts(&ident, pos_stmts);
                let neg_stmts = render_stmts(parent_ident, neg_stmts);

                let pos_stmts_len = pos_stmts.len();
                let neg_stmts_len = neg_stmts.len();

                quote! {
                    #parent_ident.#ident
                        .map(|#ident| ([#(#pos_stmts),*] as [String; #pos_stmts_len]).concat())
                        .unwrap_or_else(|| ([#(#neg_stmts),*] as [String; #neg_stmts_len]).concat())
                }
            }
            Stmt::For(Template {
                ident,
                pos_stmts,
                neg_stmts,
            }) => {
                let pos_stmts = render_stmts(&ident, pos_stmts);
                let neg_stmts = render_stmts(parent_ident, neg_stmts);

                let pos_stmts_len = pos_stmts.len();
                let neg_stmts_len = neg_stmts.len();

                quote! {
                    #parent_ident.#ident.is_empty()
                        .then(|| ([#(#neg_stmts),*] as [String; #neg_stmts_len]).concat())
                        .unwrap_or_else(|| #parent_ident.#ident.into_iter().map(|#ident| ([#(#pos_stmts),*] as [String; #pos_stmts_len]).concat()).collect())
                }
            }
        };

        renders.push(rendered);
    }

    renders
}
