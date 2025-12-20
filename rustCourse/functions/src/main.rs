// functions

// functions start with fn

// any function is written in snake case

fn main() {
    hello_world();
    tell_height(182);
    human_id("Alice", 55, 182.3);
    let _x: i32 = {
      let price: i32 = 5;
      let qty: i32 = 10;
      price * qty
    };
    println!("Result is {} ", _x);
    let y: i32 = add(4,6);
    println!("value of y is: {}", y);
    println!("value from function 'add' is : {}", add(4,6));
    // calling BMI function
    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);
    // the {:.2} means display 2 decimals points
    println!("Your BMI is: {:.2}", bmi)
}

// We have hoisting in rust like in JavaScript
fn hello_world(){
  println!("hello Rust 🦀!");
}

// you can insert input values

fn tell_height(height: i32){
  println!("My height is {} cm", height);
}

// you can insert multiple inputs

fn human_id(name: &str, age: u32, height: f32){
  println!("My name is {}, I am {} years old, and my height is {}. cm", name, age, height);
}

// Expressions and Statements

// expressions return values, statements don't
fn add (a: i32, b: i32) -> i32{
  a + b
}

// BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
  weight_kg / (height_m * height_m)
}