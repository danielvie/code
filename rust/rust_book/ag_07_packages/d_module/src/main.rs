mod mega;

use mega::greetings;

fn main() {
    println!("{}", greetings::hello(greetings::DEFAULT_NAME));
}
