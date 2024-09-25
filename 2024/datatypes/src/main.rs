fn main() {
    // Integer
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Integer: {guess}");

    let decimal = 98_222;
    println!("Decimal: {decimal}");

    let hex = 0xff;
    println!("Hex: {hex}");

    let octal = 0o77;
    println!("Octal: {octal}");

    let binary = 0b1111_0000;
    println!("Binary: {binary}");

    let byte = b'A';
    println!("Byte: {byte}");

    // Float
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("FLoat: {x}, {y}");

    // Operator
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");
    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
