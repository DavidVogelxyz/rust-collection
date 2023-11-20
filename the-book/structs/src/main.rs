fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("user@domain.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@domain.com");

    let user2 = User {
        email: String::from("new@domain.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn colored_pencils() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;

fn check_equal() {
    let subject = AlwaysEqual;
}
