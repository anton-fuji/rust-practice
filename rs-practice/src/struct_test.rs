#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Self {
            title: String::from(title),
            author: String::from(author),
            pages,
        }
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn total_pages(&self, other: &Book) -> u32 {
        self.pages + other.pages
    }
    fn get_author(&self) -> &str {
        &self.author
    }
}

fn main() {
    let book1 = Book::new("Hoge", "Fuga", 560);
    let book2 = Book::new("A", "John", 1000);
    println!("{:?}", book1);
    println!("{}", book1.get_title());
    println!("{}", book2.total_pages(&book1));
    println!("{}", book1.get_author());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_book() {
        let book = Book::new("The Rust Programming Language", "Steve Klabnik", 560);
        assert_eq!(book.pages, 560);
        assert_eq!(book.get_title(), "The Rust Programming Language");
    }

    #[test]
    fn test_total_pages() {
        let book1 = Book::new("The Rust Programming Language", "Steve Klabnik", 560);
        let book2 = Book::new("Rust in Action", "Tim McNamara", 450);
        let total = book1.total_pages(&book2);
        assert_eq!(total, 1010);
        assert_eq!(book1.get_title(), "The Rust Programming Language");
    }
}
