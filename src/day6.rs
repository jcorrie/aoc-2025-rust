use crate::reuse;

use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
    total: Option<usize>,
}

#[derive(Debug)]
struct Workbook {
    problems: Vec<Problem>,
    total: Option<usize>,
}

impl Workbook {
    fn new(input: &str) -> Workbook {
        let lines = reuse::split_input_by_line(input);
        let components: Vec<(usize, &str)> = lines
            .iter()
            .map(|value| reuse::split_string_to_list_w_index(value, ' ') if )
            .flatten()
            .collect();
        dbg!(&components);
        Workbook {
            problems: Vec::new(),
            total: None,
        }
    }
}

pub fn main(input: &str) -> u128 {
    let parse = Workbook::new(input);
    52
}
