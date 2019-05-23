use std::process::exit;

enum Checking {
    Identifier, SpaceAfterIdentifier, Operator(char), ParenOpen, ParenClose, None
}

// use Checking::*; // (works but unrecognized by CLion)
use crate::internals::check_validity::Checking::{Identifier, SpaceAfterIdentifier, Operator, ParenOpen, ParenClose, None};

pub enum Status { Ok, Unexpected(usize, char), ExpectedAtEnd(String), Msg(String) }

// TODO: write decrement_paren_depth macro
pub fn check_validity(expr: &str) -> Status {
    if expr.is_empty() || expr.chars().all(|x| x == ' ') {
        return Status::Msg("empty expression".to_string())
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
                    _ => return Status::Unexpected(i, ch)
                }
            },
            Identifier => {
                match ch {
                    'a'..='z' | 'A'..='Z' => (),
                    '&' | '|' | '<' | '=' => last = Checking::Operator(ch),
                    ')' => {
                        if paren_depth == 0 { return Status::Unexpected(i, ch) }
                        paren_depth -= 1;
                        last = Checking::ParenClose
                    },
                    ' ' => last = SpaceAfterIdentifier,
                    _ => return Status::Unexpected(i, ch)
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
                        _ => return Status::Unexpected(i, ch)
                    },
                    '<' => match ch {
                        '=' => last = Operator(ch),
                        _ => return Status::Unexpected(i, ch)
                    },
                    '=' => match ch {
                        '>' => last = Operator(ch),
                        _ => return Status::Unexpected(i, ch)
                    },
                    _ => return Status::Unexpected(i, ch)
                }
            },
            SpaceAfterIdentifier | ParenClose => {
                match ch {
                    '&' | '|' | '<' | '=' => last = Operator(ch),
                    ')' => {
                        if paren_depth == 0 { return Status::Unexpected(i, ch) }
                        paren_depth -= 1;
                        last = ParenClose
                    },
                    ' ' => (),
                    _ => return Status::Unexpected(i, ch)
                }
            }
        }
    }

    if paren_depth != 0 {
        return Status::ExpectedAtEnd("')'".to_string())
    }

    match last {
        ParenOpen => panic!("this case (last ParenOpen) should have been caught earlier"),
        Operator(_) => Status::ExpectedAtEnd("identifier or '!'".to_string()),
        _ => Status::Ok
    }
}