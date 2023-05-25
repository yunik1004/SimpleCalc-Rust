mod compiler;
mod parser;
use std::env;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);

    let input_path = &args[1];
    let output_path = &args[2];

    let input = fs::read_to_string(input_path).unwrap();
    match parser::parse_str(&input) {
        Ok(ast) => {
            let out = compiler::codegen(&ast, "main");
            let mut out_file = File::create(output_path).unwrap();
            out_file.write_all(out.as_bytes()).ok();
        }
        Err(e) => eprintln!("Parse failed: {:?}", e),
    };

    /*
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        match parser::parse_str(&input) {
            Ok(ast) => {
                let out = compiler::codegen(&ast, "calc");
                println!("{out}");
            }
            Err(e) => eprintln!("Parse failed: {:?}", e),
        };
    }
    */
}
