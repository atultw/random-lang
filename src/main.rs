fn main() {
    let program = "1 + 2";

    compile(program.parse().unwrap())
}

struct Action {
    operator: String,
    subject: u8,

}


fn compile(input: String) {
    let stuff: Vec<&str> = input.split(" ").collect();
    // println!("{:?}", stuff);

    let first = Action {
        operator: stuff[1].parse().unwrap(),
        subject: stuff[2].parse().unwrap()
    };

    let res = apply_action(stuff[0].parse().unwrap(), Box::from(first));
    println!("{:?}", res);
    return
}

fn apply_action(pre: u8, oper: Box<Action>) -> u8 {
    return if oper.operator == "+" {
        // print!("plus!");
        pre + oper.subject
    } else { 0 }
}