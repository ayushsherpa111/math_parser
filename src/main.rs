use std::io::stdin;

mod parser;
mod token;

fn main() {
    // main interpreter loop
    loop {
        print!("> ");
        let mut user_input = String::with_capacity(40);
        std::io::Write::flush(&mut std::io::stdout()).expect("failed to flush");
        stdin()
            .read_line(&mut user_input)
            .expect("Input not provided");
        let tokens = token::tokenize(user_input.trim());
        parser::parse(tokens);
    }
}
