fn main() {
    let x = 4;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x); // Error: cannot assign twice to immutable variable `x`
}
