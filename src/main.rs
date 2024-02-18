#[derive(Debug)]

enum List {
    Node(i32, Box<List>),
    Nil,
}

fn main() {
    // let x: i32 = 10;
    // let y: Box<i32> = Box::new(x);
    // println!("{:?}", y);

    let list: List = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
