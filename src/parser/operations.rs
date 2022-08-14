pub trait Add {
    fn action(&self) -> i64;
}

pub struct Operation {
    operand_a : i64,
    operand_b : i64,
}

impl Add for Operation {
    fn action(&self) -> i64 {
        self.operand_a + self.operand_b
    }
}

pub fn new(operand_a: i64, operand_b: i64) -> Operation {
    Operation { operand_a , operand_b }
}
