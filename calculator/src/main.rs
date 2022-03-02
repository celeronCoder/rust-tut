use std::io::{stdin, stdout, Write};

fn read(input: &mut String) { // taking a refrence variable to set the value of input without returning anything
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to the calculator!");
    println!("-------------------------");

    loop {
        let mut num1: String = String::new();
        let mut num2: String = String::new();
        let mut operator: String = String::new();

        print!("What is the first number?: ");
        read(&mut num1);
        print!("What is the second number?: ");
        read(&mut num2);
        print!("What operation would you like to do: ");
        read(&mut operator);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap(); // only takes the first char in the string

        let operators: String = String::from("+-*/");

        if !operators.contains(operator) {
            println!("unknown operator");
            continue; // exits the operator
        }

        let result: f32 = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator")
        };

        println!("the result of {} {} {} = {}", num1, operator, num2, result);
    }
}