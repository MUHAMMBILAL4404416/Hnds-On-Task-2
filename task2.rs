use std::io::{self,Read};

enum Operation 
{
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(num1: f64, num2: f64, operation: Operation) -> f64 
{
    match operation 
    {
        Operation::Add=>num1+num2,
        Operation::Subtract=>num1-num2,
        Operation::Multiply=>num1*num2,
        Operation::Divide=>num1/num2
    }
}

fn main() {
    let mut input_str=String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input_str).expect("Plz enter a number");
    let num1=input_str.trim().parse::<f64>().expect("Invalid number");
    input_str.clear();

    println!("Kindly Choose any of these Operations (+, -, *, /): ");
    io::stdin().read_line(&mut input_str).expect("Plz enter an Opoerator");
    let operation_char=input_str.trim().chars().next().expect("Invalid input");
    input_str.clear();

    println!("Enter the second number: ");
    io::stdin().read_line(&mut input_str).expect("Plz enter a number");
    let num2=input_str.trim().parse::<f64>().expect("Invalid number");
    input_str.clear();

    let operation=match operation_char 
    {
        '+' =>Operation::Add,
        '-' =>Operation::Subtract,
        '*' =>Operation::Multiply,
        '/' =>Operation::Divide,
        _ => panic!("Invalid symbol"),
    };

    let result=calculate(num1,num2,operation);
    println!("Result: {}",result);
}
