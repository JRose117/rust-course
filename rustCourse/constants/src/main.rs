// Constants

fn main() {
    // difference between a constant and a variable, 
    // you can't use keyword mut with constant
    // a const should have a capital letter
    // you also have to add the type
    // so the below is correct 
    // const Y: i32 = 10;
    let x = 5;
    const Y: i32 = 10;
    const PI: f64 = 3.141592653;
    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
}
