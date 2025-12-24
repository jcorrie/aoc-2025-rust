use crate::reuse;
use rayon::prelude::*;

struct Bank {
    batteries: Vec<u32>,
    joltage: Option<u32>,
}

impl Bank {
    fn new(input_string: &str) -> Bank {
        dbg!(input_string);
        let mut batteries: Vec<u32> = Vec::new();
        for character in input_string.chars() {
            batteries.push(character.to_digit(10).expect("Failure parsing character"));
        }
        Bank {
            batteries,
            joltage: None,
        }
    }
    fn get_joltage(&mut self) -> Option<u32> {
        // Need at least two elements to compute "largest then largest after it"
        if self.batteries.len() < 2 {
            dbg!("Pack is too short");
            return None;
        }

        // Find (index, value) of the largest element in the whole vec.
        // enumerate() -> (usize, &u32); max_by_key receives a reference to that tuple,
        // so we pattern-match `&(_, val)` to get the &u32 key.
        let mut max_idx: Option<usize> = None;
        let mut max_val: Option<&u32> = None;
        for (idx, val) in self.batteries.iter().enumerate() {
            if idx == self.batteries.len() - 1 {
                continue;
            }
            if (max_val == None || max_val < Some(val)) {
                max_idx = Some(idx.clone());
                max_val = Some(val);
            }
        }

        // Find the largest value in the remainder starting at max_idx + 1.
        // `.skip(max_idx + 1)` yields an Option<&u32>; `.copied()` -> Option<u32>.
        dbg!(max_idx);
        dbg!(max_val);
        let second_value = self
            .batteries
            .iter()
            .skip(max_idx.expect("No max idx") + 1)
            .max()
            .copied();
        dbg!(&second_value);

        let joltage: u32 =
            max_val.expect("No max val") * 10 + second_value.expect("No second value");
        dbg!(joltage);

        self.joltage = Some(joltage);
        self.joltage
    }
}

pub fn main(input: &str) -> u32 {
    let input_list = reuse::split_input_by_line(input);
    let mut power_counter: u32 = 0;
    for line in input_list.iter() {
        let mut bank: Bank = Bank::new(line);
        power_counter += bank.get_joltage().expect("No joltage");
    }
    power_counter
}
