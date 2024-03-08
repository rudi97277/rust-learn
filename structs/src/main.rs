fn main() {
    let mut user1 = build_user(String::from("rudi@gmail.com"), String::from("rudi97278"));

    println!(" name {}", user1.username);
    println!(" name {}", user1.username);

    let mut user2 = User {
        email: String::from("rudi9@gmail.com"),
        ..user1
    };

    let user1 = User2 {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
