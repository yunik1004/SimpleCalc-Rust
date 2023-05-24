mod parser;
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        parser::test(line.unwrap()).ok();
    }
}
