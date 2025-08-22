use std::collections::HashMap;

fn main() {
    let mut book_review = HashMap::new();

    book_review.insert(
        "Rust Book".to_string(),
        "My First Favorite book".to_string(),
    );
    book_review.insert("Go Book".to_string(), "My Second Favorite book".to_string());
    book_review.insert(
        "TypeScript Book".to_string(),
        "My Third Favorite book".to_string(),
    );
    book_review.insert(
        "The Advance Rust".to_string(),
        "My Fourth Favorite book".to_string(),
    );

    for (book, review) in &book_review {
        println!("{book} : {review}")
    }
}
