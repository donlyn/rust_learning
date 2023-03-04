fn main() {
    let x = [4; 5];
    let mut y = x; // it is copy for array, rather than move
    y[0] = 3;
    println!("x = {:?}, y = {:?}", x, y)
}
