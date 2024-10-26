use std::io::Write;

fn main() {
    println!("Rusty calculator");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Modulus");
    println!("6. Exit");
    print!("Enter your choice: ");

    let mut choice: String = Default::default();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    let choice: u8 = choice.trim().parse().expect("Invalid choice entered.");

    if choice == 6 {
        println!("Exiting..");
        std::process::exit(0);
    }

    let mut x: String = Default::default();
    let mut y: String = Default::default();

    println!("\nLet's enter two numbers for the calculation");
    print!("Enter the first number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line.");
    let x: i32 = x.trim().parse().expect("Invalid number entered.");

    print!("Enter the second number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line.");
    let y: i32 = y.trim().parse().expect("Invalid number entered.");

    match choice {
        1 => {
            // addition
            println!("{} + {} = {}", x, y, x + y);
        }
        2 => {
            // subtraction
            println!("{} - {} = {}", x, y, x - y);
        }
        3 => {
            // multiplication
            println!("{} * {} = {}", x, y, x * y);
        }
        4 => {
            // division
            if y == 0 {
                println!("Cannot divide by zero.");
                std::process::exit(1);
            } else {
                println!("{} / {} = {}", x, y, x / y);
            }
        }
        5 => {
            // modulus
            println!("{} % {} = {}", x, y, x % y);
        }
        _ => {
            // default
            println!("Unrecognised operation.");
            std::process::exit(1);
        }
    }
}
