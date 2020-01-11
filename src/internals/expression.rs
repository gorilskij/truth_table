use super::var_values::VarValues;

pub enum Expression {
    Var(String),
    #[allow(dead_code)] // Op is only used through macro expansion
    Op {
        func: fn(bool, bool) -> bool,
        left: Option<Box<Expression>>,
        right: Box<Expression>,
    },

}

impl Expression {
    pub fn evaluate(&self, values: &VarValues) -> bool {
        use Expression::*;
        match self {
            Var(v) => values.get_value(v),
            Op { func, left, right } => {
                let left = match left.as_ref() {
                    None => false,
                    Some(ex) => ex.evaluate(values)
                };
                func(left, right.evaluate(values))
            }
        }
    }
}


// logical methods
fn and(l: bool, r: bool) -> bool { l && r }
fn or(l: bool, r: bool) -> bool { l || r }
fn not(_: bool, r: bool) -> bool { !r }
fn implies(l: bool, r: bool) -> bool { !l || r }
fn iff(l: bool, r: bool) -> bool { l == r }

macro_rules! expr_logic_method {
    ($name:ident, $func:expr) => {
        pub fn $name(right: Self) -> Self {
            Self::Op { func: $func, left: None, right: Box::new(right) }
        }
    };
    ($name:ident, $func:expr, bin) => {
        pub fn $name(left: Self, right: Self) -> Self {
            Self::Op { func: $func, left: Some(Box::new(left)), right: Box::new(right) }
        }
    };
}

impl Expression {
    expr_logic_method!(new_and, and, bin);
    expr_logic_method!(new_or, or, bin);
    expr_logic_method!(new_not, not);
    expr_logic_method!(new_implies, implies, bin);
    expr_logic_method!(new_iff, iff, bin);
}