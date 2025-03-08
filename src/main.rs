fn main() {
    let mut word = String::from("Hello Nine");
    word.push_str("string");
    println!("this is a new string {}", word);
    let slice = word.clone();
    let str = &slice[0..];
    println!("The string is {}\n", str);
    println!("the string len is {}", word.len());
}
