fn main() {
    //variaveis
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let user1 = User {
        active: true,
        username: String::from("Farcj"),
        email: String::from("farcj@email"),
        sign_in_count: 1,
    };
    println!("the user is {}", user1.username);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: user1.email,
        username: user2.username,
        ..user1
    };
    println!(
        "the area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "the area of the rectangle with tuple is {} square pixels.",
        area_with_tuples(rect1)
    );
    println!(
        "the area of the rectangle with struct is {} square pixels.",
        area_with_struct(&rect2)
    );

    println!(
        "the area of the rectangle with mthods of struct is {} square pixels.",
        rect2.area()
    );
}

//Structures
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

//functions
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
