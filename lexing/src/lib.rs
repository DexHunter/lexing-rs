#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::collections::HashSet;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Token <'a> {
    Quotation {
        span: Span,
        quotation_mark: char,
        string: &'a str,
    },
    Word {
        span: Span,
        word: &'a str,
    },
    Char {
        span: Span,
        ch: char,
    },
}

impl <'a> Token <'a> {
    pub fn span (&self) -> Span {
        match self {
            Token::Quotation { span, .. } => span.clone (),
            Token::Word { span, .. } => span.clone (),
            Token::Char { span, .. } => span.clone (),
        }
    }
}

impl <'a> Token <'a> {
    pub fn len (&self) -> usize {
        self.span () .len ()
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn len (&self) -> usize {
        self.hi - self.lo
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum LexError {
    QuotationMisMatch {
        quotation_mark: char,
        index: usize,
    },
    Internal,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct CharTable {
    pub quotation_mark_set: HashSet <char>,
    pub space_set: HashSet <char>,
    pub char_set: HashSet <char>,
}

impl CharTable {
    pub fn new () -> Self {
        CharTable {
            quotation_mark_set: HashSet::new (),
            space_set: HashSet::new (),
            char_set: HashSet::new (),
        }
    }
}

impl CharTable {
    pub fn quotation_mark (mut self, ch: char) -> Self {
        self.quotation_mark_set.insert (ch);
        self
    }
}

impl CharTable {
    pub fn space (mut self, ch: char) -> Self {
        self.space_set.insert (ch);
        self
    }
}

impl CharTable {
    pub fn char (mut self, ch: char) -> Self {
        self.char_set.insert (ch);
        self
    }
}

impl CharTable {
    pub fn char_p (&self, ch: char) -> bool {
        self.char_set.contains (&ch)
    }
}

impl CharTable {
    pub fn space_p (&self, ch: char) -> bool {
        self.space_set.contains (&ch)
    }
}

impl CharTable {
    pub fn quotation_mark_p (&self, ch: char) -> bool {
        self.quotation_mark_set.contains (&ch)
    }
}

impl <'a> CharTable {
    pub fn lex (
        &self,
        input: &'a str,
    ) -> Result <Vec <Token <'a>>, LexError> {
        let lexing = Lexing {
            cursor: 0,
            char_table: self.clone (),
            token_vec: Vec::new (),
            input,
        };
        lexing.run ()
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
struct Lexing <'a> {
    cursor: usize,
    input: &'a str,
    char_table: CharTable,
    token_vec: Vec <Token <'a>>,
}

impl <'a> Lexing <'a> {
    fn run (
        mut self,
    ) -> Result <Vec <Token <'a>>, LexError> {
        loop {
            self.ignore_space ();
            if self.finished_p () {
                return Ok (self.token_vec);
            }
            self.next_token ()?;
        }
    }
}

impl <'a> Lexing <'a> {
    fn finished_p (&self) -> bool {
        self.cursor == self.input.len ()
    }
}

impl <'a> Lexing <'a> {
    fn ignore_space (&mut self) {
        loop {
            let progress = &self.input [self.cursor ..];
            if let Some (ch) = progress.chars () .next () {
                if self.char_table.space_p (ch) {
                    self.cursor += ch.len_utf8 ();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }
}

// assumptions :
// - Lexing is not finished_p
// - spaces are ignored
impl <'a> Lexing <'a> {
    fn next_token (
        &mut self,
    ) -> Result <(), LexError> {
        let progress = &self.input [self.cursor ..];
        if let Some (ch) = progress.chars () .next () {
            if self.char_table.char_p (ch) {
                self.next_char (ch)
            } else if self.char_table.quotation_mark_p (ch) {
                self.next_quote (ch)
            } else {
                self.next_word ()
            }
        } else {
            Err (LexError::Internal)
        }
    }
}

impl <'a> Lexing <'a> {
    fn next_char (
        &mut self,
        ch: char,
    ) -> Result <(), LexError> {
        let lo = self.cursor;
        let ch_len = ch.len_utf8 ();
        self.cursor += ch_len;
        let hi = self.cursor;
        let span = Span { lo, hi };
        let token = Token::Char { span, ch };
        self.token_vec.push (token);
        Ok (())
    }
}

impl <'a> Lexing <'a> {
    fn next_quote (
        &mut self,
        quotation_mark: char,
    ) -> Result <(), LexError> {
        let lo = self.cursor;
        let ch_len = quotation_mark.len_utf8 ();
        self.cursor += ch_len;
        let progress = &self.input [self.cursor ..];
        if let Some (
            quote_end
        ) = find_quote_end (progress, quotation_mark) {
            let string = &progress [.. quote_end];
            self.cursor += string.len ();
            self.cursor += ch_len;
            let hi = self.cursor;
            let span = Span { lo, hi };
            let token = Token::Quotation {
                span, quotation_mark, string,
            };
            self.token_vec.push (token);
            Ok (())
        } else {
            Err (LexError::QuotationMisMatch {
                quotation_mark,
                index: lo,
            })
        }
    }
}

impl <'a> Lexing <'a> {
    fn goto_word_end (&mut self) {
        loop {
            let progress = &self.input [self.cursor ..];
            if let Some (ch) = progress.chars () .next () {
                if self.char_table.space_p (ch) {
                    return;
                } else if self.char_table.char_p (ch) {
                    return;
                } else if self.char_table.quotation_mark_p (ch) {
                    return;
                } else {
                    self.cursor += ch.len_utf8 ();
                }
            } else {
                return;
            }
        }
    }
}

fn find_quote_end (
    string: &str,
    quotation_mark: char,
) -> Option <usize> {
    let mut cursor = 0;
    loop {
        let progress = &string [cursor ..];
        let mut chars = progress.chars ();
        if let Some (ch) = chars.next () {
            if ch == quotation_mark {
                return Some (cursor);
            } else if ch == '\\' {
                cursor += 1;
                if let Some (ch) = chars.next () {
                    cursor += ch.len_utf8 ();
                } else {
                    return None;
                }
            } else {
                cursor += ch.len_utf8 ();
            }
        } else {
            return None;
        }
    }
}

impl <'a> Lexing <'a> {
    fn next_word (
        &mut self,
    ) -> Result <(), LexError> {
        let lo = self.cursor;
        self.goto_word_end ();
        let hi = self.cursor;
        let word = &self.input [lo .. hi];
        let span = Span { lo, hi };
        let token = Token::Word {
            span, word,
        };
        self.token_vec.push (token);
        Ok (())
    }
}

#[test]
fn test_lexing () -> Result<(), LexError> {
    let char_table = CharTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "sss" c;"#;
    let token_vec = char_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 0, hi: 2 },
        word: "aa",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 3, hi: 8 },
        quotation_mark: '"',
        string: "sss",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 9, hi: 10 },
        word: "c",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Char {
        span: Span { lo: 10, hi: 11 },
        ch: ';',
    });
    assert_eq! (iter.next (), None);
    Ok (())
}

#[test]
fn test_lexing_unicode () -> Result<(), LexError> {
    let char_table = CharTable::new ()
        .space ('\n') .space ('\t') .space (' ')
        .char ('「') .char ('」');
    let input = r#"子游曰「敢問其方」"#;
    let token_vec = char_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert! (
        if let Some (Token::Word { word, .. }) = iter.next () {
            word == &"子游曰"
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Char { ch, .. }) = iter.next () {
            ch == &'「'
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Word { word, .. }) = iter.next () {
            word == &"敢問其方"
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Char { ch, .. }) = iter.next () {
            ch == &'」'
        } else {
            false
        }
    );
    assert_eq! (iter.next (), None);
    Ok (())
}

#[test]
fn test_error () -> Result<(), LexError> {
    let char_table = CharTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "sss c;"#;
    assert! (
        if let Err (LexError::QuotationMisMatch {
            quotation_mark,
            index,
        }) = char_table.lex (input) {
            (quotation_mark == '"' &&
             index == 3)
        } else {
            false
        }
    );
    Ok (())
}

#[test]
fn test_escape () -> Result<(), LexError> {
    let char_table = CharTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "s\"" c;"#;
    let token_vec = char_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 0, hi: 2 },
        word: "aa",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 3, hi: 8 },
        quotation_mark: '"',
        string: r#"s\""#,
    });
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 9, hi: 10 },
        word: "c",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Char {
        span: Span { lo: 10, hi: 11 },
        ch: ';',
    });
    assert_eq! (iter.next (), None);
    Ok (())
}
