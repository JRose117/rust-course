// Shadowing
// shadowing is not the same as marking as mutable
// if you reassign -> like x = 10 it will have an error
// needs x = x something
// can you use let keyword in shadowing
fn main() {
  let x = 5;
  let x = x + 1;
  // the compiler will only read the final value (so 6)
  let x = x + 1;
  // and now x is 7 - this is shadowing
  // creating a variable with the same name and shadowing the first one
  {
    // in this inner scope the x is shadowed and doubled
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  } 
  // outer scope so just 7
  println!("The value of x is: {x}");
  println!("Hello, world!");
}
