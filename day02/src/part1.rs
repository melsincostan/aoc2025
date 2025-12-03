use std::fs;

pub fn solve(path: &str) -> u64 {
    let content = fs::read_to_string(path).unwrap();
    content
        .trim()
        .split(',')
        .filter(|range| range.to_owned() != "")
        .map(|range| process_range(range.trim()))
        .sum()
}

fn process_range(range: &str) -> u64 {
    let (start, end) = parse_range(range);
    range_matches(start, end).iter().sum()
}

fn parse_range(range: &str) -> (u64, u64) {
    let spl = range.split('-').collect::<Vec<&str>>();
    if spl.len() != 2 {
        panic!("range doesn't have exactly one start and one end");
    }
    let start = spl[0].parse::<u64>().expect("could not parse start to u64");
    let end = spl[1].parse::<u64>().expect("could not parse end to u64");
    assert!(start <= end);
    (start, end)
}

fn range_matches(start: u64, end: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];

    for i in start..(end + 1) {
        if !num_len(i) % 2 == 0 {
            continue;
        }
        // cut the number in two. if the "upper half" is equal to the "lower half", then this works
        // this really only works because it's only if it's a repeating structure
        // first attempt was doing any kind of repeating pattern, so it found too much. But it's maybe more efficient for wider ranges.
        if i.to_string()[..(num_len(i) / 2).try_into().unwrap()]
            == i.to_string()[(num_len(i) / 2).try_into().unwrap()..]
        {
            res.push(i);
        }
    }

    res
}

fn num_len(num: u64) -> usize {
    num.to_string().len()
}

#[cfg(test)]
mod test {
    use crate::part1::{parse_range, range_matches, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1227775554);
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("11-22"), (11, 22));
        assert_eq!(parse_range("95-115"), (95, 115));
        assert_eq!(parse_range("998-1012"), (998, 1012));
        assert_eq!(
            parse_range("1188511880-1188511890"),
            (1188511880, 1188511890)
        )
    }

    #[test]
    fn test_range_matches() {
        assert_eq!(range_matches(11, 22), vec![11, 22]);
        assert_eq!(range_matches(95, 115), vec![99]);
        assert_eq!(range_matches(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(range_matches(222220, 222224), vec![222222]);
        assert_eq!(range_matches(1698522, 1698528), vec![]);
        assert_eq!(range_matches(446443, 446449), vec![446446]);
        assert_eq!(range_matches(38593856, 38593862), vec![38593859]);
        assert_eq!(range_matches(565653, 565659), vec![]);
        assert_eq!(range_matches(824824821, 824824827), vec![]);
        assert_eq!(range_matches(2121212118, 2121212124), vec![]);
    }
}
