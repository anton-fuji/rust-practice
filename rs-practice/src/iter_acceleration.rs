use rayon::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    let data: Vec<u64> = (0..1_000_000_00).collect();
    let _sum: u64 = data.iter().sum();
    let before_duration = start.elapsed();

    println!("処理時間:{:?}", before_duration);
    // 処理時間:1.529928708s

    let start = std::time::Instant::now();
    let data: Vec<u64> = (0..1_000_000_00).collect();
    let _sum: u64 = data.par_iter().sum();
    let after_duration = start.elapsed();

    println!("処理時間:{:?}", after_duration);
    // 処理時間:1.123860833s
}
