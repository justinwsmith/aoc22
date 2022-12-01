use std::str::FromStr;

pub fn solve() -> u32 {
    let input = include_str!("../inputs/day1.txt");

    let mut max_elf = 0u32;
    let mut current_elf = 0u32;

    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
        } else {
            current_elf += u32::from_str(line).unwrap();
        }
    }

    max_elf
}