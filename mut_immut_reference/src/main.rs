fn main() {
    let s = String::from("Hello");
    let b = &s;
    let c = &s;
    let d = &s;
    println!("{}, {}. {}", b,c,c);
}
