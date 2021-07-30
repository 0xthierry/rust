fn main() {
    // a variable is immutable by default, but it can be mutable with `mut`
    let mut x = 5;
    // a const is always immutable
    const PI: f64 = 3.16;
    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y + 2;
    // shadowing also allow you to change the variable type
    let spaces = "    "; // string
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len(); //usize
    println!("The value of spaces is: {}", spaces);

    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of pi is: {}", PI);
}
