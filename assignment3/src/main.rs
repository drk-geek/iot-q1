#[derive(Debug)]
struct Student {
    name : String,
    age: u8,
    grade: String,
    percentage: f32,
}

impl Student {
    fn construct()->Student{
        Student {
            name : String::from("Hassan Naseer Malick"),
            age: 23,
            grade: String::from("A-1"),
            percentage: 84.4,
    }
}
fn ppt(&self){
    println!("{}",self.percentage);
}
}

fn main(){
    Student::construct(String::from("Hassan Naseer Malick"),23,String::)
}