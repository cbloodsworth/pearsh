use std::io;

mod lexer;
pub use lexer::tokenize;

fn print_lex_results(input: String) {
    lexer::tokenize(input)
        .iter()
        .for_each(|x| {
            let lexeme = format!("[{}]", x.lexeme);
            print!("{0: <10}: ", lexeme);
            println!("{:?}", x.kind);
        });
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        print_lex_results(input);

        println!();
    }
}

