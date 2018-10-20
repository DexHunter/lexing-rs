#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::collections::HashSet;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct CharTable {
    pub quotation_mark_vec: HashSet <char>,
    pub char_vec: HashSet <char>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Token {
    Quote {
        quotation_mark: char,
        string: String,
        span: Span,
    },
    Word {
        word: String,
        span: Span,
    },
    Char {
        ch: char,
        span: Span,
    },
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct LexingError;

#[test]
fn test_lexing () -> Result<(), LexingError> {
    Ok (())
}

#[test]
fn play () {

}
