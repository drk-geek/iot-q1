use std::io;
fn main() {
    println!("Enter First Name:");
    let mut Danish_139199 = String::new();
    io::stdin().read_line(&mut Danish_139199);
    let Danish_139199:String = Danish_139199.trim().parse().unwrap();
    println!("Name: {}",Danish_139199);

    println!("Enter Roll Number");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2);
    let input_2:String = input_2.trim().parse().unwrap();
    println!("Roll Number: {}",input_2);
    //println!("Name: {} Roll No: {}",input_1,input_2);
}
    

    





