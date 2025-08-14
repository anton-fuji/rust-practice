struct User {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

fn main() {
    let user_1 = User {
        first_name: "John".to_string(),
        middle_name: Some("Hoge".to_string()),
        last_name: "Fuga".to_string(),
    };
    print_initials(&user_1);
}

fn print_initials(user: &User) {
    print!("{}.", user.first_name.chars().next().unwrap());
    if let Some(middle) = &user.middle_name {
        if let Some(initial) = middle.chars().next() {
            print!("{initial}.");
        }
    }
    println!("{}.", user.last_name.chars().next().unwrap());
}
