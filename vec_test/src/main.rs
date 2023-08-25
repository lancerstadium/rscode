#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    //
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
    println!("{:?}", v);


    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

}
