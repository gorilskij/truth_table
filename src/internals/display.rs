use crate::internals::expression::ExBox;
use crate::internals::var_values::VarValues;

pub fn display(original: &str, expression: ExBox, mut var_values: VarValues) {
    let mut name_lengths = vec![];
    for name in var_values.names() {
        name_lengths.push(name.len());
        print!("{} ", name)
    }
    print!(" #  {}", original);
    println!();

    loop {
        for (i, (space, value)) in name_lengths
            .iter()
            .zip(var_values.values())
            .enumerate() {
            if value { print!("T"); } else { print!("F"); }
            for _ in 0..*space { print!(" "); }
        }

        print!("    ");
        if expression.evaluate(&var_values) { print!("T"); } else { print!("F"); }
        println!();

        if !var_values.advance() { break }
    }
}