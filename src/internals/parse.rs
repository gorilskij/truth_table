use crate::internals::expression::Expr;
use std::ops::RangeBounds;
use std::collections::Bound;
use crate::internals::var_values::VarValues;

fn split(from: &str) -> Vec<String> {
    let mut parts = vec![];
    let mut part = String::new();
    let mut last: Option<char> = None;

    for ch in from.chars() {
        match ch {
            ' ' => (),
            'a'..='z' | 'A'..='Z' => {
                if let Some(l) = last {
                    if !l.is_alphabetic() && !part.is_empty() {
                        parts.push(part);
                        part = String::new();
                    }
                }
                part.push(ch)
            },
            '&' | '|' | '!' | '(' | ')' => {
                if !part.is_empty() {
                    parts.push(part);
                    part = String::new()
                }
                let mut p = String::new();
                p.push(ch);
                parts.push(p)
            },
            '<' => {
                if !part.is_empty() {
                    parts.push(part);
                    part = String::new()
                }
                part.push(ch)
            },
            '=' => {
                if last.unwrap() != '<' && !part.is_empty() {
                    parts.push(part);
                    part = String::new();
                }
                part.push(ch)
            },
            '>' => {
                part.push(ch);
                parts.push(part);
                part = String::new()
            },
            _ => panic!("unexpected character '{}'", ch)
        }
        last = Some(ch)
    }

    if !part.is_empty() { parts.push(part) }

    parts
}

fn extract_names(parts: &[String]) -> Vec<String> {
    let mut vars = vec![];
    for part in parts {
        if part.chars().next().unwrap().is_alphabetic() {
            vars.push(part.clone())
        }
    }
    vars
}

enum Parsing {
    String(String),
    SubList(Vec<Parsing>)
}

impl Parsing {
    // only checks equality for two Parsing::String types, otherwise false
    fn string_eq(&self, other: &str) -> bool {
        if let Parsing::String(s0) = self {
            s0.as_str() == other
        } else { false }
    }
}

fn downgrade<R>(range: R, parsing: &mut Vec<Parsing>) where R: RangeBounds<usize> {
    let index = match range.start_bound() {
        Bound::Included(t) => *t,
        Bound::Unbounded => 0,
        _ => unreachable!("start index shouldn't be Bound::Excluded")
    };

    let sub: Vec<Parsing> = parsing.drain(range).collect();
    parsing.insert(index, Parsing::SubList(sub))
}

fn downgrade_braces(parsing: &mut Vec<Parsing>) {
    for sub in parsing.iter_mut() {
        if let Parsing::SubList(l) = sub {
            downgrade_braces(l)
        }
    }

    loop {
        // find innermost '(...)'
        let mut maybe_open = None;
        let mut maybe_close = None;
        for (i, sub) in parsing.iter().enumerate() {
            if sub.string_eq("(") {
                maybe_open = Some(i)
            } else if sub.string_eq(")") {
                maybe_close = Some(i);
                break
            }
        }

        if maybe_open.is_none() { break }

        let (first, last) = (maybe_open.unwrap(), maybe_close.unwrap());
        downgrade(first+1..last, parsing);
        parsing.remove(first);
        parsing.remove(first + 1);
    }
}

fn downgrade_not(parsing: &mut Vec<Parsing>) {
    for sub in parsing.iter_mut() {
        if let Parsing::SubList(l) = sub {
            downgrade_not(l)
        }
    }

    loop {
        let maybe_index = parsing
            .iter()
            .position(|x| x.string_eq("!"));

        if let Some(index) = maybe_index {
            let mut obliterated = false;
            if let Parsing::String(s) = &parsing[index + 1] {
                if s == "!" {
                    // obliterate [!, !]
                    parsing.remove(index);
                    parsing.remove(index);
                    obliterated = true
                }
            }

            if !obliterated { downgrade(index..index+2, parsing) }
        } else { break }
    }
}

fn downgrade_infix(op: &str, parsing: &mut Vec<Parsing>) {
    for sub in parsing.iter_mut() {
        if let Parsing::SubList(l) = sub {
            downgrade_infix(op, l)
        }
    }

    loop {
        let maybe_index = parsing
            .iter()
            .position(|x| x.string_eq(op));

        if let Some(index) = maybe_index {
            downgrade(index-1..index+2, parsing)
        } else { break }
    }
}

fn to_expression(parsing: &Parsing) -> Expr {
    match parsing {
        Parsing::String(s) => Expr::Var(s.clone()),
        Parsing::SubList(l) => {
            match l.len() {
                1 => to_expression(&l[0]),
                2 => Expr::Not(Box::new(to_expression(&l[1]))),
                3 => {
                    if let Parsing::String(s) = &l[1] {
                        let left = Box::new(to_expression(&l[0]));
                        let right = Box::new(to_expression(&l[2]));

                        use super::expression::BinOpType::*;
                        match s.as_str() {
                            "&" => Expr::BinOp(And, left, right),
                            "|" => Expr::BinOp(Or, left, right),
                            "=>" => Expr::BinOp(Implies, left, right),
                            "<=>" => Expr::BinOp(Iff, left, right),
                            _ => panic!("unexpected op '{}'", s)
                        }
                    } else {
                        unreachable!("invalid Parsing::SubList in the middle of a 3-long block")
                    }
                },
                n => unreachable!("invalid length of block: {}", n)
            }
        }
    }
}

pub fn parse(from: &str) -> (Expr, VarValues){
    let parts = split(from);

    let var_values = VarValues::new(&extract_names(&parts));

    let mut parsing: Vec<Parsing> = parts
        .into_iter()
        .map(Parsing::String)
        .collect();

    downgrade_braces(&mut parsing);
    downgrade_not(&mut parsing);
    downgrade_infix("&", &mut parsing);
    downgrade_infix("|", &mut parsing);
    downgrade_infix("=>", &mut parsing);
    downgrade_infix("<=>", &mut parsing);

    let expression = to_expression(&Parsing::SubList(parsing));

    (expression, var_values)
}

// for debug printing
//fn print_parsing(parsing: &Vec<Parsing>) {
//    print_parsing_(parsing);
//    println!()
//}
//
//fn print_parsing_(parsing: &Vec<Parsing>) {
//    print!("[");
//    for (i, sub) in parsing.iter().enumerate() {
//        match sub {
//            Parsing::String(s) => print!("{}", s),
//            Parsing::SubList(l) => {
//                print_parsing_(l)
//            }
//        }
//        if i < parsing.len() - 1 { print!(", ") }
//    }
//    print!("]");
//}