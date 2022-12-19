extern crate alloc;

use alloc::collections::VecDeque;

use super::token::*;

#[derive(Clone, Copy)]
enum LexingState {
    BLANK,
    METACHAR,
    NUMBER,
}

pub fn to_line(args: Vec<String>) -> String {
    let mut line = String::new();
    for i in 1..args.len() {
        if line.is_empty() {
            line = args[i].clone();
        } else {
            line = format!("{} {}", line, args[i]);
        }
    }
    line
}

struct Lexer {
    tokens: VecDeque<Token>,
    state: LexingState,
    number: String,
}

impl Lexer {
    fn action(&mut self, c: char) {
        match self.state {
            LexingState::BLANK => self.meta_blank_state(c),
            LexingState::METACHAR => self.meta_blank_state(c),
            LexingState::NUMBER => match c {
                '0'..='9' => self.push_number(c),
                '+' | '*' | '(' | ')' => self.number_token().meta_token(c).to_metachar(),
                ' ' | '\n' | '\0' => self.number_token().to_blank(),
                _ => panic!("invalid charactor"),
            },
        };
    }

    fn meta_blank_state(&mut self, c: char) -> &mut Self {
        match c {
            '0'..='9' => self.push_number(c).to_number(),
            '+' | '*' | '(' | ')' => self.meta_token(c).to_metachar(),
            ' ' | '\n' | '\0' => self.to_blank(),
            _ => panic!("invalid charactor"),
        };
        self
    }

    fn number_token(&mut self) -> &mut Self {
        let result = self.number.parse::<isize>();
        let kind = TokenKind::NUMBER;
        match result {
            Ok(num) => self.tokens.push_back(Token::new(kind, num)),
            Err(e) => panic!("invalid number: {e}"),
        }
        self.number.clear();
        self
    }

    fn push_number(&mut self, c: char) -> &mut Self {
        self.number.push(c);
        self
    }

    fn meta_token(&mut self, c: char) -> &mut Self {
        let kind = match c {
            '+' => TokenKind::PLUS,
            '*' => TokenKind::MULTIPLY,
            '(' => TokenKind::LP,
            ')' => TokenKind::RP,
            _ => panic!("invalid charactor"),
        };
        self.tokens.push_back(Token::new(kind, 0));
        self
    }

    fn to_blank(&mut self) -> &mut Self {
        self.state = LexingState::BLANK;
        self
    }

    fn to_number(&mut self) -> &mut Self {
        self.state = LexingState::NUMBER;
        self
    }

    fn to_metachar(&mut self) -> &mut Self {
        self.state = LexingState::METACHAR;
        self
    }
}

pub fn to_tokens(line: String) -> VecDeque<Token> {
    let mut lexer = Lexer {
        tokens: Default::default(),
        state: LexingState::BLANK,
        number: String::new(),
    };
    for c in line.chars() {
        lexer.action(c);
    }
    lexer.action('\0');

    lexer.tokens.push_back(Token::new(TokenKind::END, 0));
    lexer.tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_to_line() {}

    #[test] // "1  "
    fn test_to_tokens_blank() {
        let s = String::from("1  ");
        assert_eq!(
            to_tokens(s),
            vec![
                Token {
                    kind: TokenKind::NUMBER,
                    value: 1
                },
                Token {
                    kind: TokenKind::END,
                    value: 0
                },
            ]
        )
    }

    #[test] // "1 +2*3"
    fn test_to_tokens_general1() {
        let s = String::from("1 +2*3");
        assert_eq!(
            to_tokens(s),
            vec![
                Token {
                    kind: TokenKind::NUMBER,
                    value: 1
                },
                Token {
                    kind: TokenKind::PLUS,
                    value: 0
                },
                Token {
                    kind: TokenKind::NUMBER,
                    value: 2
                },
                Token {
                    kind: TokenKind::MULTIPLY,
                    value: 0
                },
                Token {
                    kind: TokenKind::NUMBER,
                    value: 3
                },
                Token {
                    kind: TokenKind::END,
                    value: 0
                },
            ]
        )
    }
    #[test] // "(+)"
    fn test_to_tokens_general2() {
        let s = String::from("(+)");
        assert_eq!(
            to_tokens(s),
            vec![
                Token {
                    kind: TokenKind::LP,
                    value: 0
                },
                Token {
                    kind: TokenKind::PLUS,
                    value: 0
                },
                Token {
                    kind: TokenKind::RP,
                    value: 0
                },
                Token {
                    kind: TokenKind::END,
                    value: 0
                },
            ]
        )
    }
    #[test] // "123"
    fn test_to_tokens_general3() {
        let s = String::from("123");
        assert_eq!(
            to_tokens(s),
            vec![
                Token {
                    kind: TokenKind::NUMBER,
                    value: 123
                },
                Token {
                    kind: TokenKind::END,
                    value: 0
                },
            ]
        )
    }

    #[test] // "+ -12a"
    #[should_panic]
    fn test_to_tokens_invalid_number() {
        let s = String::from("+ -12a");
        to_tokens(s);
    }
}
