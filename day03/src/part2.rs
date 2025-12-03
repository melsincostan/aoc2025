use std::fs;

pub fn solve(input: &str) -> u64 {
    let raw = fs::read_to_string(input).expect("could not read input");
    raw.lines().map(|line| line_joltage(line)).sum()
}

fn line_joltage(line: &str) -> u64 {
    let mut start_pos = 0;
    let mut res = "".to_string();
    for i in (0..12).rev() {
        let (val, pos) =
            max_in(&line[start_pos..line.len() - i]).expect("should have found a value");
        start_pos = start_pos + pos + 1;
        res = format!("{}{}", res, val);
    }
    res.parse().expect("should all sum up to a number")
}

fn max_in(part: &str) -> Option<(u32, usize)> {
    let mut max_val = 0;
    let mut max_val_pos: i32 = -1;
    part.char_indices().for_each(|(idx, char)| {
        let val = char
            .to_digit(10)
            .expect("all chars should be parseable to base 10 digits");
        if val > max_val {
            max_val = val;
            max_val_pos = idx.try_into().expect("should fit in i32");
        }
    });
    if max_val_pos == -1 {
        None
    } else {
        Some((
            max_val,
            max_val_pos.try_into().expect("should fit into usize"),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::part2::{line_joltage, max_in, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 3121910778619);
    }

    #[test]
    fn test_line_joltage() {
        assert_eq!(line_joltage("987654321111111"), 987654321111);
        assert_eq!(line_joltage("811111111111119"), 811111111119);
        assert_eq!(line_joltage("234234234234278"), 434234234278);
        assert_eq!(line_joltage("818181911112111"), 888911112111);
    }

    #[test]
    fn test_max_in() {
        assert_eq!(max_in("9876543211111111"), Some((9, 0)));
    }
}
