#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ストラクトに関連したメソッドを作成できる
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20,
    };

    println!(
        "This area of the Rectangle is {} square pixels.",
        rect.area()
    );
    println!("{:?}", rect)
}
