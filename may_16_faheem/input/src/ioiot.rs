use std::io;

pub fn take_input()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input
}