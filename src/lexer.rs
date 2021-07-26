use logos::Logos;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
pub struct LexicalError;

#[derive(Logos, Clone, Debug)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("!")]
    Not,

    #[token("=>")]
    Implies,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next().map(|tok| {
            let span = self.lexer.span();
            Ok((span.start, tok, span.end))
        })
    }
}
