struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        translated
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .sum()
    }
}
fn main() {
    let s1 = String::from("III");
    let res = Solution::roman_to_int(s1);
    println!("{res}");
}
