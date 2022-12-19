extern crate alloc;

use alloc::collections::VecDeque;

use self::token::{Token, TokenKind};

mod token;

pub mod lexer;

fn make_parse_tree() {}

#[derive(Default, Clone, Copy)]
struct Action(&'static str, usize);
pub struct Parser {
    input_stream: VecDeque<Token>,
    state_stack: VecDeque<usize>,
    token_stack: VecDeque<Token>,
    nonterm: Option<Token>,
    actions: [[Action; 11]; 16],
}

impl Parser {
    fn new(tokens: VecDeque<Token>) -> Self {
        let mut parser = Parser {
            input_stream: tokens,
            state_stack: Default::default(),
            token_stack: Default::default(),
            nonterm: None,
            actions: Default::default(),
        };

        parser.state_stack.push_back(0);

        parser.actions[0][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[0][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[0][TokenKind::EXPR as usize] = Action("goto", 1);
        parser.actions[0][TokenKind::TERM as usize] = Action("goto", 2);
        parser.actions[0][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[1][TokenKind::END as usize] = Action("accept", 3);

        parser.actions[2][TokenKind::PLUS as usize] = Action("shift", 6);
        parser.actions[2][TokenKind::MINUS as usize] = Action("shift", 7);
        parser.actions[2][TokenKind::RP as usize] = Action("reduce", 3);
        parser.actions[2][TokenKind::END as usize] = Action("reduce", 3);

        parser.actions[3][TokenKind::PLUS as usize] = Action("reduce", 6);
        parser.actions[3][TokenKind::MINUS as usize] = Action("reduce", 6);
        parser.actions[3][TokenKind::MULTIPLY as usize] = Action("shift", 8);
        parser.actions[3][TokenKind::DIVIDE as usize] = Action("shift", 9);
        parser.actions[3][TokenKind::RP as usize] = Action("reduce", 6);
        parser.actions[3][TokenKind::END as usize] = Action("reduce", 6);

        parser.actions[4][TokenKind::PLUS as usize] = Action("reduce", 7);
        parser.actions[4][TokenKind::MINUS as usize] = Action("reduce", 7);
        parser.actions[4][TokenKind::MULTIPLY as usize] = Action("reduce", 7);
        parser.actions[4][TokenKind::DIVIDE as usize] = Action("reduce", 7);
        parser.actions[4][TokenKind::RP as usize] = Action("reduce", 7);
        parser.actions[4][TokenKind::END as usize] = Action("reduce", 7);

        parser.actions[5][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[5][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[5][TokenKind::EXPR as usize] = Action("goto", 10);
        parser.actions[5][TokenKind::TERM as usize] = Action("goto", 2);
        parser.actions[5][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[6][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[6][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[6][TokenKind::EXPR as usize] = Action("goto", 11);
        parser.actions[6][TokenKind::TERM as usize] = Action("goto", 2);
        parser.actions[6][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[7][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[7][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[7][TokenKind::EXPR as usize] = Action("goto", 12);
        parser.actions[7][TokenKind::TERM as usize] = Action("goto", 2);
        parser.actions[7][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[8][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[8][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[8][TokenKind::TERM as usize] = Action("goto", 13);
        parser.actions[8][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[9][TokenKind::NUMBER as usize] = Action("shift", 4);
        parser.actions[9][TokenKind::LP as usize] = Action("shift", 5);
        parser.actions[9][TokenKind::TERM as usize] = Action("goto", 14);
        parser.actions[9][TokenKind::FACT as usize] = Action("goto", 3);

        parser.actions[10][TokenKind::RP as usize] = Action("shift", 15);

        parser.actions[11][TokenKind::RP as usize] = Action("reduce", 1);
        parser.actions[11][TokenKind::END as usize] = Action("reduce", 1);

        parser.actions[12][TokenKind::RP as usize] = Action("reduce", 2);
        parser.actions[12][TokenKind::END as usize] = Action("reduce", 2);

        parser.actions[13][TokenKind::PLUS as usize] = Action("reduce", 4);
        parser.actions[13][TokenKind::MINUS as usize] = Action("reduce", 4);
        parser.actions[13][TokenKind::RP as usize] = Action("reduce", 4);
        parser.actions[13][TokenKind::END as usize] = Action("reduce", 4);

        parser.actions[14][TokenKind::PLUS as usize] = Action("reduce", 5);
        parser.actions[14][TokenKind::MINUS as usize] = Action("reduce", 5);
        parser.actions[14][TokenKind::RP as usize] = Action("reduce", 5);
        parser.actions[14][TokenKind::END as usize] = Action("reduce", 5);

        parser.actions[15][TokenKind::PLUS as usize] = Action("reduce", 8);
        parser.actions[15][TokenKind::MINUS as usize] = Action("reduce", 8);
        parser.actions[15][TokenKind::MULTIPLY as usize] = Action("reduce", 8);
        parser.actions[15][TokenKind::DIVIDE as usize] = Action("reduce", 8);
        parser.actions[15][TokenKind::RP as usize] = Action("reduce", 8);
        parser.actions[15][TokenKind::END as usize] = Action("reduce", 8);

        parser
    }

    fn run(&mut self) -> isize {
        loop {
            let state = self.get_state();
            let input = self.get_input();
            let Action(command, num) = self.actions[state][input.kind as usize];
            // println!("{command} {num} {state} {:?}", input);
            match command {
                "shift" => self.shift(num),
                "reduce" => self.reduce(num),
                "goto" => self.goto(num),
                "accept" => break,
                _ => panic!("invalid syntax"),
            }
        }
        match self.token_stack.pop_front() {
            Some(t) => t.value,
            None => panic!("invalid syntax"),
        }
    }

    fn get_input(&mut self) -> Token {
        let nonterm = self.nonterm.clone();
        match nonterm {
            Some(t) => t,
            None => match self.input_stream.front() {
                Some(t) => t.clone(),
                None => panic!("invalid syntax"),
            },
        }
    }

    fn get_state(&self) -> usize {
        let state = self.state_stack.back();
        match state {
            Some(i) => *i,
            None => panic!("invalid syntax"),
        }
    }

    fn shift(&mut self, new_state: usize) {
        self.state_stack.push_back(new_state);
        let pop = self.input_stream.pop_front();
        self.push_token(pop);
    }

    fn goto(&mut self, new_state: usize) {
        self.state_stack.push_back(new_state);
        self.push_token(self.nonterm.clone()); // ?
        self.nonterm = None;
    }

    fn push_token(&mut self, token: Option<Token>) {
        match token {
            Some(t) => self.token_stack.push_back(t),
            None => panic!("invalid syntax"),
        }
    }

    fn reduce(&mut self, num: usize) {
        match num {
            1 => self.reduce_1(),
            2 => self.reduce_2(),
            3 => self.reduce_3(),
            4 => self.reduce_4(),
            5 => self.reduce_5(),
            6 => self.reduce_6(),
            7 => self.reduce_7(),
            8 => self.reduce_8(),
            _ => panic!("invalid syntax"),
        }
    }

    fn reduce_1(&mut self) {
        let a = self.token_validation(TokenKind::EXPR);
        self.token_validation(TokenKind::PLUS);
        let c = self.token_validation(TokenKind::TERM);
        let t = Token::new(TokenKind::EXPR, c + a);
        self.nonterm = Some(t);
    }

    fn reduce_2(&mut self) {
        let a = self.token_validation(TokenKind::EXPR);
        self.token_validation(TokenKind::MINUS);
        let c = self.token_validation(TokenKind::TERM);
        let t = Token::new(TokenKind::EXPR, c - a);
        self.nonterm = Some(t);
    }

    fn reduce_3(&mut self) {
        let a = self.token_validation(TokenKind::TERM);
        let t = Token::new(TokenKind::EXPR, a);
        self.nonterm = Some(t);
    }

    fn reduce_4(&mut self) {
        let a = self.token_validation(TokenKind::TERM);
        self.token_validation(TokenKind::MULTIPLY);
        let c = self.token_validation(TokenKind::FACT);
        let t = Token::new(TokenKind::TERM, c * a);
        self.nonterm = Some(t);
    }

    fn reduce_5(&mut self) {
        let a = self.token_validation(TokenKind::TERM);
        self.token_validation(TokenKind::DIVIDE);
        let c = self.token_validation(TokenKind::FACT);
        if a == 0 {
            panic!("divided by 0");
        }
        let t = Token::new(TokenKind::TERM, c / a);
        self.nonterm = Some(t);
    }

    fn reduce_6(&mut self) {
        let a = self.token_validation(TokenKind::FACT);
        let t = Token::new(TokenKind::TERM, a);
        self.nonterm = Some(t);
    }

    fn reduce_7(&mut self) {
        let a = self.token_validation(TokenKind::NUMBER);
        let t = Token::new(TokenKind::FACT, a);
        self.nonterm = Some(t);
    }

    fn reduce_8(&mut self) {
        self.token_validation(TokenKind::LP);
        let b = self.token_validation(TokenKind::EXPR);
        self.token_validation(TokenKind::RP);
        let t = Token::new(TokenKind::FACT, b);
        self.nonterm = Some(t);
    }

    fn token_validation(&mut self, kind: TokenKind) -> isize {
        self.state_stack.pop_back();
        let pop = self.token_stack.pop_back();
        match pop {
            Some(t) => match t.kind {
                kind => t.value,
                _ => panic!("invalid syntax"),
            },
            None => panic!("invalid syntax"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_pst() {
        make_parse_tree();
    }

    fn test_parser(s: &str) -> isize {
        let tokens = lexer::to_tokens(s.to_string());
        let mut p = Parser::new(tokens);
        p.run()
    }

    #[test]
    fn test() {
        assert_eq!(test_parser("1 + 2 + 3"), 6);
        assert_eq!(test_parser("(1 + 2) * 3"), 9);
        assert_eq!(test_parser("4 * (1 + 2) * 3"), 36);
        assert_eq!(test_parser("((2))"), 2);
        assert_eq!(test_parser("((2) + 2)"), 4);
        assert_eq!(test_parser("2+ ((2) + 2)"), 6);
    }
}
