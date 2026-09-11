mod calc;

// use calc::{soma, multi};

fn main() {
    let a = 4;
    let b = 8;
    let s = calc::soma(a, b);
    let m = calc::multi(a, b);
    println!("multi de {a} * {b} = {m}");
    println!("soma  de {a} + {b} = {s}");
}