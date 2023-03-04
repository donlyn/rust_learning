struct Point(i32, i32);
fn subfn() {
  impl Point {
    fn x(&self) -> i32 { self.0 }
  } 
}

fn main() {
  let p = Point(1, 2);

  println!("{}", p.x());
}
