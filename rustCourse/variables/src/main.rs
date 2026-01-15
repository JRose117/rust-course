// Variables are immutable?

fn main() {
    println!("Hello, world!");
    // mut makes it mutable
    let mut a: i32 = 5;
    println!("The value of a is {}", a);
    a = 10;
    println!("The value of a is {}", a);
}
