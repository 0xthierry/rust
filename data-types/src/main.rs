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

    // compound types
    // we have two primitive types: tuples and arrays
    // tuples can have more than one type
    // tuples cannot grow or shrink in size
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (_, k, _) = tuple;

    println!("The value of k is: {}", k);
    println!("The value of tuple.1 is: {}", tuple.1);

    // array can have only one type
    // different from other languages, array have fixed size
    // stored on the stack
    let arr = [1,2,3,4,5];
    // typing and array and specifying the size
    let arr_typed: [i32; 5] = [1,2,3,4,5];
    // array initialized
    // all values equal to 3 and length 5
    let a_initialized = [3; 5];
}
