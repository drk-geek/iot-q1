fn main() {
  let mut s = String::from("Hello");

{
    let a = &mut s;
    a.push_str(" world");
    println!("{}",a);
}
{
    let b = &mut s;
    println!("{}",b);
}
}