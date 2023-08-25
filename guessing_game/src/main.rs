use rand::Rng;
use std::io; // prelude // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行！");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small, 太小！"),
            Ordering::Greater => println!("Too Big, 太大！"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }

    }
}
