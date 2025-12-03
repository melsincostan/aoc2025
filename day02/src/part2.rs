use std::{collections::HashSet, fs};

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
    let mut seen: HashSet<u64> = HashSet::new();
    let mut tries: HashSet<usize> = HashSet::new();

    tries.extend(mults(num_len(start)));
    tries.extend(mults(num_len(end)));

    for i in tries.iter() {
        let mut reps: HashSet<usize> = HashSet::new();
        if num_len(start) % i == 0 {
            reps.insert(num_len(start) / i);
        }

        if num_len(end) % i == 0 {
            reps.insert(num_len(end) / i);
        }
        for rep in reps.iter() {
            for j in 10_u64.pow((i - 1).try_into().unwrap())
                ..10_u64.pow(i.to_owned().try_into().unwrap())
            {
                let num = j
                    .to_string()
                    .repeat(rep.to_owned())
                    .parse::<u64>()
                    .expect("should parse into a number");

                if num >= start && num <= end && !seen.contains(&num) {
                    seen.insert(num);
                    res.push(num);
                }
            }
        }
    }

    res
}

fn mults(num: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..(num / 2) + 1 {
        if num % i == 0 {
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
    use crate::part2::{mults, parse_range, range_matches, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 4174379265);
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
        assert_eq!(range_matches(95, 115), vec![99, 111]);
        assert_eq!(range_matches(998, 1012), vec![999, 1010]);
        assert_eq!(range_matches(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(range_matches(222220, 222224), vec![222222]);
        assert_eq!(range_matches(1698522, 1698528), vec![]);
        assert_eq!(range_matches(446443, 446449), vec![446446]);
        assert_eq!(range_matches(38593856, 38593862), vec![38593859]);
        assert_eq!(range_matches(565653, 565659), vec![565656]);
        assert_eq!(range_matches(824824821, 824824827), vec![824824824]);
        assert_eq!(range_matches(2121212118, 2121212124), vec![2121212121]);
    }

    #[test]
    fn test_mults() {
        assert_eq!(mults(6), vec![1, 2, 3]);
        assert_eq!(mults(15), vec![1, 3, 5]);
        assert_eq!(mults(16), vec![1, 2, 4, 8]);
    }
}
