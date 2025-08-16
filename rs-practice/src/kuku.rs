fn main() {
    for i in 1..=9 {
        for j in 1..=9 {
            print!("{} x {} ={:2} ", j, i, i * j);
        }
        println!()
    }
}
