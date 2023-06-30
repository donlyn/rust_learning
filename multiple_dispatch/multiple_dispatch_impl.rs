use std::ops::Add;

#[derive(Debug)]
struct Position<T> {
    n: T
}

impl<T: Add<Output = T>> Add<Position<T>> for Position<T> {
    type Output = Position<T>;
    fn add(self, rhs: Position<T>) -> Self::Output {
        Position { n: self.n + rhs.n }
    }
}

impl<T: Add<Output = T>> Add<T> for Position<T> {
    type Output = Position<T>;
    fn add(self, rhs: T) -> Self::Output {
        Position { n: self.n + rhs }
    }
}


impl<T: Add<Output = T>> Add<Position<T>> for T {
    type Output = Position<T>;
    fn add(self, rhs: T) -> Self::Output {
        Position { n: self + rhs.n }
    }
}


fn main() {
    let _pos1 = Position { n: 1 } + Position { n: 2 }; 
    let _pos2 = Position { n: 1 } + 2;
    // let _pos3 =  1 + Position { n: 2 };
}
