use logos::Logos;
use std::str;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    //Defines variants and their token/regex.

    //Schluesselwoerter
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,

    //Operatoren
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    //Sonstige Token
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    //Termvariablen
    #[regex(r#"[0-9]+"#, priority = 2)]
    ConstInt,
    #[regex(r#"(\d+\.\d+)|(\.\d+)([eE]([-+])?[0-9]+)?"#, priority = 2)]
    ConstFloat,
    #[regex(r#"(true|false)"#)]
    ConstBoolean,
    #[regex(r#"\\"[^"\\n]*\\""#)]
    ConstString,
    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*")]
    Id,

    //"Pseudotoken" (nur zur Konstruktion anderer Token)
    #[regex("[0-9]")]
    Digit,
    #[token("[0-9]]+")]
    Integer,
    #[regex(r#"(\d+\.\d+)|(\.\d+)"#)]
    Float,
    #[regex("[a-zA-Z]", priority = 2)]
    Letter,


    //Comment skipping
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"//.*", logos::skip)]

    //Error handling
    #[error]
    Error,
}