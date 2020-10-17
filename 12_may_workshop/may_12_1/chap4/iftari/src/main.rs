fn main() {
    let aftari = ["Mix Chat","Samosay","Pakoray","Dahi Baray"];
    // index         0         1         2         4
    for mydata in aftari.iter(){
        println!("Every Iteration {}",mydata);
    }

    //for num in (0..=5) {
      //  println!("Aftari index {} is {}",num,aftari[num]);
    // }

    for num in (0..5) {
          println!("Aftari index {} is {:?}",num,aftari.get(num));
      }
    for num in (0..=10).rev() {

        println!("Num value is {}",num);
    }
    
    let game = "Cricket and Football";
    // index    01234567890123456789
    let game1 = &game[0..7];
    println!("Game 1 is {}",game1);
    let game2 = &game[12..=19];
    println!("Game 2 is {}",game2); 

    //understandig of panic

    let mut score :u8 = 0;
    loop {
        println!("Your score is {}",score);
        score +=1;
        for delay in 0..100000 {}
        if score == 100 {break}
    }
    println!("End of program");
}

