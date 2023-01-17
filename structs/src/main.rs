struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}


struct AlwaysEqual;



fn main() {
    let subject = AlwaysEqual;
    println!("Hello, world!");
    let user1 = User {
        email: String::from("alfonso.egio@gmail.com"),
        name: String::from("Alfonso Oriol Egio Artal"),
        active: true,
        sign_in_count: 0
    };
    println!("{}", user1.name);
    println!("{}", user1.active);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);

    let user2 = User {
        email: String::from("agustin@gmail.com"),
        name: String::from("Agustín barbudo"),
        active: true,
        sign_in_count: 3
    };

    println!("{}", user2.name);
    println!("{}", user2.active);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);

    let user3 = build_user(String::from("lkasjd@laksjd.com"),
                           String::from("owieuwoui"));

    println!("{}", user3.name);
    println!("{}", user3.active);
    println!("{}", user3.email);
    println!("{}", user3.sign_in_count);
}

fn build_user(email: String, name: String) -> User {
    let sign_in_count = 1;
    User {
        email,
        name,
        sign_in_count,
        active: true
    }
}
