fn main() {
    let score: i32 = 65;

    if score > 80 {
        println!("A");
    } else if score >= 70 {
        println!("B")
    } else if score >= 60 {
        println!("C")
    } else if score >= 50 {
        println!("D")
    } else {
        println!("E")
    }
}
