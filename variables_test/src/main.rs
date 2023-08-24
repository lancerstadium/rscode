// const MAX_POINTS: u32 = 100000;

fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let x = 5;
    let x = x + 1; // shadowing
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // shadowing

    println!("spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");

    println!("guess is: {}", guess);

    // u32: 0 ~ 2^n-1
    // i32: -(2^n-1) ~ 2^(n-1) -1
    // usize/isize

    // Integer: i32 default
    // Float: f64 default (IEEE-754)
    let x = 2.0; 
    println!("The value of x is {}", x);

    // Bool: 1 Bytes
    // let t = true;
    // let f = false;

    
    // Char: 4 Bytes (Unicode)
    // let x = 'z';
    // let y: char = 'z';
    let z = '😘';
    println!("The unicode emoji is {}", z);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{}, {}, {},", x, y, z);


    // array --> stack栈上
    // let a = [1, 2, 3, 4, 5];
    let a= [3; 5];
    println!("a[0] is {}", a[0]);
    println!("{:?}", a);


}
