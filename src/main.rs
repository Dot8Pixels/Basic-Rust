fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // for x in xs.iter() {
    //     println!("x is {}", x);
    // }

    for (i, v) in xs.iter().enumerate() {
        println!("{} -> {}", i, v);
    }
}
