fn main() {

    // String
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s3 = s1;
    
    
    println!("s2 = {}, s3 = {}", s2, s3);

    // function
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// reference
fn calculate_length(s: &String) -> usize {
    s.len()
}