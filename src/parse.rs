use crate::types::{Stmt, Template};
use std::iter::Peekable;

pub(crate) fn parse(ident: syn::Ident, raw: String) -> Template {
    let mut chars = raw.chars().peekable();

    let mut stmts = Vec::new();

    loop {
        stmts.push(parse_unopen(&mut chars));
        if chars.peek().is_none() {
            break;
        }

        stmts.push(parse_open(&mut chars));
        if chars.peek().is_none() {
            break;
        }
    }

    Template { ident, stmts }
}

fn parse_unopen(chars: &mut Peekable<std::str::Chars>) -> Stmt {
    let word = chars.take_while(|c| c != &OPEN);
    return Stmt::Lit(word.collect());
}

fn parse_open(chars: &mut Peekable<std::str::Chars>) -> Stmt {
    match chars.next_if_eq(&STMT) {
        Some(_) => parse_open_stmt(chars),
        None => parse_open_unstmt(chars),
    }
}

fn parse_open_stmt(chars: &mut Peekable<std::str::Chars>) -> Stmt {
    let word = parse_word(chars);
    let ident = parse_ident(chars);

    match word.as_ref() {
        IF => Stmt::If(Template {
            stmts: parse_until_close(chars, IF),
            ident,
        }),
        LET => Stmt::Let(Template {
            stmts: parse_until_close(chars, LET),
            ident,
        }),
        FOR => Stmt::For(Template {
            stmts: parse_until_close(chars, FOR),
            ident,
        }),
        _ => panic!("Unknown statement key"),
    }
}

fn parse_until_close(chars: &mut Peekable<std::str::Chars>, k: &'static str) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    loop {
        stmts.push(parse_unopen(chars));
        if chars.peek().is_none() {
            break;
        }

        if let Some(_) = chars.next_if_eq(&CLOSE2) {
            assert_eq!(&chars.take(k.len()).collect::<String>(), k);
            assert_eq!(chars.next().unwrap(), CLOSE);
            break;
        }

        stmts.push(parse_open(chars));
        if chars.peek().is_none() {
            break;
        }
    }

    stmts
}

fn parse_open_unstmt(chars: &mut Peekable<std::str::Chars>) -> Stmt {
    Stmt::Show(parse_ident(chars))
}

fn parse_word(chars: &mut Peekable<std::str::Chars>) -> String {
    let word = chars.take_while(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase());
    word.collect()
}

fn parse_ident(chars: &mut Peekable<std::str::Chars>) -> syn::Ident {
    let word = chars.take_while(|c| c != &CLOSE);
    quote::format_ident!("{}", word.collect::<String>())
}

const OPEN: char = '{';
const CLOSE: char = '}';
const STMT: char = '#';
const CLOSE2: char = '/';

const IF: &'static str = "if";
const LET: &'static str = "let";
const FOR: &'static str = "for";
