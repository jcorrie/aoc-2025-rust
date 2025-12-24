use crate::reuse;

const MAX_VALUE: u32 = 99;
const MIN_VALUE: u32 = 0;

#[derive(Debug)]
struct Dial {
    dial: u32,
    counter: i32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial {
            dial: 50,
            counter: 0,
        }
    }

    fn increment_right(&mut self, increment_amount: u32) {
        for _ in 0..increment_amount {
            if self.dial == MAX_VALUE {
                self.dial = MIN_VALUE
            } else {
                self.dial += 1
            }
            if self.dial == 0 {
                self.counter += 1
            }
        }
    }

    fn increment_left(&mut self, increment_amount: u32) {
        // Convert to i32 to handle negative values properly
        for _ in 0..increment_amount {
            if self.dial == MIN_VALUE {
                self.dial = MAX_VALUE
            } else {
                self.dial -= 1
            }

            if self.dial == 0 {
                self.counter += 1
            }
        }
    }
    pub fn parse_instruction(&mut self, instruction: &str) {
        let instruction = instruction.trim();
        if instruction.is_empty() {
            return;
        }
        let direction: &str = instruction.get(0..1).expect("Error parsing");
        let increment_amount: u32 =
            reuse::string_to_i32(instruction.get(1..).expect("Error parsing.")) as u32;
        dbg!(direction);
        dbg!(increment_amount);
        match direction {
            "L" => self.increment_left(increment_amount),
            "R" => self.increment_right(increment_amount),
            _ => panic!("Unexpected direction"),
        }
    }
}

pub fn main(input: &str) -> i32 {
    let mut dial = Dial::new();
    let input_list = reuse::split_input_by_line(input);
    for item in input_list.iter() {
        dial.parse_instruction(item);
        dbg!(&dial);
    }
    dial.counter as i32
}
