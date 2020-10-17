use std::io;
use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut player:Vec<String> = Vec::new();
    let mut ludostar = HashMap::new();
    println!("Welcome to Ludo Star");
    println!("How many players?");
    let value = loop{
    let mut players = String::new();
    io::stdin().read_line(&mut players)
                .expect("Failed to read a line");
    let players_int =  players.trim().parse::<u8>();  
    match players_int{
        Ok(ok)  => break ok ,
        Err(e) => println!("Numbers only!, Enter again"),
                }
            };
    for i in 1..(value+1) {
        println!("Enter Player {} name", i);
        let mut players = String::new();
        io::stdin().read_line(&mut players)
                .expect("Failed to read a line");
                
        let pnumber = players.trim();
        player.push(pnumber.to_string());
    }
    for i in &player  {
        ludostar.insert(i, 0);
    }
    {
    println!("{:?}", &player);
    println!("{:?}", &ludostar);
    }
    let mut rng = rand::thread_rng();
    let dice = Uniform::from(1..7);
    let mut turn = 1;
    let mut win = 0;
    let mut turn = 1;
    
while win < 100 || win > 100{
    let mut player_number = 1;
for person in &player{
    let mut roll = dice.sample(&mut rng);
    let total = ludostar.entry(&person).or_insert(0);
        match roll {
        6 => {
                roll += dice.sample(&mut rng);
                match roll {
                                12 => {
                                        roll +=dice.sample(&mut rng);
                                        match roll{
                                                    18 => {roll = 0;
                                                        println!("Tring!! {} is 0 due to 3 sixes in a row and Total {}", &person, &total) 
                                                            }
                                                    _ => (),
                                                    }    
                                    } 
                                _ =>    (),
                            } 

            }      
            _ => (),
    }
    *total +=roll; 
    win = *total;
    if win == 100{ println!("Turn {}, {} Score is {}, Total is {}\n", turn, person, &roll, *total);
                   println!("Congratulations! {} has won on turn {}", &person, turn); 
                 break}
    else if win > 100 {
        *total -= roll;
    }
    let peet = *total;
    println!("Turn {}, Dice Roll for Player {} - {} Score is {}, Total is {}", turn, player_number, person, &roll, &peet);
    for (k,v) in ludostar.iter_mut(){
        if &person != k {
            if &peet == v{
                *v = 0;
                println!("\n Tring!! {} has been kicked by {} at score of {} \n",&k, &person, &peet);
            }
        }
    }
 player_number += 1;  
}
turn += 1;
        while true {

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
         Ok(temp) => break,
         Err(e) => println!("Something not good"),
            }
        }
}
}       
    