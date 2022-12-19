#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    NUMBER,
    LP,
    RP,
    END,
    EXPR,
    TERM,
    FACT,
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

// ?
impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            kind: self.kind,
            value: self.value,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: isize) -> Self {
        Self { kind, value }
    }
}
