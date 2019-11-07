extern crate pure;

use pure::scan::Lexeme;

fn main() {
    let tokens = pure::scan::tokenize("3.14");
    let lexemes: Vec<&Lexeme> = tokens
        .iter()
        .map(|token| token.lexeme())
        .collect();

    println!("{:?}", lexemes)
}