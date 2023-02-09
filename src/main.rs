use std::io;
use rand;

#[derive(Debug)]
#[derive(Clone)]
struct SpaceShuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl SpaceShuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> SpaceShuttle {
        SpaceShuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

// tuple struct
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T
}

impl<T> Rectangle<T> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

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

    let vehicle = SpaceShuttle::new("Endeavour");

    let vehicle2 = SpaceShuttle {
        name: String::from("Discovery"),
        ..vehicle
    };
    println!("vehicle 1 is {:?}\n vehicle 2 is {:?}", vehicle, vehicle2)
}

fn say_hello(name: &str) {
    let unicode_char = '\u{261D}';
    println!("Hello, {name}!{unicode_char}")
}

// lack of the semicolon defines the return
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
