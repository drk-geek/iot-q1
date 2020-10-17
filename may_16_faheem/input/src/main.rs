use std::io;
mod ioiot;
fn main() {
    
    println!("Enter first number");
    let input_01:i32 = ioiot::take_input().trim().parse().unwrap();
    println!("Enter second number");
    let input_02:i32 = ioiot::take_input().trim().parse().unwrap();
    println!("The addition is: {}",(input_01+input_02));

     
}
