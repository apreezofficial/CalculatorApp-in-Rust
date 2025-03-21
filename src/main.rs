use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("Enter first number:");

        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please type a number!");

        println!("Enter operator (+, -, *, /):");

        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        println!("Enter second number:");

        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Please type a number!");

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
