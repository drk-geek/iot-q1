// fn main() {
//     let mut name = String::from("Danish");
//     let salary:i32 = 50_000;
//     let fee:f64 = 25_00.85;
   
//    println!("First Name: {} Fees is {} and Salary is {}",name,fee,salary);
//    //user_define();

//    //square();

//    //fn user_define(){
//     let lname = &mut name;
//     lname.push_str(" Rehman");
//     println!("Full Name : {}",lname);
//     //}

//     fn square(&mut fee: f64) -> (){
//         let result = fee*fee;
//         println!("{}". result);
//     }


// }

fn main(){
    let age :u8 = 22;
    let height :f32 = 5.5;
    let mut course = String::from("IOT");
    println!("{}",age);
    println!("{}",height);
    println!("{}",course);
    let height_square = printing(age, height, &mut course);
    println!("{}",height_square);
    println!("{}",course);

}
fn printing(age:u8,height:f32,course:&mut String) ->f32{
    course.push_str(" Batch 6 - 34");
    for temp in 0..age {
        print!("{}",temp);
    }
    height*height

}

