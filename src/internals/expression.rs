use super::var_values::VarValues;

pub enum BinOpType { And, Or, Implies, Iff }

pub enum Expr {
    Var(String),
    BinOp(BinOpType, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

// logical methods
use fns::*;
mod fns {
    #![allow(dead_code)]
    pub fn and(l: bool, r: bool) -> bool { l && r }
    pub fn or(l: bool, r: bool) -> bool { l || r }
    pub fn not(_: bool, r: bool) -> bool { !r }
    pub fn implies(l: bool, r: bool) -> bool { !l || r }
    pub fn iff(l: bool, r: bool) -> bool { l == r }
}

impl Expr {
    pub fn evaluate(&self, var_values: &VarValues) -> bool {
        use Expr::*;
        use BinOpType::*;
        match self {
            Var(v) => var_values.get_value(v),
            Not(e) => !e.evaluate(var_values),
            BinOp(ty, left, right) => {
                let func = match ty {
                    And => and,
                    Or => or,
                    Implies => implies,
                    Iff => iff,
                };

                func(left.evaluate(var_values), right.evaluate(var_values))
            }
        }
    }
}