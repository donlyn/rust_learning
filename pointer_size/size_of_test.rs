#![allow(unused)]
fn main() {
    use std::rc::Rc;
    let pointer_size = std::mem::size_of::<&u8>();
    println!("for pointer or reference to dynamically sized types:");
    println!("{}, {}", pointer_size, std::mem::size_of::<&[u8]>());
    println!("{}, {}", pointer_size, std::mem::size_of::<*const [u8]>());
    println!("{}, {}", pointer_size, std::mem::size_of::<Box<[u8]>>());
    println!("{}, {}", pointer_size, std::mem::size_of::<Rc<[u8]>>());
    println!("");
    println!("for pointer or reference to fixed sized types:");
    println!("{}, {}", pointer_size, std::mem::size_of::<&bool>());
    println!("{}, {}", pointer_size, std::mem::size_of::<*const bool>());
    println!("{}, {}", pointer_size, std::mem::size_of::<Box<bool>>());
    println!("{}, {}", pointer_size, std::mem::size_of::<Rc<bool>>());
}
