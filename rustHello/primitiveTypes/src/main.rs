fn main(){
  let x: i32 = 42;
  let y: u64 = 100;
  // signed positive (so range is half of unsigned because covers 
  // negative to positive)
  // unsigned - no negative
  println!("Signed Integer: {}", x);
  println!("Unigned Integer: {}", y);
  // Floats
  let pi: f64 = 3.14;
  println!("Value of Pi = {}", pi);
  
  // boolean
  let is_snowing: bool = true;
  println!("is it snowing? {}", is_snowing);
  // character type - char
  let letter: char = 'a';
  println!("First letter of alphabet: {}", letter);

}

