//#![allow(unused)]

#[derive(Debug, Eq, PartialEq, Hash )]
pub enum Token {

    // Boundary Tokens
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]

    // Operator Tokens
    Tick,       // `
    Tilde,      // ~
    Bang,       // !
    At,         // @
    Hash,       // #
    Dollar,     // $
    Percent,    // %
    Caret,      // ^
    Ampersand,  // &
    Asterisk,   // *
    Dash,       // -
    Underscore, // _
    Equal,      // =
    Plus,       // +
    BSlash,     // \
    Vertical,   // |
    Semicolon,  // ;
    Colon,      // :
    Comma,      // ,
    Left,       // <
    Dot,        // .
    Right,      // >
    FSlash,     // /
    Query,      // ?

    Atom(usize),
    String(usize),

    // End of File
    Eof,

    Valid,
    Uc(char),
}
