use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    print_scores(&scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Get team blue's score is {score}.");

    scores.insert(String::from("Blue"), 25);
    print_scores(&scores);

    scores.entry(String::from("Red")).or_insert(33);
    print_scores(&scores);


    // part2
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

fn print_scores(sc: &HashMap<String, i32>) {
    println!("=====Teams:=====");
    for (key, value) in sc {
        println!("{key}: {value}");
    }
    println!("================");
}
