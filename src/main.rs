mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let line = parser::lexer::to_line(args);
    let tokens = parser::lexer::to_tokens(line);

    println!("{:?}", tokens);
}
