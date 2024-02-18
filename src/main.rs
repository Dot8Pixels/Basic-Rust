use std::{cell::RefCell, rc::Rc};
fn main() {
    let x = Rc::new(RefCell::new(Box::new(10)));

    let y = Rc::clone(&x);
    let z = Rc::clone(&x);

    println!("y: {:?}", y);
    println!("z: {:?}", z);

    *z.borrow_mut() = Box::new(20);

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}
