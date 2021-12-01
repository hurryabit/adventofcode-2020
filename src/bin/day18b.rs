use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

/*
Expr ::= Sum | Sum '*' Expr
Sum ::= Atom | Atom '+' Sum
Atom ::= '0' | ... | '9' | '(' Expr ')'
*/

fn expr(input: &str) -> (i64, &str) {
    let (x, input) = sum(input);
    if let Some(input) = input.strip_prefix(" * ") {
        let (y, input) = expr(input);
        (x * y, input)
    } else {
        (x, input)
    }
}

fn sum(input: &str) -> (i64, &str) {
    let (x, input) = atom(input);
    if let Some(input) = input.strip_prefix(" + ") {
        let (y, input) = sum(input);
        (x + y, input)
    } else {
        (x, input)
    }
}

fn atom(input: &str) -> (i64, &str) {
    if let Some(ch) = input.chars().next() {
        if let Some(digit) = ch.to_digit(10) {
            (digit as i64, &input[1..])
        } else if ch == '(' {
            let (x, input) = expr(&input[1..]);
            if let Some(input) = input.strip_prefix(')') {
                (x, input)
            } else if let Some(ch) = input.chars().next() {
                panic!("expected ')', found '{}'", ch);
            } else {
                panic!("expected ')', found eof");
            }
        } else {
            panic!("expected digit or '(', found '{}'", ch);
        }
    } else {
        panic!("expected digit or '(', found eof");
    }
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day18.txt")?);
    let mut sum: i64 = 0;
    for line in reader.lines() {
        let line = line?;
        let (result, rest) = expr(&line);
        assert!(rest.is_empty());
        sum += result;
    }

    println!("The sum of all results is {}", sum);
    Ok(())
}
