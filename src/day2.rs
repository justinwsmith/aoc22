use std::collections::HashMap;

fn init1() -> (HashMap<&'static str, u32>, HashMap<u8, u32>) {
    let match_scores = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    let play_scores = HashMap::from([(b'X', 1), (b'Y', 2), (b'Z', 3)]);

    (match_scores, play_scores)
}

fn init2() -> (
    HashMap<&'static str, u8>,
    HashMap<u8, u32>,
    HashMap<u8, u32>,
) {
    let play_responses = HashMap::from([
        ("A X", b'C'),
        ("A Y", b'A'),
        ("A Z", b'B'),
        ("B X", b'A'),
        ("B Y", b'B'),
        ("B Z", b'C'),
        ("C X", b'B'),
        ("C Y", b'C'),
        ("C Z", b'A'),
    ]);

    let match_scores = HashMap::from([(b'X', 0), (b'Y', 3), (b'Z', 6)]);
    let play_scores = HashMap::from([(b'A', 1), (b'B', 2), (b'C', 3)]);

    (play_responses, match_scores, play_scores)
}

pub fn solve() -> u32 {
    let input = include_str!("../inputs/day2.txt");

    let (match_scores, play_scores) = init1();

    let mut score = 0u32;
    for line in input.lines() {
        let my_play = line.as_bytes()[line.len() - 1];
        score += play_scores
            .get(&my_play)
            .expect(format!("Unsupported outcome: '{}'", line).as_str());
        score += match_scores.get(line).unwrap();
    }

    score
}

pub fn solve2() -> u32 {
    let input = include_str!("../inputs/day2.txt");

    let (play_responses, match_scores, play_scores) = init2();

    let mut score = 0u32;
    for line in input.lines() {
        let my_play = play_responses.get(line).unwrap();
        let match_outcome = line.as_bytes()[line.len() - 1];

        score += match_scores.get(&match_outcome).unwrap();
        score += play_scores
            .get(&my_play)
            .expect(format!("Unsupported outcome: '{}'", line).as_str());
    }

    score
}
