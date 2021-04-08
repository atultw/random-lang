// use std::num::ParseIntError;

// use core::panicking::{panic_fmt, panic_str};

use std::process::exit;
use std::thread::current;

fn main() {
    let program = "2 * 5 - 10 + 4 / 2";

    compile(program.parse().unwrap())
}

#[derive(Debug)]
enum Val {
    Given(u8),
    Expression(Box<Expr>),
}

impl Val {
    fn value(self) -> u8 {
        return match self {
            Val::Given(_u) => _u,
            Val::Expression(_u) => _u.eval()
        };
    }
}

#[derive(Debug)]
struct Expr {
    object: Val,
    operator: String,
    subject: u8,
}


impl Expr {
    fn eval(self) -> u8 {
        return match self.operator.as_str() {
            "+" => self.object.value() + self.subject,
            "-" => self.object.value() - self.subject,
            "*" => self.object.value() * self.subject,
            "/" => self.object.value() / self.subject,
            _ => { 0 }
        };
    }
}

fn compile(input: String) {
    let reserved: Vec<&str> = vec!["+", "-", "*", "/", "=", "!", "|"];

    let stuff: Vec<&str> = input.split(" ").collect();

    let mut i = 0;
    while i < stuff.len() {
        if i % 2 == 0 && reserved.contains(&stuff[i]) {
            println!("Reserved Word {:?} at pos {:?}", stuff[i], i);
            exit(0);
        }

        if i % 2 == 0 {
            match stuff[i].parse::<u8>() {
                Ok(_) => { println!("Number found"); }
                Err(_) => {
                    println!("Non-integer operand {:?} at pos {:?}", stuff[i], i);
                    exit(0);
                }
                // _ => {}
            };
        }

        if i % 2 == 1 && !reserved.contains(&stuff[i]) {
            println!("Word {:?} at pos {:?} is not an operator", stuff[i], i);
            exit(0);
        }
        i = i + 1;
    }

    let first = Expr {
        object: Val::Given(String::from(stuff[0]).parse().unwrap()),
        operator: stuff[1].parse().unwrap(),
        subject: stuff[2].parse().unwrap(),
    };

    // let mut i = 0;
    fn to_tree(all: Vec<&str>, prev: Expr, index: usize) -> u8 {
        if index >= all.len() as usize {
            // done
            return prev.eval()
        }

        let current = Expr {
            object: Val::Expression(Box::from(prev)),
            operator: all[index].parse().unwrap(),
            subject: all[index+1].parse().unwrap(),
        };
        println!("{:?}", current);
        to_tree(all, current, index+2)
    }

    let res= to_tree(stuff, first, 3);

    // let res = Box::from(second.eval());
    println!("{:?}", res);
    return;
}

// fn apply_action(pre: u8, oper: Box<Action>) -> u8 {
//     return if oper.operator == "+" {
//         // print!("plus!");
//         pre + oper.subject
//     } else { 0 }
// }