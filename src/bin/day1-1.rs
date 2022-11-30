use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day1.txt").unwrap();

    println!("input:\n{input}")
}
