use std::collections::HashSet;
use std::mem;
use std::ops::Deref;
use std::str::FromStr;

struct Map {
    knot_pos: Vec<(i32, i32)>,
}

fn reconcile(head_pos: &(i32, i32), tail_pos: &(i32, i32)) -> (i32, i32) {
    let mut new_tail_pos = tail_pos.clone();
    let delta_x = head_pos.0 - tail_pos.0;
    let delta_y = head_pos.1 - tail_pos.1;
    if delta_x.abs() > 1 {
        new_tail_pos.0 += delta_x.signum();
        if delta_y.abs() > 0 {
            new_tail_pos.1 += delta_y.signum();
        }
    } else if delta_y.abs() > 1 {
        new_tail_pos.1 += delta_y.signum();
        if delta_x.abs() > 0 {
            new_tail_pos.0 += delta_x.signum();
        }
    }
    new_tail_pos
}

impl Map {
    fn new(knot_count: u32) -> Self {
        let mut knot_pos = Vec::new();
        for _ in 0..knot_count {
            knot_pos.push((0, 0));
        }
        Map { knot_pos }
    }

    fn reconcile(&mut self) -> (i32, i32) {
        let mut new_tail_pos = (0, 0);
        for i in 0..(self.knot_pos.len() - 1) {
            let head_pos = self.knot_pos[i];
            let tail_pos = self.knot_pos[i + 1];
            new_tail_pos = reconcile(&head_pos, &tail_pos);
            let _ = mem::replace(&mut self.knot_pos[i + 1], new_tail_pos);
        }
        new_tail_pos
    }

    fn move_up(&mut self) -> (i32, i32) {
        let head_pos = self.knot_pos[0];
        let new_head_pos = (head_pos.0, head_pos.1 + 1);
        let _ = mem::replace(&mut self.knot_pos[0], new_head_pos);
        self.reconcile()
    }
    fn move_down(&mut self) -> (i32, i32) {
        let head_pos = self.knot_pos[0];
        let new_head_pos = (head_pos.0, head_pos.1 - 1);
        let _ = mem::replace(&mut self.knot_pos[0], new_head_pos);
        self.reconcile()
    }
    fn move_left(&mut self) -> (i32, i32) {
        let head_pos = self.knot_pos[0];
        let new_head_pos = (head_pos.0 - 1, head_pos.1);
        let _ = mem::replace(&mut self.knot_pos[0], new_head_pos);
        self.reconcile()
    }
    fn move_right(&mut self) -> (i32, i32) {
        let head_pos = self.knot_pos[0];
        let new_head_pos = (head_pos.0 + 1, head_pos.1);
        let _ = mem::replace(&mut self.knot_pos[0], new_head_pos);
        self.reconcile()
    }
}

struct MapTracker {
    map: Map,
    tail_pos_set: HashSet<(i32, i32)>,
}

impl MapTracker {
    fn new(knot_count: u32) -> Self {
        let mut tail_pos_set = HashSet::new();
        tail_pos_set.insert((0, 0));
        MapTracker {
            map: Map::new(knot_count),
            tail_pos_set,
        }
    }

    fn move_up(&mut self, num: u32) {
        for _ in 0..num {
            self.tail_pos_set.insert(self.map.move_up());
        }
    }

    fn move_down(&mut self, num: u32) {
        for _ in 0..num {
            self.tail_pos_set.insert(self.map.move_down());
        }
    }

    fn move_left(&mut self, num: u32) {
        for _ in 0..num {
            self.tail_pos_set.insert(self.map.move_left());
        }
    }

    fn move_right(&mut self, num: u32) {
        for _ in 0..num {
            self.tail_pos_set.insert(self.map.move_right());
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let mut map_tracker = MapTracker::new(2);
    for line in input.lines() {
        let line_split = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(2, line_split.len());
        let direction = line_split.get(0).unwrap().deref();
        let num = u32::from_str(line_split.get(1).unwrap()).unwrap();
        match direction {
            "U" => map_tracker.move_up(num),
            "D" => map_tracker.move_down(num),
            "L" => map_tracker.move_left(num),
            "R" => map_tracker.move_right(num),
            _ => panic!("Unexpected direction: '{}'", direction),
        }
    }
    map_tracker.tail_pos_set.len() as u32
}

pub fn solve2(input: &str) -> u32 {
    let mut map_tracker = MapTracker::new(10);
    for line in input.lines() {
        let line_split = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(2, line_split.len());
        let direction = line_split.get(0).unwrap().deref();
        let num = u32::from_str(line_split.get(1).unwrap()).unwrap();
        match direction {
            "U" => map_tracker.move_up(num),
            "D" => map_tracker.move_down(num),
            "L" => map_tracker.move_left(num),
            "R" => map_tracker.move_right(num),
            _ => panic!("Unexpected direction: '{}'", direction),
        }
    }
    map_tracker.tail_pos_set.len() as u32
}

#[test]
fn test_solve() {
    let input = r#"
R 4
U 4
"#
    .trim();
    assert_eq!(7, solve(input));

    let input = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#
    .trim();
    assert_eq!(13, solve(input));
}

#[test]
fn test_solve2() {
    let input = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#
    .trim();
    assert_eq!(36, solve2(input));
}
