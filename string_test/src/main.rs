fn main() {
    // let s = "initial contexts".to_string();
    let mut s = String::from("initial contexts");

    s.push_str(" bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    //let a = a1 + "-" + &a2 + "-" + &a3;
    let a = format!("{a1}-{a2}-{a3}");
    println!("{a}");
}
