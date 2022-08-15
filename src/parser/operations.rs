#[derive(Debug)]
pub enum Operand{
    Expr(Expression)
}


#[derive(Debug)]
pub struct Term {
    pub signed: bool,

    pub const_divisor: u128,
    pub const_divident: u128,

    pub divisor: Vec<Operand>,
    pub dividend: Vec<Operand>
}

// 2+2/5
// + and - separates an expression into terms
#[derive(Debug)]
pub struct Expression{
    pub terms: Vec<Term>
}
