use crate::types::{Stmt, Template};
use std::iter::Peekable;

pub(crate) fn parse(ident: syn::Ident, raw: String) -> Template {
    let mut chars = raw.chars().peekable();

    let mut pos_stmts = Vec::new();

    loop {
        pos_stmts.push(parse_unopen(&mut chars));
        if chars.peek().is_none() {
            break;
        }

        pos_stmts.push(parse_open(&mut chars));
        if chars.peek().is_none() {
            break;
        }
    }

    Template {
        ident,
        pos_stmts,
        neg_stmts: Vec::new(),
    }
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
        IF => {
            let (pos_stmts, neg_stmts) = parse_until_close(chars, IF);
            Stmt::If(Template {
                pos_stmts,
                neg_stmts,
                ident,
            })
        }
        LET => {
            let (pos_stmts, neg_stmts) = parse_until_close(chars, LET);
            Stmt::Let(Template {
                pos_stmts,
                neg_stmts,
                ident,
            })
        }
        FOR => {
            let (pos_stmts, neg_stmts) = parse_until_close(chars, FOR);
            Stmt::For(Template {
                pos_stmts,
                neg_stmts,
                ident,
            })
        }
        _ => panic!("Unknown statement key"),
    }
}

fn parse_until_close(
    chars: &mut Peekable<std::str::Chars>,
    k: &'static str,
) -> (Vec<Stmt>, Vec<Stmt>) {
    let mut pos_stmts = Vec::new();

    loop {
        pos_stmts.push(parse_unopen(chars));
        if chars.peek().is_none() {
            return (pos_stmts, Vec::new());
        }

        if let Some(_) = chars.next_if_eq(&ELSEC) {
            assert_eq!(&chars.take(4).collect::<String>(), ELSE);
            assert_eq!(chars.next().unwrap(), CLOSE);
            break;
        }

        if let Some(_) = chars.next_if_eq(&CLOSE2) {
            assert_eq!(&chars.take(k.len()).collect::<String>(), k);
            assert_eq!(chars.next().unwrap(), CLOSE);
            return (pos_stmts, Vec::new());
        }

        pos_stmts.push(parse_open(chars));
        if chars.peek().is_none() {
            return (pos_stmts, Vec::new());
        }
    }

    let mut neg_stmts = Vec::new();

    loop {
        neg_stmts.push(parse_unopen(chars));
        if chars.peek().is_none() {
            break;
        }

        if let Some(_) = chars.next_if_eq(&CLOSE2) {
            assert_eq!(&chars.take(k.len()).collect::<String>(), k);
            assert_eq!(chars.next().unwrap(), CLOSE);
            break;
        }

        neg_stmts.push(parse_open(chars));
        if chars.peek().is_none() {
            break;
        }
    }

    (pos_stmts, neg_stmts)
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
const ELSEC: char = '%';

const IF: &'static str = "if";
const LET: &'static str = "let";
const FOR: &'static str = "for";
const ELSE: &'static str = "else";
