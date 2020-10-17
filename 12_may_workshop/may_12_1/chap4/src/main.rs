fn main() {
   let age1 :u8 = 22; //pop
   println!("Age is {:?}",age1);
   //let food = String::new();
   let food = String::from("Zinger Burgur");
   println!("Food is {:?}",food);
   kfc(food); //owner of food moved to kfc
   //println!("{}",food); //it will give us error
   let food2 = String::from("Mac Fullary");
   macdonald(food2.clone()); //food2 remains in scop
   println!("After clone {}",food2); //it will print
   let mut chai = String::new(); 
   chai.push_str("Mix Tea");
   println!("data of chai {}",chai);
   println!("Pointer/address/refernce of chai {:p}",&chai);
   println!("Lenght of chai {}",chai.len());
   pc(&chai);
   println!("After sending to PC");
   println!("data of chai {}",chai);
   println!("Pointer/address/refernce of chai {:p}",&chai);
   println!("Lenght of chai {}",chai.len());
   
   println!("End of program");
   let mut food3 = String::new();
   println!("Befor sending {:?}",food3);
   tandoadam(&mut food3);
   println!("After sending {:?}",food3); 

} // drop food2 (pop age1) //chai drop
fn kfc(deal:String) { // deal takes ownership
    println!("we are in KFC");
    println!("Today deal is: {}",deal);
} //drop deal
fn macdonald(deal1:String) { //deal1 string type
    println!("we are in Macdonals");
    println!("Today deal is: {}",deal1);
} //deal1 drops here

fn pc(tea:&String) {
    println!("We are at Pathan Continatal");
    println!("Today deal is: {}",tea);
    println!("Address of Chai is: {:p}",tea);
    println!("Address of tea : {:p}",&tea);

} // drop will  not call here

fn tandoadam(today:&mut String) {
    println!("we are in TandoAdam");
    println!("Before push {}",today);
    today.push_str("Mutton Sajji");
    println!("After push {}",today);
}