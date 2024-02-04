fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    let result: i32 = add(x, y);

    println!("{} + {} = {}", x, y, result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
