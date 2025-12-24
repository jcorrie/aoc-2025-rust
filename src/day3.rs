use crate::reuse;
use rayon::prelude::*;
use std::cmp::Reverse;

struct Bank {
    batteries: Vec<u32>,
    joltage: Option<u64>,
}

impl Bank {
    fn new(input_string: &str) -> Bank {
        let mut batteries: Vec<u32> = Vec::new();
        for character in input_string.chars() {
            batteries.push(character.to_digit(10).expect("Failure parsing character"));
        }
        Bank {
            batteries,
            joltage: None,
        }
    }
    fn get_joltage(&mut self) -> Option<u64> {
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
        let second_value = self
            .batteries
            .iter()
            .skip(max_idx.expect("No max idx") + 1)
            .max()
            .copied();

        let joltage: u32 =
            max_val.expect("No max val") * 10 + second_value.expect("No second value");

        self.joltage = Some(joltage as u64);
        self.joltage
    }
  
    fn get_joltage_2(&mut self) -> Option<u64> {
        let k = 12;
        let n = self.batteries.len();
        if n < k {
            dbg!("Pack is too short");
            return None;
        }
    
        // Number of elements we are allowed to remove (n - k)
        let mut can_drop = n - k;
        let mut stack: Vec<u32> = Vec::with_capacity(k);
    
        for &digit in &self.batteries {
            // Pop smaller elements while we still can drop and current digit is larger
            while let Some(&last) = stack.last() {
                if can_drop == 0 { break; }
                if last < digit {
                    stack.pop();
                    can_drop -= 1;
                } else {
                    break;
                }
            }
            stack.push(digit);
        }
    
        // Keep only first k digits (stack may be longer if we never popped enough)
        stack.truncate(k);
    
        // Build string and parse to u64 (digits are 0..=9)
        let as_string: String = stack
            .into_iter()
            .map(|d| std::char::from_digit(d, 10).expect("digit out of range"))
            .collect();
    
        let value = as_string.parse::<u64>().expect("can't parse joltage");
        self.joltage = Some(value);
        self.joltage
    }

}

pub fn main(input: &str) -> u64 {
    let input_list = reuse::split_input_by_line(input);
    let mut power_counter: u64 = 0;
    for line in input_list.iter() {
        let mut bank: Bank = Bank::new(line);
        power_counter += bank.get_joltage_2().expect("No joltage");
    }
    power_counter
}
