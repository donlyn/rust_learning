
struct Point {
  x: i32,
  y: i32
}
impl Point {
  fn get_x<'a>(&'a mut self, other : &'a mut Point) -> &'a mut i32 {
    &mut other.x
  }
}
fn main() {
  let mut p = Point { x: 1, y: 2 };
  let mut o = Point { x: 6, y: 7 };
  let x = p.get_x(&mut o);
  *x += 1;
  println!("{} {}", *x, p.y);
}
