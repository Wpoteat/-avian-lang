pub struct Token{
    enum TokenKind{
        Number(i64),
        Plus,
        Minus,
        Asterisk,
        Slash,
        LeftParen,
        RightParen,

    }
    start: usize,
    end: usize;

}
impl Token{
    fn new(kind: TokenKind, span: TextSpan, literal: String) -> Token{
        Token{kind, span, literal}
    }
}
pub fn tokenizer() -> Vec<Token>{
    vec![]
}
