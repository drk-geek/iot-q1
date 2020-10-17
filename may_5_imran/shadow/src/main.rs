fn main() {
    let price1 = 300;
    println!("Today Price is {}",price1);
    println!("Address of price1 {:p}",&price1);
    let price1 = price1 + 100;
    println!("Today Price is {}",price1);
    println!("Address of price1 {:p}",&price1);

    let mut salary1:u16 = 10_000;
    println!("Salary 1 is {}",salary1);
    println!("Address of Salary1 {:p}",&salary1);
    salary1 = salary1+2000;
    println!("Salary 1 is {}",salary1);
    println!("Address of Salary1 {:p}",&salary1);

}
