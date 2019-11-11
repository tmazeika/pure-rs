extern crate pure;

fn main() {
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => handle_line(&line),
            Err(error) => panic!("{}", error),
        };
    }
}

fn handle_line(line: &str) {
    use pure::scan::*;

    let tokens = tokenize(line);
    let lexemes: Vec<&Lexeme> = tokens
        .iter()
        .map(|token| token.lexeme())
        .collect();

    println!("{:?}", lexemes)
}