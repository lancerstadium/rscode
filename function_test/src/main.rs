fn main() {
    println!("Hello, world!");
    another_function(5, 6); // argument

    let x = 5;
    let y = { // block
        let x = 1;
        plus_five(x)
    };
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}

fn another_function(x: i32, y: i32) { // parameter
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}