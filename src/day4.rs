use std::cmp::min;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn parse_range_str(range_str: &str) -> RangeInclusive<u32> {
    let split = range_str.split("-").collect::<Vec<&str>>();
    assert_eq!(2, split.len());
    let range_start = u32::from_str(split.get(0).unwrap()).unwrap();
    let range_end = u32::from_str(split.get(1).unwrap()).unwrap();
    (range_start..=range_end).into()
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let split = line.split(",").collect::<Vec<&str>>();
    assert_eq!(2, split.len());
    let range1 = parse_range_str(split.get(0).unwrap());
    let range2 = parse_range_str(split.get(1).unwrap());
    (range1, range2)
}

pub fn solve(input: &str) -> u32 {
    let mut count = 0u32;
    for line in input.lines() {
        let (range1, range2) = parse_line(line);
        let smallest_range_size = min(range1.clone().count(), range2.clone().count());
        let set1 = range1.collect::<HashSet<u32>>();
        let set2 = range2.collect::<HashSet<u32>>();
        if set1.intersection(&set2).count() == smallest_range_size {
            count += 1;
        }
    }

    count
}

pub fn solve2(input: &str) -> u32 {
    let mut count = 0u32;
    for line in input.lines() {
        let (range1, range2) = parse_line(line);
        let set1 = range1.collect::<HashSet<u32>>();
        let set2 = range2.collect::<HashSet<u32>>();
        if set1.intersection(&set2).count() > 0 {
            count += 1;
        }
    }

    count
}

#[test]
fn test_day4() {
    let day4_input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#
    .trim();
    assert_eq!(2, solve(day4_input));
}
