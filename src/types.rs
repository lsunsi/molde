#[derive(Debug)]
pub enum Stmt {
    Lit(String),
    Show(syn::Ident),
    If(Template),
    Let(Template),
    For(Template),
}

#[derive(Debug)]
pub struct Template {
    pub ident: syn::Ident,
    pub stmts: Vec<Stmt>,
}
