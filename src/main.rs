use std::fs;
mod day1;
mod reuse;

fn main() {
    let file: &str = "src/input-main.txt";
    let contents = fs::read_to_string(file).expect("File not found");
    let output = day1::main(&contents);
    println!("{:?}", output)
}