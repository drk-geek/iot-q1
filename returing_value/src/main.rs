fn main() {
   let result = gives_ownership();
   println!("{}",result);
    let s1 = String::from("Pakistan");
    let result1 = takes_and_gives_back(s1);
    println!("{}",result1);
}

fn gives_ownership () -> String {
    let s = String::from("Hello world");
    s
}


fn takes_and_gives_back(x: String) -> String {   
x    
}