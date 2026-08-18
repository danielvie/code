| Length                 | Signed | Unsigned |
| ---------------------- | ------ | -------- |
| 8-bit                  | i8     | u8       |
| 16-bit                 | i16    | u16      |
| 32-bit                 | i32    | u32      |
| 64-bit                 | i64    | u64      |
| 128-bit                | i128   | u128     |
| Architecture-dependent | isize  | usize    |

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

# tuple

general way of grouping a number of values

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

# array
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```


# functions

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

# control flow
```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

```rust
loop {
    println!("again!");
}
```

https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop