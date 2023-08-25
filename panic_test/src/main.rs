use std::{fs::File, error};

fn main() {
    // panic!("crash and burn");
    let greeting_file_result = File::open("hello,txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}!", error),
    };
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}