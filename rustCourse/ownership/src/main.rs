// ownership, borrowing and references

// three rules 
// each value has an owner
// there can only be one owner at a time
// when the owner goes out of scope, the value will be dropped
fn main() {
    let s1 = String:: from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    // s1 is now invalid because 
    // let s1 = String:: from("RUST");
    // let s2 = s1;
    // println!("{}", s2);

    // scope example
    // let s1 = String:: from("RUST");
    // let len = calculate_length(&s1);
    // println!("Length of '{}' is {}, s1, len")

}

// fn print_lost(S: &string){
//   println!("{}", &s1);
// }

fn calculate_length(s: &String) -> usize{
  s.len()
}