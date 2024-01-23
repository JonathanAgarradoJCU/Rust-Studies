use std::io;

fn main() {
    // Read the lengths of the two legs from the user
    let leg_a = read_input("Enter the length of side A: ");
    let leg_b = read_input("Enter the length of side B: ");

    // Calculate the length of the hypotenuse using the Pythagorean theorem
    let hypotenuse = pythagorean_theorem(leg_a, leg_b);

    // Display the result
    println!("The length of the hypotenuse is: {}", hypotenuse);
}

fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn pythagorean_theorem(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}
