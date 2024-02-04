fn main() {
    let mut x: i32 = 1;

    println!("{}", x);

    double(&mut x);

    println!("{}", x);
}

fn double(x: &mut i32) {
    *x += *x;
}
