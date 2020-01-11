mod var_values;
mod expression;
mod check_validity;
mod parse;
mod display;
mod simplify;

use check_validity::{check_validity, Status}; // long import only for CLion

pub fn display(expression: &str) {
    use Status::*;
    match check_validity(expression) {
        Ok => (),
        Unexpected(i, ch) => {
            let init_msg = format!("unexpected '{}' in \"", ch);
            let init_spaces = init_msg.len() + i;

            println!("{}{}\" at index {}", init_msg, expression, i);

            for _ in 0..init_spaces { print!(" ") }
            println!("^");
            return
        },
        ExpectedAtEnd(s) => {
            let init_msg = format!("expected {} at end of \"{}", s, expression);

            println!("{}\"", init_msg);

            for _ in 0..init_msg.len() { print!(" "); }
            println!("^");
            return
        },
        Msg(s) => {
            println!("{}", s);
            return
        }
    }

    let (expr, var_val) = parse::parse(expression);
    display::display(expression, &expr, var_val)
}