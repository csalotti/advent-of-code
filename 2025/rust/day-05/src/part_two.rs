use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn is_in(self: &Self, u: u64) -> bool {
        (u >= self.start) && (u <= self.end)
    }
    fn capacity(self: &Self) -> u64 {
        self.end - self.start + 1
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("-")
            .expect(format!("{s} is not a range").as_str());

        Ok(Range {
            start: start
                .parse::<u64>()
                .expect(format!("start value {start} isn't u64").as_str()),
            end: end
                .parse::<u64>()
                .expect(format!("end value {end} isn't u64").as_str()),
        })
    }
}

fn process_ranges(ranges: &str) -> Vec<Range> {
    let mut ranges_parsed: Vec<Range> = ranges
        .split("\n")
        .map(|l| l.trim().parse::<Range>().unwrap())
        .collect();

    ranges_parsed.sort_by(|a, b| b.start.cmp(&a.start));
    let mut ranges_merged: Vec<Range> = Vec::new();

    let mut curr = ranges_parsed.pop().unwrap();
    while let Some(range) = ranges_parsed.pop() {
        if range.start <= curr.end {
            curr.end = curr.end.max(range.end);
        } else {
            ranges_merged.push(curr.clone());
            curr = range;
        }
    }
    ranges_merged.push(curr);

    ranges_merged
}

pub fn process(input: &'static str) -> u64 {
    let (freshness_ranges, _) = input.split_once("\n\n").unwrap();
    let freshness_ranges_merged = process_ranges(freshness_ranges);
    freshness_ranges_merged.iter().map(|r| r.capacity()).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_05_part_two() {
        let input = "3-5
                                   10-14
                                   16-20
                                   12-18

                                   1
                                   5
                                   8
                                   11
                                   17
                                   32";
        assert_eq!(process(input), 14);
    }
}
