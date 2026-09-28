
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
        email: email,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    
    let user2 = build_user(String::from("user2@example.com"), String::from("bla"));

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };


    println!("user1 email: {}", user1.email);
    println!("user2 email: {}", user2.email);
    println!("user3 email: {}", user3.email);
    println!("user3 name: {}", user3.username);

    
}