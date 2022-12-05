use std::collections::HashSet;

fn setify(comp: &str) -> HashSet<u8> {
    let mut set = HashSet::new();
    for byte in comp.as_bytes() {
        set.insert(byte.to_owned());
    }
    set
}

fn value_for(byte: u8) -> u32 {
    if byte >= b'a' && byte <= b'z' {
        (1 + byte - b'a') as u32
    } else {
        (27 + byte - b'A') as u32
    }
}

pub fn solve(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let comp1set = setify(comp1);
        let comp2set = setify(comp2);
        let mut intersection = comp1set.intersection(&comp2set);
        let common = intersection.nth(0).unwrap();
        sum += value_for(*common)
    }

    sum
}

pub fn solve2(input: &str) -> u32 {
    let mut state = 0;
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut set3 = HashSet::new();
    let mut sum = 0;
    for line in input.lines() {
        if state == 0 {
            set1 = setify(line);
        } else if state == 1 {
            set2 = setify(line);
        } else {
            set3 = setify(line);
            let common: Vec<u8> = set1
                .iter()
                .map(|b| b.to_owned())
                .filter(|b| set2.contains(b))
                .filter(|b| set3.contains(b))
                .map(|b| b.to_owned())
                .collect::<Vec<u8>>();
            assert_eq!(1, common.len());
            let common = common.get(0).unwrap();
            sum += value_for(*common);
        }
        state = (state + 1) % 3;
    }

    sum
}

#[test]
fn day3_test() {
    let day3_test_input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#
    .trim();
    assert_eq!(solve(day3_test_input), 157);
}
