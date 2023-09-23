struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("vvrs@github.com"),
        username: String::from("vvrs"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("vvrs2");

    let user2 = build_user(String::from("abc@xyz.com"), String::from("abc"));

    let user3 = User {
        username: String::from("pqrs"),
        email: String::from("pqrs@xyz.com"),
        ..user2
    };

    // Tuple structs
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    // Calculating the area
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(height1, width1)
    );

    // With Structs
    let rect = (30, 50);

    // To remove ambiguity, create a struct
    println!(
        "The area of the rectangle is {} square pixels.",
        aread(rect)
    );

    let rects = Rectangle {
        height: 30,
        width: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        areas(&rects)
    );

    // Printing structs
    println!("Rectangle: {:?}", rects);
    println!("Rectangle Area: {}", rects.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}

fn aread(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn areas(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
