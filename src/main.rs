fn main() {
    let message: String = String::from("rayato");

    hello(message);

    println!("{}", message);
}

fn hello(message: String) {
    println!("hello, {}", message)
}
