mod tokens;
use tokens::Tokens;

pub fn tokenize(user_string: &str) -> Vec<Tokens> {
    let mut token_vector = Vec::new();
    user_string.chars().enumerate().for_each(|(_idx, curr)| {
        match curr {
            '+' => token_vector.push(Tokens::Add),
            '-' => token_vector.push(Tokens::Substract),
            '/' => token_vector.push(Tokens::Divide),
            '*' => token_vector.push(Tokens::Multiply),
            _ => token_vector.push(Tokens::Number)
        }
    });
    token_vector
}
