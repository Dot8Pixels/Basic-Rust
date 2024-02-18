use std::rc::Rc;
fn main() {
    let x = Rc::new(Box::new(10));

    let y = Rc::clone(&x);
    let z = Rc::clone(&x);

    println!("y: {}, z: {}", y, z);
}
