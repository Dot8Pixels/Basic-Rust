fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;

    let max_number: &i32 = max(&x, &y);

    println!("{}", max_number)

    // let r: &i32;

    // {
    //     let x: i32 = 5;
    //     r = &x;

    //     println!("{}", r);
    // }
}
