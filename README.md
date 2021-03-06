# Lexing

[![Build Status](https://travis-ci.org/parsing-tech/lexing-rs.svg?branch=master)](https://travis-ci.org/parsing-tech/lexing-rs)

## Welcome

A lexer in rust, which handles doublequoted string.

- zero copy
- no DSL, just rust code
  - for lexer is much more simple than parser
- to serve as the fontend of parsing-tech

## Example

### abc

``` rust
use lexing::{
    CharTable, Token, Span, LexError,
};

fn main () -> Result<(), LexError> {
    let char_table = CharTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"a "b" c;"#;
    let token_vec = char_table.lex (input)?;
    println! ("- token_vec = {:#?}", token_vec);
    Ok (())
}
```

output :

``` rust
- token_vec = [
    Word {
        span: Span {
            lo: 0,
            hi: 1
        },
        word: "a"
    },
    Quotation {
        span: Span {
            lo: 2,
            hi: 5
        },
        quotation_mark: '\"',
        string: "b"
    },
    Word {
        span: Span {
            lo: 6,
            hi: 7
        },
        word: "c"
    },
    Char {
        span: Span {
            lo: 7,
            hi: 8
        },
        ch: ';'
    }
]
```

## Docs

- [API docs](https://parsing-tech.github.io/lexing-rs/api/lexing)

## Contributing

We use Collective Code Construction Contract -- a.k.a. C4, as our collaboration protocol.

- [The C4 RFC](https://rfc.zeromq.org/spec:42/C4)
- [Our Style Guide](STYLE-GUIDE.md)

To highlight some features of C4 :

```
- Everyone, without distinction or discrimination,
  SHALL have an equal right to become a Contributor under the terms of this contract.

- Change on the project SHALL be governed by the pattern of
  accurately identifying problems
  and applying minimal, accurate solutions to these problems.
```

## CODE OF CONDUCT

[Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## LICENSE

[GPLv3](LICENSE)
