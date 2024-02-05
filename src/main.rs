fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = vec![];

    for i in v1.iter() {
        if is_even(i) {
            v2.push(*i);
        }
    }

    println!("v2: {:?}", v2);
}

fn is_even(num: &i32) -> bool {
    num % 2 == 0
}
