fn main() {
    println!("Hello, world!");
    let n = 10; // statement no return
    another_function(n);
    another_function(five());
}

fn five() -> i32 {
    5 // expression, return a value, and don't have semicolon
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}