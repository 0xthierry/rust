fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); error, because the refence is moved to s2
    //**Ownership Rules**
    //    - Each value has a variable that's called its owner
    //    - There can only be one owner at a time
    //    - When the owner goes out of scope, the value will be dropped
    println!("{}, world!", s2);
}
