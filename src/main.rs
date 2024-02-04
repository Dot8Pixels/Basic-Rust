fn main() {
    let message: String = String::from("Hello, world");
    println!("{}", message);

    let message2: &str = "Hello, world";
    println!("{}", message2);

    /* Compound Type ------------------------------------------------------------ */
    // Tuples
    let tup1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", tup1);

    let slice1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", slice1);
}
