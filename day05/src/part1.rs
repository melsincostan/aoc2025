use std::fs;

pub fn solve(path: &str) -> usize {
    let input = fs::read_to_string(path).expect("could not open input");
    let parts: Vec<&str> = input.splitn(2, "\n\n").collect();
    assert_eq!(parts.len(), 2);
    let ranges: Vec<(u64, u64)> = parts[0].split("\n").map(parse_range).collect();

    parts[1]
        .split("\n")
        .filter(|raw| raw.to_owned() != "")
        .map(|raw| raw.parse::<u64>().expect("could not parse number"))
        .filter(|id| {
            for range in &ranges {
                if id >= &range.0 && id <= &range.1 {
                    return true;
                }
            }
            false
        })
        .count()
}

fn parse_range(raw: &str) -> (u64, u64) {
    let spl: Vec<&str> = raw.split("-").collect();
    assert_eq!(spl.len(), 2);
    (
        spl[0].parse().expect("could not parse range start"),
        spl[1].parse().expect("could not parse range end"),
    )
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 3);
    }
}
