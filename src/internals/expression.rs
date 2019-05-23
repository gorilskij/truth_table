use super::var_values::VarValues;
use std::ops::Deref;

pub trait Expression {
    fn evaluate(&self, values: &VarValues) -> bool;
    fn enbox(self) -> ExBox;
}

pub struct ExBox(Box<Expression>);
impl Deref for ExBox {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

// Implementations:
pub struct Var(String);
impl Var {
    pub fn new(name: &String) -> Self {
        Self(name.clone())
    }
}
impl Expression for Var {
    fn evaluate(&self, values: &VarValues) -> bool {
        values.get_value(&self.0)
    }

    fn enbox(self) -> ExBox {
        ExBox(Box::new(self))
    }
}

pub struct Op {
    func: &'static fn(bool, bool) -> bool,
    left: Option<ExBox>,
    right: ExBox
}
impl Op {
    pub fn new_and(left: ExBox, right: ExBox) -> ExBox { Op { func: AND, left: Some(left), right } .enbox() }
    pub fn new_or(left: ExBox, right: ExBox) -> ExBox { Op { func: OR, left: Some(left), right } .enbox() }
    pub fn new_not(right: ExBox) -> ExBox { Op { func: NOT, left: None, right } .enbox() }
    pub fn new_implies(left: ExBox, right: ExBox) -> ExBox { Op { func: IMPLIES, left: Some(left), right } .enbox()}
    pub fn new_iff(left: ExBox, right: ExBox) -> ExBox { Op { func: IFF, left: Some(left), right } .enbox()}
}
impl Expression for Op {
    fn evaluate(&self, values: &VarValues) -> bool {
        let left = match self.left.as_ref() {
            None => false,
            Some(ex) => ex.evaluate(values)
        };
        (self.func)(left, self.right.evaluate(values))
    }

    fn enbox(self) -> ExBox {
        ExBox(Box::new(self))
    }
}

fn and_func(x: bool, y: bool) -> bool { x && y }
fn or_func(x: bool, y: bool) -> bool { x || y }
fn not_func(_: bool, x: bool) -> bool { !x }
fn implies_func(x: bool, y: bool) -> bool { !x || y }
fn iff_func(x: bool, y: bool) -> bool { x == y }
static AND: &fn(bool, bool) -> bool = &(and_func as fn(bool, bool) -> bool);
static OR: &fn(bool, bool) -> bool = &(or_func as fn(bool, bool) -> bool);
static NOT: &fn(bool, bool) -> bool = &(not_func as fn(bool, bool) -> bool);
static IMPLIES: &fn(bool, bool) -> bool = &(implies_func as fn(bool, bool) -> bool);
static IFF: &fn(bool, bool) -> bool = &(iff_func as fn(bool, bool) -> bool);