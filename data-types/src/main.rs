fn main() {
    // we have two type of data types, scalar types and compound types
    // scalar types
    // integer types, it can be i(signed) or u(unsigned), the bits go from 8 to 128
    // so you can use something like this i64
    // the default size is i32
    // to two the range of values in this case is -(2 ** (32 -1)) and 2 ** (32 - 1) - 1
    let binary: i32 = 0b0111;
    let hex: i32 = 0xff;
    let n = 32;

    // floating types
    // the default size is f64 because is the same speed as f32 on modern cpus
    let x = 2.0;

    // boolean types
    let t = true;
    let f: bool = false; // explicit type annotation


    // char types
    let c = 'z';
    let cat_emoji = '😻';
}
