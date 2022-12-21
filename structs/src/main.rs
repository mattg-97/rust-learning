fn main() {
    let mut user1 = User {
        email: String::from("matthew.gill@and.digital"),
        username: String::from("mattjgill"),
        active: true,
        sign_in_count: 1
    };
    println!("{}", user1.email);
    user1.email = String::from("mattjohngill@gmail.com");
    println!("{}", user1.email);
    let user2 = build_user(String::from("help@email.com"), String::from("helpmail"));
    println!("{}", user2.email);
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("admin@mail.com"),
        ..user1
    };

    println!("User 3 username: {}", user3.username);
    let point_of_origin = Point(0, 0, 0);
}

struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
