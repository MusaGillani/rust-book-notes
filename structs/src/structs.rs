struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("t@g.com"),
        username: String::from("t"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("new name");

    let user2 = build_user(String::from("new@t.com"), String::from("new user"));

    let user3 = User {
        email: String::from("another@t.com"),
        username: String::from("another user"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn create_tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}
