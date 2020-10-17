#[derive(Debug)]
enum Move{
    Walk,
    Jump,
    Run,
    Hop,

}

fn daily(data:Move){
    
}
fn main() {
    let mymove = Move::Walk;
    println!("{:?}",mymove);
    let urmove = Move::Jump;
    println!("{:?}",urmove);
    let run = Move::Run;
    println!("{:?}",run);
    let hop = Move::Hop;
    println!("{:?}",hop);
}
