fn main() {
    fn type_a(x: i32) -> i32 {
        x + 1
    }

    let type_b = |x: i32| -> i32 { x + 1 };

    let type_c = |x| x + 1;

    let type_d = |x| x + 1;

    println!("{:?}", type_a(3));
    println!("{:?}", type_b(3));
    println!("{:?}", type_c(3));
    println!("{:?}", type_d(3));
}
