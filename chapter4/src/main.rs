// fn main() {
//     println!("Hello, world!");
// }


// #![allow(unused_variables)]
// fn main() {
// let s = "hello";
// }
// fn main() {
//     {                      // s is not valid here, it’s not yet declared
//         let s = "hello";   // s is valid from this point forward

//         // do stuff with s
//     }                      // this scope is now over, and s is no longer valid
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
