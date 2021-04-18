const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Max points constant is {}", MAX_POINTS);
    // By default variables are immutable
    let mut x = plus_one(4);
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    // but you can still shadow variables
    let x = x + 1; // Useful for doing some transformations and then making it immutable after those transformations
    println!("x is {}", x);
    // shadowing also lets us change the type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Num spaces: {}", spaces);
    // Rust has tuples! The can't change in size after declaration
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    snake_case_function();

    println!("This function returns {}!", five());

    // Rust has implied returns from conditionals just like Kotlin
    let condition = true;

    let number = if condition { 5 } else { 6" };

    println!("The value of number is: {}", number);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    return 5;
}

// Rust uses snake_case for fn names
fn snake_case_function() {
    println!("tunnel_snakes_rule!")
}
