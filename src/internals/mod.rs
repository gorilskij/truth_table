mod var_values;
mod expression;
mod check_validity;
mod parse;
mod display;

pub fn display(expression: &str) {
    // TODO: check
    check_validity::check_validity(expression);
    let (expr, var_val) = parse::parse(expression);
    display::display(expression, expr, var_val)
}