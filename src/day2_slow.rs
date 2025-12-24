use crate::reuse;
use rayon::prelude::*;

fn get_divisors(number: u128) -> Vec<u128> {
    (1..=number).filter(|x| number % x == 0).collect()
}

fn is_duplicate(id: &str) -> bool {
    let divisors = get_divisors(id.len() as u128);
    let chars: Vec<char> = id.chars().collect();
    for divisor in divisors.iter() {
        let chunk_length = *divisor as usize;
        if chunk_length == id.len() {
            continue;
        };
        let mut chunks = chars.chunks(chunk_length);

        let first_chunk = chunks.next().expect("No first chunk");
        if chunks.all(|c| c == first_chunk) {
            return true;
        } else {
        }
    }
    false
}
fn list_files(file_range_raw: &str) -> Vec<u128> {
    let bookends: Vec<&str> = reuse::split_string_to_list(file_range_raw, '-');
    let bookends_num: Vec<u128> = bookends
        .iter()
        .map(|x| reuse::string_to_usize(x.trim()) as u128)
        .collect();
    let range: Vec<u128> = (*bookends_num.first().expect("Range not right")
        ..=*bookends_num.last().expect("Range not right - end"))
        .collect();
    range
}

pub fn main(input: &str) -> u128 {
    let input_list = reuse::split_string_to_list(input, ',');
    let mut list_of_invalids: Vec<u128> = Vec::new();

    for item in input_list.iter() {
        let file_range: Vec<u128> = list_files(item);
        for file in file_range.iter() {
            let is_invalid = is_duplicate(&file.to_string());
            if is_invalid {
                list_of_invalids.push(*file);
            }
        }
    }
    let sum_of_invalids: u128 = list_of_invalids.iter().sum();
    sum_of_invalids as u128
}
