fn main() {
    println!("Hello, world!");

    let g = another_function(5);
    println!("The value of g is: {}", g);
}

fn another_function(x: i32) -> i32 {
    x + 1
}
