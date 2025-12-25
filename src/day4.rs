use crate::reuse;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    is_roll: AtomicBool,
    part_2_categorisation: Arc<Mutex<Option<bool>>>,
}

const DELTA: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
#[derive(Debug)]
struct Floorplan {
    rolls: Vec<Vec<Position>>,
}

impl Floorplan {
    fn new(input: &str) -> Floorplan {
        let rows: Vec<String> = reuse::split_input_by_line(input);
        let items: Vec<Vec<Position>> = rows
            .iter()
            .enumerate()
            .map(|(y_index, value)| {
                {
                    value
                        .chars()
                        .enumerate()
                        .map(|(x_index, character)| match character {
                            '@' => Position {
                                x: x_index,
                                y: y_index,
                                is_roll: AtomicBool::new(true),
                                part_2_categorisation: Arc::new(Mutex::new(None)),
                            },
                            _ => Position {
                                x: x_index,
                                y: y_index,
                                is_roll: AtomicBool::new(false),
                                part_2_categorisation: Arc::new(Mutex::new(None)),
                            },
                        })
                }
                .collect()
            })
            .collect();
        Floorplan { rolls: items }
    }
    fn part_1_categorise(&mut self) -> usize {
        let mut true_rolls: usize = 0;
        for row in self.rolls.iter() {
            for position in row.iter() {
                let mut counter: u8 = 0;
                for (delta_y, delta_x) in DELTA.iter() {
                    let value = self
                        .rolls
                        .get((position.y as isize + delta_y) as usize)
                        .and_then(|x| x.get((position.x as isize + delta_x) as usize))
                        .map(|x| x.is_roll.load(Ordering::Relaxed));
                    match value {
                        None => {
                            continue;
                        }
                        Some(v) => match v {
                            true => counter += 1,
                            false => {
                                continue;
                            }
                        },
                    }
                }
                if counter < 4 && position.is_roll.load(Ordering::Relaxed) == true {
                    true_rolls += 1;
                }
            }
        }
        true_rolls
    }

    fn part_2_categorise(&mut self) -> usize {
        let mut further_iterations_flag: bool = true;
        let mut true_rolls: usize = 0;
        while further_iterations_flag {
            further_iterations_flag = false;
            for row in self.rolls.iter() {
                for position in row.iter() {
                    let mut counter: u8 = 0;
                    for (delta_y, delta_x) in DELTA.iter() {
                        let value = self
                            .rolls
                            .get((position.y as isize + delta_y) as usize)
                            .and_then(|x| x.get((position.x as isize + delta_x) as usize))
                            .map(|x| x.is_roll.load(Ordering::Relaxed));
                        match value {
                            None => {
                                continue;
                            }
                            Some(v) => match v {
                                true => counter += 1,
                                false => {
                                    continue;
                                }
                            },
                        }
                    }
                    if counter < 4 && position.is_roll.load(Ordering::Relaxed) == true {
                        true_rolls += 1;
                        *position.part_2_categorisation.lock().expect("Can't access") = Some(true);
                        position.is_roll.swap(false, Ordering::Relaxed);
                        further_iterations_flag = true;
                    }
                }
            }
        }
        true_rolls
    }
}

pub fn main(input: &str) -> usize {
    let mut floorplan = Floorplan::new(input);
    let num_true_rolls = floorplan.part_2_categorise();
    dbg!(&num_true_rolls);
    num_true_rolls
}
