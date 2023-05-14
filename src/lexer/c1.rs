use logos::Logos;
use std::fs;
use std::str;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    //Defines variants and their token/regex.

    //Schluesselwoerter
    #[token("bool")]
    KW_BOOLEAN,
    #[token("do")]
    KW_DO,
    #[token("else")]
    KW_ELSE,
    #[token("float")]
    KW_FLOAT,
    #[token("for")]
    KW_FOR,
    #[token("if")]
    KW_IF,
    #[token("int")]
    KW_INT,
    #[token("printf")]
    KW_PRINTF,
    #[token("return")]
    KW_RETURN,
    #[token("void")]
    KW_VOID,
    #[token("while")]
    KW_WHILE,

    //Operatoren
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("*")]
    ASTERISK,
    #[token("/")]
    SLASH,
    #[token("=")]
    ASSIGN,
    #[token("==")]
    EQ,
    #[token("!=")]
    NEQ,
    #[token("<")]
    LSS,
    #[token(">")]
    GRT,
    #[token("<=")]
    LEQ,
    #[token(">=")]
    GEQ,
    #[token("&&")]
    AND,
    #[token("||")]
    OR,

    //Sonstige Token
    #[token(",")]
    COMMA,
    #[token(";")]
    SEMICOLON,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,

    //Termvariablen
    #[token("{INTEGER}")]
    CONST_INT,
    #[token("{FLOAT} ( [eE] ([-+])? {INTEGER} )?\
        | {INTEGER} [eE] ([-+])? {INTEGER}")]
    CONST_FLOAT,
    #[regex(r#"(true|false)"#)]
    CONST_BOOLEAN,
    #[regex(r#"\\"[^"\\n]*\\""#)]
    CONST_STRING,
    #[token("({LETTER})+ ({DIGIT} | {LETTER})*")]
    ID,

    //"Pseudotoken" (nur zur Konstruktion anderer Token)
    #[regex("[0-9]")]
    DIGIT,
    #[token("{DIGIT}+")]
    INTEGER,
    #[regex(r#"(\d+\.\d+)|(\.\d+)"#)]
    FLOAT,
    #[regex("[a-zA-Z]")]
    LETTER,


    // Error handling and comment skipping
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"//.*", logos::skip)]
    Error,
}