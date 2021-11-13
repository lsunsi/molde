mod case;
mod parse;
mod render;
mod types;

#[proc_macro]
pub fn molde(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = syn::parse_macro_input!(tokens as syn::LitStr).value();
    let path = std::path::Path::new(&literal);

    let ident = &path
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_owned();

    let raw = std::fs::read_to_string(path).unwrap();
    let ident = quote::format_ident!("{}", ident);

    let template = parse::parse(ident.clone(), raw);
    let structs = render::structs(&template);
    let method = render::method(template);

    (quote::quote! {#structs #method}).into()
}
