// 配列はスタック領域に確保される

fn arr() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let m = months[5];
    println!("{}", m);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
