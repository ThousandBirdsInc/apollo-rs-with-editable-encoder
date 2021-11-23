/// Tokens generated by the lexer.
///
/// TokenKinds are [Lexical Tokens](https://spec.graphql.org/October2021/#sec-Appendix-Grammar-Summary.Lexical-Tokens) outlined in the GraphQL specification.
///
/// Punctuator
///   # ! $ & ... , : = @ ( ) [ ] { } |
/// Name
/// IntValue
/// FloatValue
/// StringValue
///
/// TokenKinds can be accessed by a convenience macro, `T!`. For example to
/// access the Bang TokenKind, you may match with `TokenKind::Bang`, or use the
/// macro `T![!]`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum TokenKind {
    Whitespace, // \r | \n |   | \t
    Comment,    // # comment
    Bang,       // !
    Dollar,     // $
    Amp,        // &
    Spread,     // ...
    Comma,      // ,
    Colon,      // :
    Eq,         // =
    At,         // @
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    LCurly,     // {
    RCurly,     // }
    Pipe,       // |
    Eof,

    // composite nodes
    Name,
    StringValue,
    Int,
    Float,
}

#[macro_export]
macro_rules! T {
    [!] => { $ crate :: TokenKind :: Bang } ;
    [$] => { $ crate :: TokenKind :: Dollar } ;
    [&] => { $ crate :: TokenKind :: Amp } ;
    [...] => { $ crate :: TokenKind :: Spread } ;
    [,] => { $ crate :: TokenKind :: Comma } ;
    [:] => { $ crate :: TokenKind :: Colon } ;
    [=] => { $ crate :: TokenKind :: Eq } ;
    [@] => { $ crate :: TokenKind :: At } ;
    ['('] => { $ crate :: TokenKind :: LParen } ;
    [')'] => { $ crate :: TokenKind :: RParen } ;
    ['['] => { $ crate :: TokenKind :: LBracket } ;
    [']'] => { $ crate :: TokenKind :: RBracket } ;
    ['{'] => { $ crate :: TokenKind :: LCurly } ;
    ['}'] => { $ crate :: TokenKind :: RCurly } ;
    [|] => { $ crate :: TokenKind :: Pipe } ;

    // composite nodes
    [name] => { $ crate :: TokenKind :: Name } ;
    [string] => { $ crate :: TokenKind :: StringValue} ;
    [int] => { $ crate :: TokenKind :: Int} ;
    [float] => { $ crate :: TokenKind :: Float} ;
}
