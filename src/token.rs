#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    LParen, // (
    RParen, // )
    EOF,
}
