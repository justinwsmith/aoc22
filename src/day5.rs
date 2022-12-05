use regex::Regex;
use std::collections::HashSet;
use std::os::macos::raw::ino_t;
use std::str::FromStr;

fn count_stacks(line: &str) -> usize {
    (line.len() + 1) / 4
}

fn item_on_stack(num: usize, line: &str) -> Option<u8> {
    let str_pos = num * 4 + 1;
    let line_bytes = line.as_bytes();
    if let Some(&ch) = line_bytes.get(str_pos) {
        if ch >= b'A' && ch <= b'Z' {
            return Some(ch);
        }
    }
    None
}

fn update_stacks(line: &str, stacks: &mut Vec<Vec<u8>>) {
    let stack_count = count_stacks(line);
    while stacks.len() < stack_count {
        stacks.push(Vec::new())
    }
    for i in 0..stack_count {
        if let Some(ch) = item_on_stack(i, line) {
            let mut affected_stack = stacks.get_mut(i).unwrap();
            affected_stack.insert(0, ch)
        }
    }
}

fn get_result(stacks: Vec<Vec<u8>>) -> String {
    let mut result = String::new();
    for stack in stacks {
        if stack.is_empty() {
            result.push(' ');
        } else {
            result.push(char::from(stack.last().unwrap().to_owned()));
        }
    }
    result
}

fn move_crates(count: u32, from_stack: &mut Vec<u8>, to_stack: &mut Vec<u8>) {
    if count > from_stack.len() as u32 {
        panic!(
            "Can't move {} crates from stack with only {} crates.",
            count,
            from_stack.len()
        );
    }
    let mut count = count;
    while count > 0 {
        to_stack.push(from_stack.pop().unwrap());
        count -= 1;
    }
}

enum ParseState {
    INITIALIZE_STACK,
    MOVE_CMD,
}

pub fn solve(input: &str) -> String {
    let move_ptn = Regex::new("^move (\\d+) from (\\d) to (\\d)$").unwrap();

    let mut stacks = Vec::new();
    let mut state = ParseState::INITIALIZE_STACK;
    for line in input.lines() {
        match state {
            ParseState::INITIALIZE_STACK => {
                if line.trim().as_bytes().get(0).unwrap().to_owned() != b'[' {
                    state = ParseState::MOVE_CMD
                }
                update_stacks(line, &mut stacks);
            }
            ParseState::MOVE_CMD => {
                if !move_ptn.is_match(line) {
                    eprintln!("Unmatched line: {}", line);
                    continue;
                }
                let captures = move_ptn.captures(line).unwrap();
                let count = captures.get(1).unwrap().as_str();
                let count = u32::from_str(count).unwrap();
                let from_stack = usize::from_str(captures.get(2).unwrap().as_str()).unwrap() - 1;
                let mut from_stack = stacks.get_mut(from_stack).unwrap();
                let mut workspace = Vec::new();
                for _ in 0..count {
                    workspace.insert(0, from_stack.pop().unwrap());
                }
                let to_stack = usize::from_str(captures.get(3).unwrap().as_str()).unwrap() - 1;
                let mut to_stack = stacks.get_mut(to_stack).unwrap();
                for _ in 0..count {
                    to_stack.push(workspace.pop().unwrap());
                }
            }
        }
    }

    get_result(stacks)
}

pub fn solve2(input: &str) -> String {
    let move_ptn = Regex::new("^move (\\d+) from (\\d) to (\\d)$").unwrap();

    let mut stacks = Vec::new();
    let mut state = ParseState::INITIALIZE_STACK;
    for line in input.lines() {
        match state {
            ParseState::INITIALIZE_STACK => {
                if line.trim().as_bytes().get(0).unwrap().to_owned() != b'[' {
                    state = ParseState::MOVE_CMD
                }
                update_stacks(line, &mut stacks);
            }
            ParseState::MOVE_CMD => {
                if !move_ptn.is_match(line) {
                    eprintln!("Unmatched line: {}", line);
                    continue;
                }
                let captures = move_ptn.captures(line).unwrap();
                let count = captures.get(1).unwrap().as_str();
                let count = u32::from_str(count).unwrap();
                let from_stack = usize::from_str(captures.get(2).unwrap().as_str()).unwrap() - 1;
                let mut from_stack = stacks.get_mut(from_stack).unwrap();
                let mut workspace = Vec::new();
                for _ in 0..count {
                    workspace.push(from_stack.pop().unwrap());
                }
                let to_stack = usize::from_str(captures.get(3).unwrap().as_str()).unwrap() - 1;
                let mut to_stack = stacks.get_mut(to_stack).unwrap();
                for _ in 0..count {
                    to_stack.push(workspace.pop().unwrap());
                }
            }
        }
    }

    get_result(stacks)
}

#[test]
fn test_update_stacks() {
    let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
"#
    .strip_prefix("\n")
    .unwrap()
    .strip_suffix("\n")
    .unwrap();
    let mut stacks = Vec::new();
    for line in input.lines() {
        update_stacks(line, &mut stacks);
    }
    assert_eq!([b'Z', b'N'], stacks.get(0).unwrap().as_slice());
    assert_eq!([b'M', b'C', b'D'], stacks.get(1).unwrap().as_slice());
    assert_eq!([b'P'], stacks.get(2).unwrap().as_slice());
}

#[test]
fn test_item_on_stack() {
    let line = "[D]                     [N] [F]";
    assert_eq!(Some(b'D'), item_on_stack(0, line));
    assert_eq!(None, item_on_stack(1, line));
    assert_eq!(None, item_on_stack(2, line));
    assert_eq!(None, item_on_stack(3, line));
    assert_eq!(None, item_on_stack(4, line));
    assert_eq!(None, item_on_stack(5, line));
    assert_eq!(Some(b'N'), item_on_stack(6, line));
    assert_eq!(Some(b'F'), item_on_stack(7, line));
    assert_eq!(None, item_on_stack(8, line));
}

#[test]
fn test_solve() {
    let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
    .strip_prefix("\n")
    .unwrap()
    .strip_suffix("\n")
    .unwrap();
    let solution = solve(input);
    assert_eq!("CMZ", solution.as_str());
}

#[test]
fn test_solve2() {
    let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
    .strip_prefix("\n")
    .unwrap()
    .strip_suffix("\n")
    .unwrap();
    let solution = solve2(input);
    assert_eq!("MCD", solution.as_str());
}
