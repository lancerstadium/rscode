

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    match number {
        4 => println!("number is divisible by 4"),
        3 => println!("number is divisible by 3"),
        2 => println!("number is divisible by 2"),
        _ => println!("number is not divisible by 4, 3, or 2")
    }

    // let if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    let result= loop {
        counter += 1;
        println!("again, {}!", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is :{}!", result);

    // // while
    // while number != 0 {

    // }

    
    // for


}
