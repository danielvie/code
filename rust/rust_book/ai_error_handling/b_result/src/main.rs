use std::{fs::File, io::{self, ErrorKind, Read}};

// fn main() {
fn main() {

    // ================================================
    // the result enum
    // ================================================

    // enum Result<T< E> {
    //     Ok(T),
    //     Err(E),
    // }

    // ================================================
    // lets call a function that returns a Result
    // ================================================

    // let greeting_file_result = File::open("hello.txt");
    // println!("result: {:?}", greeting_file_result);

    // ================================================
    // lets use match with the greeting_file
    // ================================================

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file: {error:?}"),
    // };
    // println!("greeting_file: {:?}", greeting_file);

    // ================================================
    // matching on differenct errors
    // ================================================

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // println!("greeting_file: {greeting_file:?}");

    // ================================================
    // alternatives to using mach with Result<T, E>
    // ================================================

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // println!("greeting_file: {greeting_file:?}");

    // ================================================
    // shortcuts for Panic on Error
    // ================================================

    // ------------------------------------------------
    // if we run this, then we'll see an panic!
    // ------------------------------------------------

    // let greeting_file = File::open("hello.txt").unwrap();

    // ------------------------------------------------
    // with `expect`, then we can choose the error message
    // ------------------------------------------------

    // let _ = File::open("hello.txt").expect("hello.txt should be included in this project");

    // ================================================
    // Propagating Errors
    // ================================================

    // fn read_username_from_file() -> io::Result<String> {
    //     let username_file_result = File::open("hello.txt");

        // let mut username_file = match username_file_result {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // let result = read_username_from_file();
    // println!("result: {result:?}");

    // ================================================
    // The ? Operator Shortcut
    // ================================================
    
    
}
