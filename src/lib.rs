use std::io;

pub fn run() {
    println!("Enter FIRST number: ");
    let a = read_number();
    println!("Enter OPERAND (+, -, * or /)");
    let operand = read_user_input();
    println!("Enter SECOND number: ");
    let b = read_number();
    
    if a.is_none() || b.is_none() {
        panic!("Incorrect value");
    }

    let n1 = a.unwrap();
    let n2 = b.unwrap();
    let result = calculate(n1, n2, &operand);

    print!("\n\n{n1} {operand} {n2} = {result}");
}

fn calculate(n1: f64, n2: f64, operand: &String) -> f64 {
    let result;
    match operand.as_str() {
        "-" => result = n1 - n2,
        "+" => result = n1 + n2,
        "*" => result = n1 * n2,
        "/" => {
            if n2 == 0.0 {
                panic!("Cannot divide by 0!")
            } else {
                result = n1 / n2
            }
        }
        _ => panic!("Invalid operand!"),
    }
    result
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read input");
    String::from(input.trim())
}

fn read_number() -> Option<f64> {
    let str = read_user_input();
    match str.parse::<f64>() {
        Ok(value) => Some(value),
        Err(err) => None
    }
}