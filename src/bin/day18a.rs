use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Item {
    Operand(i64),
    Operator(char),
}

use Item::*;

fn push_operand(stack: &mut Vec<Item>, mut operand: i64) {
    loop {
        match stack.last().copied() {
            Some(Operator('(')) | None => {
                stack.push(Operand(operand));
                break;
            }
            Some(Operator(op)) => {
                stack.pop();
                if let Some(Operand(other)) = stack.pop() {
                    if op == '+' {
                        operand += other;
                    } else {
                        operand *= other;
                    }
                } else {
                    panic!("expect operand under operator")
                }
            }
            Some(Operand(_)) => panic!("pushing operand on operand"),
        }
    }
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day18.txt")?);
    let mut sum: i64 = 0;
    for line in reader.lines() {
        let line = line?;

        let mut stack: Vec<Item> = Vec::new();
        for ch in line.chars() {
            match ch {
                ' ' => {}
                '+' | '*' | '(' => stack.push(Operator(ch)),
                ')' => {
                    if let Some(Operand(operand)) = stack.pop() {
                        assert_eq!(stack.pop(), Some(Operator('(')));
                        push_operand(&mut stack, operand);
                    } else {
                        panic!("expect operand under closing bracket");
                    }
                }
                ch => {
                    if let Some(operand) = ch.to_digit(10) {
                        push_operand(&mut stack, operand as i64);
                    } else {
                        panic!("unknown symbol: {}", ch);
                    }
                }
            }
        }
        assert_eq!(stack.len(), 1);
        match stack[0] {
            Operand(result) => sum += result,
            Operator(op) => panic!("unexpected operator on final stack: {}", op),
        }
    }

    println!("The sum of all results is {}", sum);
    Ok(())
}
