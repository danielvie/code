fn main() {

    // ----------------------
    // creating a New String
    let mut s = String::new();

    // we can create a str literal, then convert to string
    let data = "initial contents";
    let s = data.to_string();


    // we can call the `to_string` method from str literal
    let s = "initial contents".to_string();

    let s1: String = String::from("ik ben hier");
    let mut s2: &str = "ik ben hier";





    // &str::
    // borrowed string slice
    // stored in the program's binary
    // fixed-size and immutable
    // no heap allocation
    // cheap to copy

    // String
    // owns its contents
    // allocates memory on the heap
    // memory is freed when goes out of scope
    // moving it transfers ownership


    // UPDATING STRING
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("string: {s}");

    let s2 = " again";
    s.push_str(s2);
    println!("new string: {s}");


    // the .push method takes a single character
    let mut s = String::from("lo");
    s.push('8');
    println!("s: {s}");
    
    // concatenating with + ro format!

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;

    println!("concatenate string: {s3}");
    println!("concatenate string: {s1}");

    // the `+` operator works like `fn add(self, rhs: &str) -> String`

    let s1 = String::from("Hello, ");
    let s3 = format!("{s1}{s2}");
    println!("");
    println!("format:: {s1}");
    println!("format:: {s2}");
    println!("format:: {s3}");

    // concatenate multiple strings

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    
    let s = format!("{s1}-{s2}-{s3}");
    
    
    println!("\nconcat multi: {s}");


    // indexing into Strings
    // rust strings dont support indexing

    // let s1 = String::from("hi");
    // let h = s1[0];


    let s1 = "test";
    let s  = &s1[0..1];
    println!("[[test]][0] -> {s}");


    // you can get the unicode scalar values using .chars()

    let s = String::from("hoi");
    println!("");
    for c in s.chars() {
        println!("{c}");
    }

    
    // you can also get the bytes
    println!("");
    for c in s.bytes() {
        println!("{c}");
    }

}
