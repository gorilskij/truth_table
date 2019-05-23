enum Checking {
    Identifier, SpaceAfterIdentifier, Operator(char), ParenOpen, ParenClose, None
}

// use Checking::*; // <- CLion doesn't recognize
use crate::internals::check_validity::Checking::{Identifier, SpaceAfterIdentifier, Operator, ParenOpen, ParenClose, None};
use std::process::exit;

// TODO: convert to Result (Ok() | Err())
// TODO: implement 'expected' messages
// TODO: implement index and arrow
fn exit_unexpected(i: usize, ch: char, expr: &str) -> ! {
    let init_msg = format!("unexpected character '{}' in \"", ch);
    let init_spaces = init_msg.len() + i;

    println!("{}{}\" at index {}", init_msg, expr, i);

    for _ in 0..init_spaces { print!(" ") }
    println!("^");

    exit(0)
}

fn exit_with_message(msg: &str) -> ! {
    println!("{}", msg);
    exit(0)
}

fn decrement_paren_depth(index: usize, depth: &mut usize, from: &str) {
    if *depth == 0 { exit_unexpected(index, ')', from) }
    *depth -= 1
}

pub fn check_validity(expr: &str) {
    if expr.is_empty() || expr.chars().all(|x| x == ' ') {
        exit_with_message("empty expression")
    }

    let mut last = Checking::None;
    let mut paren_depth: usize = 0;

    for (i, ch) in expr.chars().enumerate() {
        match last {
            None | ParenOpen => {
                match ch {
                    'a'..='z' | 'A'..='Z' => last = Identifier,
                    '!' => last = Operator(ch),
                    '(' => {
                        paren_depth += 1;
                        last = ParenOpen
                    },
                    ' ' => (),
                    _ => exit_unexpected(i, ch, expr)
                }
            },
            Identifier => {
                match ch {
                    'a'..='z' | 'A'..='Z' => (),
                    '&' | '|' | '<' | '=' => last = Checking::Operator(ch),
                    ')' => {
                        if paren_depth == 0 { exit_unexpected(i, ch, expr) }
                        paren_depth -= 1;
                        last = Checking::ParenClose
                    },
                    ' ' => last = SpaceAfterIdentifier,
                    _ => exit_unexpected(i, ch, expr)
                }
            },
            Operator(last_ch) => {
                match last_ch {
                    '!' | '&' | '|' | '>' => match ch {
                        'a'..='z' | 'A'..='Z' => last = Identifier,
                        '!' => last = Operator(ch),
                        '(' => {
                            paren_depth += 1;
                            last = ParenOpen
                        },
                        ' ' => (),
                        _ => exit_unexpected(i, ch, expr)
                    },
                    '<' => match ch {
                        '=' => last = Operator(ch),
                        _ => exit_unexpected(i, ch, expr)
                    },
                    '=' => match ch {
                        '>' => last = Operator(ch),
                        _ => exit_unexpected(i, ch, expr)
                    },
                    _ => exit_unexpected(i, ch, expr)
                }
            },
            SpaceAfterIdentifier | ParenClose => {
                match ch {
                    '&' | '|' | '<' | '=' => last = Operator(ch),
                    ')' => decrement_paren_depth(i, &mut paren_depth, expr),
                    ' ' => (),
                    _ => exit_unexpected(i, ch, expr)
                }
            }
        }
    }

    if paren_depth != 0 {
        exit_with_message("expected ')' at the end")
    }

    match last {
        ParenOpen => panic!("this case (last ParenOpen) should have been caught earlier"),
        Operator(_) => exit_with_message("expected identifier or expression at the end"),
        _ => ()
    }

    // TODO: conclusive match to ensure that parens are matched and final value is valid (not &)
}