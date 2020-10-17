#[derive(Debug)]
struct Food {
    resturatnt: String,
    food_item: String,
    size:u8,
    price:u16,
    availability:bool,
    
}

fn main(){

    let pizza = Food {
        resturatnt: String::from("Pizza Hut"),
        food_item: String::from("Chicken Fajita"),
        size:16,
        price:1500,
        availability:true,
    };

    let karahi = Food {
        resturatnt: String::from("BBQQ Tonight"),
        food_item: String::from("Chicken Gonger"),
        size:1,
        price:1200,
        availability:true,
};
    println!("We are in Pizza Hut");
    println!("Pizza Price {}",pizza.price);
    println!("Pizza {:#?}",pizza);
    println!("We are in BBQ Tonight");
    println!("Karahi Price {}",karahi.price);
    println!("karahi {:#?}",karahi);

    printing(pizza);
    printing(karahi);

}

fn printing(data:Food){
    println!("Data We are in Pizza Hut");
    println!("Pizza Price {}",data.price);
    println!("Pizza {:#?}",data);
    println!("Data We are in BBQ Tonight");
    println!("Karahi Price {}",data.price);
    println!("karahi {:#?}",data)

}

fn pc() ->Food {
    let Chai = Food {
        resturatnt: String::from("Pathan"),
        food_item: String::from("Mix Tea"),
        size:,
        price:100,
        availability:true,
}