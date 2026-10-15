struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("ashish"),
        email: String::from("ashish@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    user1.email = String::from("new@example.com");
    user1.sign_in_count += 1;
}