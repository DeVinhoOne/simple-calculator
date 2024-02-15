use std::io;

pub fn run() {
    loop {
        println!("Enter FIRST number: ");
        let mut a = read_number();
        while a.is_none() {
            a = read_number();
        }
        println!("Enter OPERAND (+, -, * or /)");
        let operand = read_user_input();
        println!("Enter SECOND number: ");
        let mut b = read_number();
        while b.is_none() {
            b = read_number();
        }
    
        let n1 = a.unwrap();
        let n2 = b.unwrap();
        let result = calculate(n1, n2, &operand);
    
        println!("\n\n{n1} {operand} {n2} = {result}\n\n");
    }
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
        Err(err) => {
            println!("Given value is not a valid number. Try again");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        let n1 = 18.0;
        let operand = String::from("/");
        let n2 = 0.0;
        calculate(n1, n2, &operand);
    }

    #[test]
    fn test_division() {
        let n1 = 18.0;
        let operand = String::from("/");
        let n2 = 3.0;
        assert_eq!(6.0, calculate(n1, n2, &operand));
    }

    #[test]
    #[should_panic(expected = "Invalid operand!")]
    fn not_valid_operand() {
        calculate(5.0, 9.0, &String::from("?"));
    }
}