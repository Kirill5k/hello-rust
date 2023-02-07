use std::io;
use rand;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    say_hello(&name);

    let numbers_array = [1, 2, 3, 4, 5, rand::random()];
    let tuple = (10, 3.14, "string");
    println!("array {numbers_array:?} of length {}", numbers_array.len());
    println!("first item from tuple is {}", tuple.0);

    let mut count = 0;
    let result = loop {
        if count == 10 { break count * 10 }
        count += 1;
        println!("count is {count}")
    };
    println!("loop result is {}", sum(result, 0));

    for (i, n) in numbers_array.iter().enumerate() {
        println!("item {i} is {n}")
    }
}

fn say_hello(name: &str) {
    let unicode_char = '\u{261D}';
    println!("Hello, {name}!{unicode_char}")
}

// lack of the semicolon defines the return
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
