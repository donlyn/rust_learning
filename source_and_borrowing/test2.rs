
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let a = area(rect1);
  println!("{:?} {}", rect1, a);
}

fn area(rectangle: Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
