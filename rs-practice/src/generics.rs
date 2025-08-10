fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers1 = vec![34, 50, 25, 100];
    let result1 = largest(&numbers1);
    println!("The largest number is: {result1}");

    let numbers2 = vec![34, 50, 25, 47, 300, 90, 100];
    let result2 = largest(&numbers2);
    println!("The largest number is: {result2}");

    let chars = vec!['y', 'm', 'a', 'q'];
    let result_char = largest(&chars);
    println!("The largest character is: {result_char}");
}
