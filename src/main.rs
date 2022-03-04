use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
    println!(
        "{:?}",
        output(first_number, operator, second_number, result)
    );
}

fn operate(operator: char, first_number: f32, second_mnumber: f32) -> f32 {
    match operator {
        '+' => first_number + second_mnumber,
        '-' => first_number - second_mnumber,
        '/' => first_number / second_mnumber,
        '*' | 'x' | 'X' => first_number * second_mnumber,
        _ => panic!("Invalid operator used"),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}
