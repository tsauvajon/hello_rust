// Struct with named fields
struct User {
    // We use a String over a &str because we want User to have the ownership
    // of all its data
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Structs with anonymous fields
// Those are different types: a function accepting a Color as parameter will
// not accept a Point
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like struct
struct UnitLikeStruct();

fn main() {
    instanciate_struct();
    mutate_struct();
    builder();
    anonymous_struct();
}

fn instanciate_struct() {
    let user1 = User {
        email: String::from("thomas.sauvajon.dev[at]gmail.com"),
        username: String::from("tsauvajon"),
        active: true,
        sign_in_count: 18000000,
    };

    log_user(&user1);
}

fn mutate_struct() {
    // The whole instance must be mutable, can't mark only 1 field as mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    log_user(&user1);
}

fn log_user(u: &User) {
    println!(
        "email: {}, username: {}, active: {}, sign in count: {}",
        u.email,
        u.username,
        u.active,
        u.sign_in_count,
    );
}

fn builder() {
    let email = String::from("rene.lataupe@free.fr");
    let username = String::from("rlatop");
    let taupe = build_user(email, username);
    log_user(&taupe);

    let user2 = build_from_another(&taupe);
    log_user(&user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        // Shortand similar to JavaScript
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_from_another(u: &User) -> User {
    // Both instanciations give the same value
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: u.active,
        sign_in_count: u.sign_in_count,
    };

    User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // Copy everything that isn't email or username
        ..*u
    }
}

fn anonymous_struct() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
