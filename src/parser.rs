mod operations;
use crate::token::Tokens;
use std::iter::Peekable;
use std::slice::Iter;

fn parse_expression(tokens: &mut Peekable<Iter<Tokens>>) -> operations::Expression {
    let mut expr_vec = vec![];

    #[repr(u8)]
    #[derive(Clone, Copy)]
    enum State {
        Adding = 0,
        Substracting = 1,
    }

    let mut state = State::Adding;
    while let Some(token) = tokens.peek() {
        match token {
            Tokens::Add => {
                state = State::Adding;
                tokens.next();
            }
            Tokens::Substract => {
                state = State::Substracting;
                tokens.next();
            }
            _ => {
                let mut term = parse_term(tokens);
                term.signed = state as u8 != 0;
                expr_vec.push(term);
            }
        }
    }
    operations::Expression{
       terms: expr_vec
    }
}

// 2+22/5
fn parse_term(tokens: &mut Peekable<Iter<Tokens>>) -> operations::Term {
    #[derive(Clone, Copy)]
    enum State {
        Mul,
        Div,
    }
    let mut state = State::Mul;
    let mut term = operations::Term{
        signed: false,
        const_divisor: 1,
        const_divident: 1,

        divisor: vec![],
        dividend: vec![],
    };
    while let Some(token) = tokens.peek() {
        match token {
            Tokens::Add | Tokens::Substract => break,
            Tokens::Multiply =>  {
                state = State::Mul;
                tokens.next();
            },
            Tokens::Divide => {
                state = State::Div;
                tokens.next();
            },
            Tokens::Number(val) => {
                match state {
                    State::Mul => term.const_divident *= val,
                    State::Div => term.const_divisor *= val,
                }
                tokens.next();
            },
            // _ => panic!("")
        }
    }
    term
}

pub fn parse(token: Vec<Tokens>) {
    let mut iter_token = token.iter().peekable();
    let expression = parse_expression(&mut iter_token);
    println!("{expression:?}")
}
