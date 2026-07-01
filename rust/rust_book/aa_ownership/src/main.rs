fn append(s: &String, v: &str) -> String {
    format!("{s}{v}")
}

fn takes_own(some_str: String) {
    println!("{some_str}");
}

fn takes_own_and_back(some_str: String) -> String {
    println!("{some_str}");
    some_str
}

fn takes_ref(some_str: &String) {
    println!("{some_str}");
}

fn main() {
    let s = String::from("hello");

    let s2 = append(&s, ", you");
    let s3 = append(&s, ", me");


    let mut hallo = String::from("ik bin hier");
    
    hallo = takes_own_and_back(hallo);
    hallo = takes_own_and_back(hallo);

    println!("s2: {}, s3: {}", s2, s3);
    println!("hallo: {}", hallo);
}
