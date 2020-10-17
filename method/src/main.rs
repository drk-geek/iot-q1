// struct Rectangle {
//     height:u32,
//     width:u32,
// }

// impl Rectangle {
//     fn area (&self) -> u32 {
//         self.width*self.height
//     }
// }
// fn main() {
//     let rec_1 = Rectangle {height:100, width:50};
//     let rec_2= Rectangle {height:90, width:10};
//     let result = rec_1.area();
//     let result_1 = rec_2.area();
//     println!("The area of Rectangle is: {}",result);
//     println!("The area of 2nd Rectangle is: {}",result_1);
// }


struct Rectangle {
    height :u32,
    width: u32,
}
impl Rectangle {
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main () {

    let rec_1 = Rectangle {height:100, width:50};
    let rec_2 = Rectangle {height:90, width:40};
    let rec_3 = Rectangle {height:70, width:30};

    let result = rec_1.can_hold(&rec_2);
    println!("Rec_1 can hold Rec_2 : {}",result);
    
    println!("Rec_1 can hold Rec_2: {}",rec_1.can_hold(&rec_3))
}
