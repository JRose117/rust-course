fn main(){
  // arrays
  // everything in an array needs to be the same type (no mixing numbers
  // or letters) you also need to say the size of the array
  // i32 is the type, and 5 is length of array
  let numbers: [i32; 5] = [1,2,3,4,5];
  println!("Number Array: {:?}", numbers);
  let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
  println!("Fruit Array element 1: {:?}", fruits);
  println!("Fruit Array element 1: {}", fruits[0]);
  println!("Fruit Array element 2: {}", fruits[1]);
  println!("Fruit Array element 3: {}", fruits[2]);

  // Tuples
  // can be mixed types
  let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
  println!("Human typle: {:?}", human);
  // tuples can also return arrays
  let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
  println!("My Mix Typle: {:?}", my_mix_tuple);

  // Slices
  let number_slices: &[i32] = &[1,2,3,4,5];
  println!("Number Slice: {:?}", number_slices);
  let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
  println!("Animal Slice: {:?}", animal_slices);
  let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
  println!("Book Slice: {:?}", book_slices);

  // Strings vs String Slices (&str)
  // strings (growable, mutable, owned string types)
  let mut stone_cold: String = String::from("Hell");
  stone_cold.push_str("yeah!");
  println!("Stone Cold Says: {}", stone_cold );

  // string slice is immutable
  let string: String = String::from("Hello, World!");
  // if you add [0..5] it will return the characters up to the 5th character
  let slice: &str = &string[0..5];
  println!("Slice Value, {}", slice);
}

