use std::fs;

pub fn solve(path: &str) -> i32 {
    let contents = fs::read_to_string(path).unwrap(); // call me cloudflare

    let mut state = 50;
    let mut res = 0;
    let mut rot = 0;

    contents.trim().lines().for_each(|line| {
        let ret = perm(state, line);
        state = ret.0;
        if ret.0 == 0 {
            res += 1;
        }
        rot += ret.1;
    });

    res + rot
}

fn perm(in_state: i32, line: &str) -> (i32, i32) {
    let direction = line.chars().nth(0).unwrap();
    let count = line[1..].parse::<i32>().expect("not a number");

    match direction {
        'L' => perm_left(in_state, count),
        'R' => perm_right(in_state, count),
        _ => panic!("unknown direction"),
    }
}

fn perm_left(in_state: i32, count: i32) -> (i32, i32) {
    let uncapped = in_state - count;
    let mut passed0 = uncapped.abs() / 100;
    if uncapped < 0 && in_state != 0 {
        passed0 += 1;
    }
    (
        (100 + uncapped) % 100,
        passed0
            - if uncapped % 100 == 0 && uncapped != 0 {
                1
            } else {
                0
            },
    )
}

fn perm_right(in_state: i32, count: i32) -> (i32, i32) {
    let uncapped = in_state + count;
    let passed0 = uncapped / 100;
    (
        uncapped % 100,
        passed0
            - if uncapped % 100 == 0 && uncapped > 0 {
                1
            } else {
                0
            },
    )
}

#[cfg(test)]
mod test {
    use crate::part2::{perm, perm_left, perm_right, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 6);
    }

    #[test]
    fn test_perm() {
        assert_eq!(perm(50, "L68"), (82, 1));
        assert_eq!(perm(82, "L30"), (52, 0));
        assert_eq!(perm(52, "R48"), (0, 0));
        assert_eq!(perm(0, "L5"), (95, 0));
        assert_eq!(perm(95, "R60"), (55, 1));
        assert_eq!(perm(55, "L55"), (0, 0));
        assert_eq!(perm(0, "L1"), (99, 0));
        assert_eq!(perm(99, "L99"), (0, 0));
        assert_eq!(perm(0, "R14"), (14, 0));
        assert_eq!(perm(14, "L82"), (32, 1));
    }

    #[test]
    fn test_perm_left() {
        assert_eq!(perm_left(50, 68), (82, 1));
        assert_eq!(perm_left(82, 30), (52, 0));
        assert_eq!(perm_left(0, 5), (95, 0));
        assert_eq!(perm_left(55, 55), (0, 0));
        assert_eq!(perm_left(0, 1), (99, 0));
        assert_eq!(perm_left(99, 99), (0, 0));
        assert_eq!(perm_left(14, 82), (32, 1));
        assert_eq!(perm_left(50, 150), (0, 1));
        assert_eq!(perm_left(50, 250), (0, 2));
        assert_eq!(perm_left(0, 0), (0, 0));
    }

    #[test]
    fn test_perm_right() {
        assert_eq!(perm_right(52, 48), (0, 0));
        assert_eq!(perm_right(95, 60), (55, 1));
        assert_eq!(perm_right(0, 14), (14, 0));
        assert_eq!(perm_right(50, 1000), (50, 10));
        assert_eq!(perm_right(1, 199), (0, 1));
        assert_eq!(perm_right(0, 0), (0, 0));
    }
}
