struct User {
    name: String,
    email: String,
    age: u8
}

fn main() {
    let user1 = User {
        name: String::from("Jack"),
        email: String::from("jack@gmail.com"),
        age: 1
    };

    let user2 = User {
        name: String::from("John"),
        ..user1
    };

    println!("{}", user1.name);
}