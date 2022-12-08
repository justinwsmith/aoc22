use std::str::FromStr;

struct Map(Vec<Vec<u8>>);

impl Map {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Self(map)
    }

    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    fn width(&self) -> i32 {
        self.0.get(0).unwrap().len() as i32
    }

    fn get(&self, row_num: i32, col_num: i32) -> Option<u8> {
        if let Some(row) = self.0.get(row_num as usize) {
            if let Some(val) = row.get(col_num as usize) {
                return Some(*val);
            }
        }
        None
    }

    fn scenic_up(&self, row_num: i32, col_num: i32) -> u32 {
        let val = self.get(row_num, col_num).unwrap();
        let mut i = row_num - 1;
        let j = col_num;
        let mut count = 0u32;
        while let Some(cell_val) = self.get(i, j) {
            count += 1;
            if val <= cell_val {
                break;
            }
            i -= 1;
        }
        count
    }

    fn scenic_down(&self, row_num: i32, col_num: i32) -> u32 {
        let val = self.get(row_num, col_num).unwrap();
        let mut i = row_num + 1;
        let j = col_num;
        let mut count = 0u32;
        while let Some(cell_val) = self.get(i, j) {
            count += 1;
            if val <= cell_val {
                break;
            }
            i += 1;
        }
        count
    }

    fn scenic_left(&self, row_num: i32, col_num: i32) -> u32 {
        let val = self.get(row_num, col_num).unwrap();
        let i = row_num;
        let mut j = col_num - 1;
        let mut count = 0u32;
        while let Some(cell_val) = self.get(i, j) {
            count += 1;
            if val <= cell_val {
                break;
            }
            j -= 1;
        }
        count
    }

    fn scenic_right(&self, row_num: i32, col_num: i32) -> u32 {
        let val = self.get(row_num, col_num).unwrap();
        let i = row_num;
        let mut j = col_num + 1;
        let mut count = 0u32;
        while let Some(cell_val) = self.get(i, j) {
            count += 1;
            if val <= cell_val {
                break;
            }
            j += 1;
        }
        count
    }

    fn scenic_score(&self, row_num: i32, col_num: i32) -> u32 {
        self.scenic_up(row_num, col_num)
            * self.scenic_down(row_num, col_num)
            * self.scenic_left(row_num, col_num)
            * self.scenic_right(row_num, col_num)
    }

    fn visible_up(&self, row_num: i32, col_num: i32) -> bool {
        let val = self.get(row_num, col_num).unwrap();
        let mut i = row_num - 1;
        let j = col_num;
        while let Some(cell_val) = self.get(i, j) {
            if val <= cell_val {
                return false;
            }
            i -= 1;
        }
        true
    }

    fn visible_down(&self, row_num: i32, col_num: i32) -> bool {
        let val = self.get(row_num, col_num).unwrap();
        let mut i = row_num + 1;
        let j = col_num;
        while let Some(cell_val) = self.get(i, j) {
            if val <= cell_val {
                return false;
            }
            i += 1;
        }
        true
    }

    fn visible_left(&self, row_num: i32, col_num: i32) -> bool {
        let val = self.get(row_num, col_num).unwrap();
        let i = row_num;
        let mut j = col_num - 1;
        while let Some(cell_val) = self.get(i, j) {
            if val <= cell_val {
                return false;
            }
            j -= 1;
        }
        true
    }

    fn visible_right(&self, row_num: i32, col_num: i32) -> bool {
        let val = self.get(row_num, col_num).unwrap();
        let i = row_num;
        let mut j = col_num + 1;
        while let Some(cell_val) = self.get(i, j) {
            if val <= cell_val {
                return false;
            }
            j += 1;
        }
        true
    }
}

fn build_map(input: &str) -> Map {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            let val = u8::from_str(&char.to_string()).unwrap();
            row.push(val);
        }
        map.push(row);
    }
    Map(map)
}

pub fn solve(input: &str) -> u32 {
    let mut count = 0;
    let map = build_map(input);
    for i in 0..map.height() {
        let i = i as i32;
        for j in 0..map.width() {
            let j = j as i32;
            if map.visible_up(i, j)
                || map.visible_down(i, j)
                || map.visible_left(i, j)
                || map.visible_right(i, j)
            {
                count += 1
            }
        }
    }

    count
}

pub fn solve2(input: &str) -> u32 {
    let mut max_score = 0;

    let map = build_map(input);
    for i in 0..map.height() {
        let i = i as i32;
        for j in 0..map.width() {
            let j = j as i32;
            let score = map.scenic_score(i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

#[test]
fn test_solve() {
    let input = r#"
30373
25512
65332
33549
35390
"#
    .trim();

    assert_eq!(21, solve(input));
}

#[test]
fn test_solve2() {
    let input = r#"
30373
25512
65332
33549
35390
"#
    .trim();

    let map = build_map(input);
    assert_eq!(4, map.scenic_score(1, 2));
    assert_eq!(8, map.scenic_score(3, 2));

    assert_eq!(8, solve2(input));
}
