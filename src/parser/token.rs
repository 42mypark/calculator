#[derive(PartialEq, Debug)]
pub enum TokenKind {
    NUMBER,
    PLUS,
    MULTIPLY,
    LP,
    RP,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: isize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: isize) -> Self {
        Self { kind, value }
    }
}
