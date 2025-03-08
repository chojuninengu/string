fn main() {
    let mut word = String::from("Hello Nine");
    word.push_str("string");
    println!("this is a new string {}", word);
    let slice = word;
    let str = &slice[1..5];
    println!("The string is {}", str);

    println!("Hello, world!");
}
