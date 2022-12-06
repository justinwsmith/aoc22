struct FlowCache<const N: usize> {
    bytes: [u8; N],
    cnt: usize,
}

impl<const N: usize> FlowCache<N> {
    fn new() -> Self {
        FlowCache {
            bytes: [0; N],
            cnt: 0,
        }
    }

    fn push(&mut self, b: u8) {
        if self.cnt < N {
            self.bytes[self.cnt] = b;
            self.cnt += 1;
        } else {
            self.bytes.copy_within(1..N, 0);
            self.bytes[N - 1] = b;
        }
    }

    fn all_unique(&self) -> bool {
        if self.cnt < N {
            return false;
        }

        for i in 0..(N - 1) {
            for j in (i + 1)..N {
                if self.bytes[i] == self.bytes[j] {
                    return false;
                }
            }
        }
        true
    }
}

pub fn solve(input: &str) -> u32 {
    let mut fc: FlowCache<4> = FlowCache::new();
    let mut cnt = 1;
    for b in input.bytes() {
        fc.push(b);
        if fc.all_unique() {
            return cnt;
        }
        cnt += 1;
    }
    panic!("Marker not detected!");
}

pub fn solve2(input: &str) -> u32 {
    let mut fc: FlowCache<14> = FlowCache::new();
    let mut cnt = 1;
    for b in input.bytes() {
        fc.push(b);
        if fc.all_unique() {
            return cnt;
        }
        cnt += 1;
    }
    panic!("Marker not detected!");
}

#[test]
fn test_flow_cache() {
    let mut fc: FlowCache<4> = FlowCache::new();
    fc.push(b'a');
    fc.push(b'b');
    assert!(!fc.all_unique());
    fc.push(b'c');
    assert!(!fc.all_unique());
    fc.push(b'a');
    assert!(!fc.all_unique());
    fc.push(b'b');
    assert!(!fc.all_unique());
    fc.push(b'd');
    assert!(fc.all_unique());
    fc.push(b'a');
    assert!(!fc.all_unique());
}

#[test]
fn test_solve() {
    assert_eq!(5, solve("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, solve("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

#[test]
fn test_solve2() {
    assert_eq!(19, solve2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, solve2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, solve2("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, solve2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, solve2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}
