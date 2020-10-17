#[derive(Debug)]
struct Colour (i32, i32, i32);
struct points (i32, i32, i32);
fn main() {
    let black = Colour(6,9,0);
    println!("{:#?}",black);
}
