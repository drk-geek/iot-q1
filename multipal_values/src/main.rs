fn main() {
   let s = String::from("Pakistan");
   let (result,result_1) = lenght(s);
   println!("the lenght of the word {} is {}",result_1,result);
}

fn lenght(name : String) -> (usize,String) {
(name.len(),name)
}