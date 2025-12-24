#![allow(dead_code)]

use regex::Regex;

pub fn string_to_usize(input: &str) -> u128 {
    let output = match input.parse::<u128>() {
        Ok(value) => Some(value),
        Err(_) => None,
    };
    output.expect("Not a usize")
}



pub fn split_string_to_list_of_ints(input: &str) -> Vec<usize> {
    let mut output_vec: Vec<usize> = Vec::new();
    let char_vec: Vec<char> = string_to_char_vec(input, true);
    for character in char_vec {
        let parsed_int: u32 = character.to_digit(10).expect("Error reading digit A");
        if parsed_int <= 9 {
            output_vec.push(parsed_int.try_into().expect("Error reading digit B"));
        } else {
            output_vec.push(parsed_int.try_into().expect("Error reading digit C"));
            // panic!("Input not single digit number");
        }
    }
    output_vec
}

pub fn split_string_to_list(input: &str, delimiter: char) -> Vec<&str> {
    input.split(delimiter).collect()
}

pub fn split_input_by_line(input: &str) -> Vec<String> {
    input.lines().map(|line| line.replace('\n', "")).collect()
}


pub fn regex_vec_groups(input: &str, pattern: &str) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(input).expect("no matches");
    matches.push(caps.get(1).map_or("", |m| m.as_str()).to_string());
    matches.push(caps.get(2).map_or("", |m| m.as_str()).to_string());
    matches.push(caps.get(3).map_or("", |m| m.as_str()).to_string());

    matches
}

pub fn regex_string(input: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.find_iter(input)
        .map(|match_| match_.as_str().to_string())
        .collect()

}
pub fn chunks(string: &str, chunks: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / chunks);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(chunks) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

pub fn string_to_char_vec(string: &str, remove_whitespace: bool) -> Vec<char> {
    if remove_whitespace {
         string.chars().filter(|c| !c.is_whitespace()).collect()
    } else {
        string.chars().collect()
    }
}