use std::fs;
mod day6;
mod reuse;

pub fn real_main() -> u128 {
    let file: &str = "src/input-main.txt";
    let contents = fs::read_to_string(file).expect("File not found");
    let output = day6::main(&contents);
    output
}
