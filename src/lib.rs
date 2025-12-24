use std::fs;
mod day3;
mod reuse;

pub fn real_main() -> u64 {
    let file: &str = "src/input-main.txt";
    let contents = fs::read_to_string(file).expect("File not found");
    let output = day3::main(&contents);
    output
}
