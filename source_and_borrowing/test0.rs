fn main() {
    // let a = Box::new(String::from("a literal string"));
    // let mut b = *a;
    // b.push_str(" here is a pushed string");
    let a = Box::new(4);
    let mut b = *a;
    b = 5;

    println!("{} and {}", a, b);
}
