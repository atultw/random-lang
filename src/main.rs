
fn main() {
    let program = "1 + 2 + 10";

    compile(program.parse().unwrap())
}

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
    let stuff: Vec<&str> = input.split(" ").collect();
    // println!("{:?}", stuff);

    println!("{:?}", stuff.iter().position(|&r| r == "+").unwrap());

    let first = Expr {
        object: Val::Given(String::from(stuff[0]).parse().unwrap()),
        operator: stuff[1].parse().unwrap(),
        subject: stuff[2].parse().unwrap(),
    };

    let second = Expr {
        object: Val::Expression(Box::from(first)),
        operator: stuff[3].parse().unwrap(),
        subject: stuff[4].parse().unwrap()
    };

    let res = Box::from(second.eval());
    println!("{:?}", res);
    return;
}

// fn apply_action(pre: u8, oper: Box<Action>) -> u8 {
//     return if oper.operator == "+" {
//         // print!("plus!");
//         pre + oper.subject
//     } else { 0 }
// }