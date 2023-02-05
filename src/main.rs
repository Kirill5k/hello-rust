fn main() {
    let numbers_array = [1, 2, 3, 4, 5];
    let tuple = (10, 3.14, "string");
    say_hello("World");
    println!("array {numbers_array:?} of length {}", numbers_array.len());
    println!("first item from tuple is {}", tuple.0)
}

fn say_hello(name: &str) {
    let unicode_char = '\u{261D}';
    println!("Hello, {name}!{unicode_char}")
}

// lack of the semicolon defines the return
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
