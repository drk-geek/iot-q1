use std::io;

fn main() {
let mut s = String::new();


io::stdin().read_line(&mut s)
    .expect("Failed to read a line");

println!("{}",s);

let mut s_interger:u32 = s.trim().parse()
    .expect("Please input a digit");
s_interger = s_interger +1;
println!("{}",s_interger);
}
