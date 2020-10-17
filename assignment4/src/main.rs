#[derive(Debug)]
enum Direction{
    Forward,
    Left,
    Backward,
    Right,
}

impl Direction {
    fn next_move(&self) -> Direction {
        match self {
            Direction::Left => Direction::Forward,
            Direction::Right  =>  Direction::Left,
            Direction::Backward => Direction::Right,
            Direction::Forward => Direction::Backward,
        }
    }
}

fn main() {
    
    let mut move1 = Direction::Left;

    for i in (0..10) {
        
        println!("{:?}", move1);
        move1 = move1.next_move();
        
    }
}