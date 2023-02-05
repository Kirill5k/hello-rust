
fn main() {
    let var = "World";
    let unicode_char = '\u{261D}';
    let numbers_array = [1, 2, 3, 4, 5];
    let tuple = (10, 3.14, "string");
    println!("Hello, {var}!{unicode_char}");
    println!("array {numbers_array:?} of length {}", numbers_array.len());
    println!("first item from tuple is {}", tuple.0)
}