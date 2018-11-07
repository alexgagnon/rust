// create a struct of key-value pairs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// create a named tuple (also a struct)
// good for if you want semantic meaning behind two tuples that might have the same types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// can create no-field structs, useful for assigning Traits
struct Nothing {}

fn main() {
    // use [structname] and then key value pairs to define it
    // keys don't have to be in same order
    let user1 = User {
        email: String::from("Alex"),
        username: String::from("alexgagnon"),
        active: true,
        sign_in_count: 1,
    };

    // you must define the entire instance to be mutable to change any of its values
    let mut user2 = User {
        email: String::from("Lisa"),
        username: String::from("lisafabbro"),
        active: false,
        sign_in_count: 2,
    };

    // user1.active = false; // illegal, not mut
    user2.sign_in_count = user2.sign_in_count + 1; // fine

    let user3 = build_user(String::from("hello"), String::from("there"));
    let mut user4 = build_user(String::from("goodbye"), String::from("then"));
    user4.username = String::from("forever");

    print_user(&user1);
    print_user(&user2);

    // can quickly use other instance values in a new one using update syntax ..
    let _user5 = User {
        username: String::from("Gandalf"),
        email: String::from("gandalf@middle.earth"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        // if a variable matches a struct key, use the shorthand
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.active);
    println!("{}", user.sign_in_count);
}
