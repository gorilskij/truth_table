use super::var_values::VarValues;
use crate::internals::expression::Expr;

pub fn display(original: &str, expression: &Expr, mut var_values: VarValues) {
    let mut name_lengths = vec![];
    for name in var_values.names() {
        name_lengths.push(name.len());
        print!("{} ", name)
    }
    print!(" —  {}", original);
    println!();

    let mut tautology = true;
    let mut contradiction = true;

    let mut even = true;

    loop {
        for (space, value) in name_lengths
            .iter()
            .zip(var_values.values()) {
            if value { print!("T"); } else { print!("F"); }
            for _ in 0..*space { print!(" "); }
        }

        if even { print!(" -  ") } else { print!(" —  ") };
        even = !even;

        if expression.evaluate(&var_values) {
            print!("T");
            contradiction = false;
        } else {
            print!("F");
            tautology = false;
        }
        println!();

        if !var_values.advance() { break }
    }

    if tautology && contradiction {
        panic!("tautology & contradiction") // just in case
    } else if tautology {
        println!("(Tautology)")
    } else if contradiction {
        println!("(Contradiction)")
    }
}