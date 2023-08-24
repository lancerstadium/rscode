#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}



// tuple struct
// struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user3 = build_user(String::from("123@example.com"), String::from("123"));

    // println!("{:?}" ,user1);
    println!("{:?}" ,user2);
    println!("{:?}" ,user3);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width: {}", rect1.width());
    }

    let sq = Rectangle::square(3);

    dbg!(&sq);
    
}

fn build_user(email:String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 123,
    }
}


