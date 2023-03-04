fn main() {
    let mut s = String::from("hello");

    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    s.push_str(" world");

    println!("{}", r3); // s is immutable reference
    //println!("{} and {}", r3, s); // s is immutable reference
    //println!("{}, {}, and {}", r1, r2, s);
}
