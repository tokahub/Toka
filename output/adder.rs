use std::io;

fn main() {
    // Prompt the user for the first number
    println!("Enter the first number:");

    // Read the first number from user input
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Please enter a valid number");

    // Prompt the user for the second number
    println!("Enter the second number:");

    // Read the second number from user input
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Please enter a valid number");

    // Calculate the sum of the two numbers
    let sum = num1 + num2;

    // Print the result
    println!("The sum of {} and {} is {}", num1, num2, sum);
}