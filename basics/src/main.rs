fn main() {
    println!("Welcome to Rust!");

    // Compound Types - Arrays
    // Unlike a tuple, every element of an array must have the same type.
    let a_val = [1, 2, 3, 4, 5];
    println!("{}", a_val[2]);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{}", months[3]);

    // Compound Types - Tuples
    // Tuples can have variety of types as shown below
    let tup_val: (i32, f64, u8) = (10, 6.5, 1);
    println!("{}", tup_val.1);



}
