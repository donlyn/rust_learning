
struct Point {
  x: i32,
  y: i32
}
impl Point {
  fn get_x(&mut self) -> i32 {
    self.x
  }
}
fn main() {
  let mut p = Point { x: 1, y: 2 };
  let x = &mut p.get_x();
  *x += 1;
  println!("{} {}", *x, p.y);
}
