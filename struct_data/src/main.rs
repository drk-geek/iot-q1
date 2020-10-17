#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16,
    availbility: bool,
}




// fn main() {
//     let book_1 = Book {
//         name:String::from("Book A"),
//         author:String::from("Author A"),
//         price: 500,
//         availbility:true,
//     };

//     let book_2 = Book {
//         name:String::from("Book B"),
//         author:String::from("Author B"),
//         price: 600,
//         availbility:true,
//     };  

//     println!("{:#?}",book_1);
//     println!("{:#?}",book_2);
// }

fn main() {
    let book_1 = Book {
        name: String::from("Book A"),
        author: String::from("Author A"),
        price: 500,
        availbility: true,
    };
    
    let book_2 = Book {
        name:String::from("Book B"),
        author:String::from("Author B"),
        // price:book_1.price,
        // availbility:book_1.availbility,
        ..book_1
    };

println!("{:#?}",book_2);
}