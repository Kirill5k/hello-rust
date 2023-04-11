

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let add_one = |x: i32| {
        x + 1
    };

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}