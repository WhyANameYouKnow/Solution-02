use logos::Logos;
use std::str;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    //Defines variants and their token/regex.

    //Schluesselwoerter
    #[token("bool", priority = 5)]
    KwBoolean,
    #[token("do", priority = 3)]
    KwDo,
    #[token("else", priority = 5)]
    KwElse,
    #[token("float", priority = 5)]
    KwFloat,
    #[token("for", priority = 3)]
    KwFor,
    #[token("if", priority = 5)]
    KwIf,
    #[token("int", priority = 3)]
    KwInt,
    #[token("printf", priority = 3)]
    KwPrintf,
    #[token("return", priority = 5)]
    KwReturn,
    #[token("void", priority = 3)]
    KwVoid,
    #[token("while", priority = 3)]
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

    //"Pseudotoken" (nur zur Konstruktion anderer Token)
    #[regex("[0-9]", priority = 2)]
    Digit,
    #[regex("[0-9]+", priority = 4)]
    Integer,
<<<<<<< HEAD
    #[regex(r#"(\d+\.\d+)|(\.\d+)"#, priority = 1)]
=======
    #[regex(r#"(\d+\.\d+)|(\.\d+)"#, priority = 2)]
>>>>>>> 6863b0344ac80cadc46440c175c88388b97cd14e
    Float,
    #[regex("[a-zA-Z]", priority = 3)]
    Letter,

    //Termvariablen
<<<<<<< HEAD
    #[regex(r#"(\d+\.\d+)|(\.\d+)([eE]([-+])?[0-9]+)?"#, priority = 3)]
=======
    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*", priority = 2)]
    Id,
    #[regex(r#"[0-9]+"#, priority = 3)]
    ConstInt,
    #[regex(r#"(\d+\.\d+)|(\.\d+)([eE]([-+])?[0-9]+)?"#, priority = 4)]
>>>>>>> 6863b0344ac80cadc46440c175c88388b97cd14e
    ConstFloat,
    #[regex("[0-9]+", priority = 5)]
    ConstInt,
    #[regex(r#"(true|false)"#)]
    ConstBoolean,
    #[regex(r#"\\"[^"\\n]*\\""#)]
    ConstString,
    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*", priority = 4)]
    Id,

    //Comment skipping
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"//.*", logos::skip)]

    //Error handling
    #[error]
    Error,
}