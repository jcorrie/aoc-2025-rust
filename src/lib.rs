use std::fs;
mod day5;
mod reuse;

pub fn real_main() -> u128 {
    let file: &str = "src/input-main.txt";
    let contents = fs::read_to_string(file).expect("File not found");
    let output = day5::main(&contents);
    output
}
