// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn printMesage(is_gt_100: bool) {
    match is_gt_100 {
        true => println!("Its Big"),
        false => println!("Its small"),
    }
}

struct Book {
    pages: i32,
    rating: f32,
}

fn display_page_count(book: &Book) {
    println!("Pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Book rating = {:?}", book.rating);
}
fn main() {
    let my_number = 400;
    let size = my_number > 100;
    printMesage(size);

    // memory management

    let book = Book {
        pages: 20,
        rating: 30.5,
    };

    display_page_count(&book);
    display_rating(&book)
}
