fn main() {
    // sum ・product
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum: i32 = v.iter().sum();
    let prod: i32 = v.iter().product();
    println!("sum: {sum}, product: {prod}");

    // map
    let m: Vec<_> = v.iter().map(|&c| c % 2 == 0).collect();
    println!("map: {m:?}");

    // filter
    let f: Vec<_> = v.into_iter().filter(|&x| x % 3 == 0).collect();
    println!("{f:?}");

    // zip (create new tuple)
    let a = vec![1, 2, 3, 4, 5];
    let b = vec!['a', 'b', 'c', 'd', 'e'];
    for (num, char) in a.iter().zip(b.iter()) {
        println!("{num}: {char}");
    }

    // fold
    let vv = vec![1, 2, 3];
    let fld = vv.into_iter().fold(0, |acc, x| acc + x);
    println!("{fld}");
}
