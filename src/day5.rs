use crate::reuse;

use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Ingredient {
    id: usize,
    is_fresh: Option<bool>,
}

#[derive(Debug)]
struct Inventory {
    ingredients: Vec<Ingredient>,
    fresh_ranges: Vec<(usize, usize)>,
}
impl Inventory {
    fn new(input: &str) -> Inventory {
        let lines = reuse::split_input_by_line(input);
        let regex_pattern: Regex = Regex::new(r"\d+").expect("Regex pattern corrupt");
        let mut inventory = Inventory {
            ingredients: Vec::new(),
            fresh_ranges: Vec::new(),
        };
        fn parse_first_part(line: &String, re: &Regex) -> (usize, usize) {
            let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
            if matches.len() != 2 {
                panic!("Wrong number of items in range");
            }
            let fresh_range: (usize, usize) = (
                matches
                    .get(0)
                    .expect("No first range")
                    .parse::<usize>()
                    .expect("Can't parse first range"),
                matches
                    .get(1)
                    .expect("No first range")
                    .parse::<usize>()
                    .expect("Can't parse first range"),
            );
            fresh_range
        }
        let mut in_first_part_flag: bool = true;
        // First loop
        for line in lines.iter() {
            if line.trim().len() == 0 {
                in_first_part_flag = false;
                continue;
            }
            match in_first_part_flag {
                true => {
                    inventory
                        .fresh_ranges
                        .push(parse_first_part(line, &regex_pattern));
                }
                false => {
                    let ingredient: Ingredient = Ingredient {
                        id: line.parse::<usize>().expect("Can't parse ingredient"),
                        is_fresh: None,
                    };
                    inventory.ingredients.push(ingredient);
                }
            }
        }
        inventory
    }
    fn is_fresh(&self, id: usize) -> bool {
        self.fresh_ranges
            .iter()
            .any(|(start, end)| &id >= start && &id <= end)
    }
    pub fn check_numbers(&self) -> usize {
        let mut counter: usize = 0;
        for ingredient in self.ingredients.iter() {
            let is_fresh: bool = self.is_fresh(ingredient.id);
            match is_fresh {
                true => counter += 1,
                false => continue,
            }
        }
        counter
    }

    fn sort_ranges(mut self) -> Self {
        self.fresh_ranges.sort_by_key(|&(start, _end)| start);
        self
    }
    fn deduplicate_ranges(mut self) -> Self {
        let mut previous_end: usize = 0;
        for (index, (start, end)) in self.fresh_ranges.iter_mut().enumerate() {
            // Start by skipping the first row.
            if index == 0 {
                previous_end = *end;
                continue;
            }
            dbg!(previous_end);
            dbg!(&start);
            if previous_end >= *start {
                *start = previous_end + 1;
            }
            if start <= end {
                previous_end = *end;
            }
        }
        self
    }

    fn count_ranges(mut self) -> u128 {
        let mut counter: u128 = 0;
        for (start, end) in self.fresh_ranges.iter() {
            if start <= end {
                counter += (end - start) as u128 + 1;
            }
        }
        counter
    }
}
pub fn main(input: &str) -> u128 {
    let inventory = Inventory::new(input);
    let counter = inventory.sort_ranges().deduplicate_ranges().count_ranges();
    counter
}
