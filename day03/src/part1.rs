use std::fs;

pub fn solve(input: &str) -> u32 {
    let raw = fs::read_to_string(input).expect("could not read input");
    raw.lines().map(|line| line_joltage(line)).sum()
}

fn line_joltage(line: &str) -> u32 {
    let (first_digit, first_digit_pos) =
        max_in(&line[..line.len() - 1]).expect("should have found a first part");
    let (second_digit, _) =
        max_in(&line[first_digit_pos + 1..]).expect("should have found a second digit");
    format!("{}{}", first_digit, second_digit)
        .parse()
        .expect("should be able to convert back to a number")
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
    use crate::part1::{line_joltage, max_in, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 357);
    }

    #[test]
    fn test_line_joltage() {
        assert_eq!(line_joltage("987654321111111"), 98);
        assert_eq!(line_joltage("811111111111119"), 89);
        assert_eq!(line_joltage("234234234234278"), 78);
        assert_eq!(line_joltage("818181911112111"), 92);
        assert_eq!(line_joltage("911181119111"), 99);
    }

    #[test]
    fn test_max_in() {
        assert_eq!(max_in("9876543211111111"), Some((9, 0)));
    }
}
