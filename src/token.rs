mod tokens;
pub use tokens::Tokens;

pub fn tokenize(user_string: &str) -> Vec<Tokens> {
    let mut token_vector = Vec::new();
    let mut acc_str = String::new();
    user_string
        .chars()
        .enumerate()
        .for_each(|(idx, curr)| match curr {
            '+' | '-' | '/' | '*' => {
                if acc_str.len() > 0 {
                    token_vector.push(Tokens::Number(
                        acc_str.parse::<u128>().expect("Failed to parse number"),
                    ));
                    acc_str.clear();
                }
                match curr {
                    '-' => token_vector.push(Tokens::Substract),
                    '+' => token_vector.push(Tokens::Add),
                    '/' => token_vector.push(Tokens::Divide),
                    '*' => token_vector.push(Tokens::Multiply),
                    _ => panic!("Invalid operation found at index {idx}"),
                }
            }
            _ => {
                acc_str.push(curr);
            }
        });
    if acc_str.len() > 0 {
        token_vector.push(Tokens::Number(
            acc_str.parse::<u128>().expect("Failed to parse number"),
        ));
    }
    token_vector
}
