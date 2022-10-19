// use std::io;

fn concat_string(a: &str, b: &str) {
    let result =format!("{} {}", a, b);
    println!("{result}");
}

fn main() {
    concat_string("I Love", "Rust");
}
