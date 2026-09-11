use calc_lib::math::{sub, sum};

fn main() {
    let a = 8;
    let b = 4;

    println!("{a} + {b} = {}", sum(a, b));
    println!("{a} - {b} = {}", sub(a, b));
}