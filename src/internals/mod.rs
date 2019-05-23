mod var_values;
mod expression;
mod parse;
mod display;

pub fn display(expression: &str) {
    // TODO: check
    let (expr, var_val) = parse::parse(expression);
    display::display(expression, expr, var_val)
}