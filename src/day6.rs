use crate::reuse;
use std::convert::TryFrom;

use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        match string.parse::<char>() {
            Ok(v) => {
                return match v {
                    '*' => Ok(Self::Multiply),
                    '+' => Ok(Self::Add),
                    _ => Err(format!("Can't format character: {}", v)),
                }
            }
            Err(e) => Err(format!("Not a character: {}. {}", string, e)),
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(usize),
    Operation(Operation),
}

impl TryFrom<String> for Value {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // First try to parse into a usize

        match string.trim().parse::<usize>() {
            Ok(v) => return Ok(Value::Number(v)),
            Err(_) => {
                // Do nothing
            }
        }
        // Now try parsing into an operation
        let operation: Operation = string.try_into()?;
        Ok(Value::Operation(operation))
    }
}

#[derive(Debug)]
struct Problem {
    operation: Operation,
    numbers: Vec<usize>,
    total: Option<usize>,
}

impl Problem {
    fn get_total(&mut self) {
        match self.operation {
            Operation::Add => self.total = Some(self.numbers.iter().sum()),
            Operation::Multiply => self.total = Some(self.numbers.iter().product()),
        }
    }
}

#[derive(Debug)]
struct Workbook {
    problems: Vec<Problem>,
    total: Option<usize>,
}

impl Workbook {
    fn new(input: &str) -> Workbook {
        let lines = reuse::split_input_by_line(input);
        let length_of_line: usize = lines.first().expect("No lines read").len();
        let num_ops_per_line: usize = lines
            .last()
            .expect("No lines read")
            .split_ascii_whitespace()
            .count();
        let size_of_chunk = (length_of_line + 1) / num_ops_per_line;
        let size_of_chunk = 5;
        let chunks: Vec<(usize, String)> = lines
            .into_iter()
            .map(|s| {
                let temp_vec: Vec<(usize, String)> = reuse::chunks_owned(&s, size_of_chunk)
                    .into_iter()
                    .enumerate()
                    .map(|(index, mut s)| {
                        s.truncate(size_of_chunk - 1);
                        (index, s)
                    })
                    .collect();
                temp_vec
            })
            .flatten()
            .collect();

        dbg!(&chunks);
        fn create_hashmap(components: Vec<(usize, String)>) -> HashMap<usize, Vec<String>> {
            let mut new_hashmap: HashMap<usize, Vec<String>> = HashMap::new();
            for (key, value) in components.into_iter() {
                new_hashmap.entry(key).or_insert_with(Vec::new).push(value);
            }
            new_hashmap
        }
        fn transform_hashmap(components: HashMap<usize, Vec<String>>) -> Vec<Problem> {
            let mut problems: Vec<Problem> = Vec::new();
            for problem_key in components.keys() {
                let relevant_strings: &Vec<String> = components
                    .get(problem_key)
                    .expect("Can't get component by key");
                dbg!(relevant_strings);
                let mut new_hashmap: HashMap<usize, Vec<char>> = HashMap::new();
                for s in relevant_strings.iter().take(relevant_strings.len() - 1) {
                    for (index, c) in s.chars().enumerate() {
                        new_hashmap.entry(index).or_insert_with(Vec::new).push(c);
                    }
                }
                let operation: Operation = relevant_strings
                    .iter()
                    .last()
                    .expect("No operation found")
                    .trim()
                    .to_string()
                    .try_into()
                    .expect("Can't parse");
                dbg!(&new_hashmap);
                let mut numbers: Vec<usize> = Vec::new();
                for key in new_hashmap.keys() {
                    let c_vec = new_hashmap.get(key).expect("Can't get component by key");
                    let as_string: String = c_vec.into_iter().filter(|c| c != &&' ').collect();
                    let as_usize: usize = as_string.parse::<usize>().expect("Can't parse as usize");
                    numbers.push(as_usize);
                }
                let problem = Problem {
                    operation: operation,
                    numbers: numbers,
                    total: None,
                };
                problems.push(problem);
            }
            problems
        }
        let hash_map = create_hashmap(chunks);
        let problems = transform_hashmap(hash_map);

        Workbook {
            problems: problems,
            total: None,
        }
    }
    fn get_total(&mut self) {
        let mut total: usize = 0;
        for problem in self.problems.iter() {
            match problem.total {
                Some(v) => total += v,
                None => panic!("No total"),
            }
        }
        self.total = Some(total)
    }
    fn new_1(input: &str) -> Workbook {
        let lines = reuse::split_input_by_line(input);
        let components: Vec<(usize, String)> = lines
            .iter()
            .map(|value| {
                let list: Vec<(usize, String)> = reuse::split_string_to_list_w_index(value, ' ')
                    .iter()
                    .map(|(_index, value)| value.to_string())
                    .filter(|x| x.trim().len() > 0)
                    .enumerate()
                    .collect();
                list
            })
            .flatten()
            .collect();
        let mut new_hashmap: HashMap<usize, Vec<Value>> = HashMap::new();
        for (key, value) in components.into_iter() {
            let value: Value = value.try_into().expect("Failed to parse operation.");
            new_hashmap.entry(key).or_insert_with(Vec::new).push(value);
        }
        let problems: Vec<Problem> = new_hashmap
            .into_iter()
            .map(|(_key, values)| {
                let mut numbers: Vec<usize> = Vec::new();
                let mut operation: Option<Operation> = None;
                for v in values.into_iter() {
                    match v {
                        Value::Number(v) => numbers.push(v),
                        Value::Operation(v) => operation = Some(v),
                    }
                }
                Problem {
                    operation: operation.expect("No operation found"),
                    numbers: numbers,
                    total: None,
                }
            })
            .collect();
        Workbook {
            problems: problems,
            total: None,
        }
    }
}

pub fn main(input: &str) -> u128 {
    let mut parse = Workbook::new(input);
    for problem in parse.problems.iter_mut() {
        problem.get_total();
    }
    parse.get_total();
    parse.total.expect("No total available") as u128
}
