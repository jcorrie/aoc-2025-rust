use std::fs;
mod day4;
mod reuse;

pub fn real_main() -> usize {
    let file: &str = "src/input-main.txt";
    let contents = fs::read_to_string(file).expect("File not found");
    let output = day4::main(&contents);
    output
}
