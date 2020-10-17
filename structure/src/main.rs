#[derive(Debug)]
struct Food {
resturant:String,
food_item:String,
size:u8,
price:u16,
availability:bool,

}

fn main(){
    
    let pizza = Food {
        resturant:String::from("Pizza Hut"),
        food_item:String::from("Chicken Fajita"),
        size:16,
        price:1500,
        availability:true
    };
    println!("Pizza {:#?}",pizza);

    fn printing(data:Food){
        resturant:String::from("Pizza Hut"),
        food_item:String::from("Chicken Fajita"),
        size:16,
        price:1500,
        availability:true

    }
}

