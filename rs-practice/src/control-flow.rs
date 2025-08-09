fn main() {
    let mut num = 3;

    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    println!("LIFTOFF!")
}

// match文
/*
*
*#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let red: Color = Color::Red;
    let green: Color = Color::Green;
    let blue: Color = Color::Blue;
    println!("Red Color Code: {}", color_to_str(&red));
    println!("Green Color Code: {}", color_to_str(&green));
    println!("Blue Color Code: {}", color_to_str(&blue));
}

fn color_to_str(color: &Color) -> &str {
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
    }
}

* */
