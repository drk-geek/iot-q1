// fn main(){
//     let mut counter = 0;
//     let val_counter = loop {
//     println!("Hello world!");
//     counter = counter + 1;
//     if (counter == 10) {
//        break counter
//     }    
// };
// println!("{}",val_counter);
// }

// fn main (){

//     let mut counter = 0;
//     while counter < 3 {
//         println!("Hello World");
//         counter = counter +1;
//     }
// }
// let mut counter_1 =0;
// let lottery_number =[1,23,45,91,78,100];
// while counter_1 < lottery_number.len() {
//     println!("{}",lottery_number[counter_1]);
//     counter_1 = counter_1+1;
// }
// }

fn main() {

for a in (0..5).rev() {
    println!("{}. Hello World",a);
}


let lottery_number = [1,23,45,67,89];
for num in lottery_number.iter(){
    println!("{}",num);
}
}