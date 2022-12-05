/// day 4: camp cleanup
use common::read_input_as_string;
use regex::{Match, Regex};

fn fully_contains(min1: u32, max1: u32, min2: u32, max2: u32) -> bool {
    let f = |min_a, min_b, max_a, max_b| min_a <= min_b && max_b <= max_a;
    f(min1, min2, max1, max2) || f(min2, min1, max2, max1)
}

fn any_overlap(min1: u32, max1: u32, min2: u32, max2: u32) -> bool {
    let f = |min_a, min_b, max_a| min_a <= min_b && min_b <= max_a;
    f(min1, min2, max1) || f(min2, min1, max2)
}

fn overlapping_work(input: &str, overlap_fn: fn(u32, u32, u32, u32) -> bool) -> usize {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let get_match = |m: Option<Match>| u32::from_str_radix(m.unwrap().as_str(), 10).unwrap();

    input
        .trim()
        .split('\n')
        .filter_map(|line| {
            if let Some(caps) = re.captures(line) {
                Some(overlap_fn(
                    get_match(caps.get(1)),
                    get_match(caps.get(2)),
                    get_match(caps.get(3)),
                    get_match(caps.get(4)),
                ))
            } else {
                None
            }
        })
        .filter(|x| *x)
        .count()
}

fn main() {
    let input = read_input_as_string(2022, 4).unwrap();
    println!("{}", overlapping_work(&input, fully_contains));
    println!("{}", overlapping_work(&input, any_overlap));
}

#[cfg(test)]
mod test {
    use super::{any_overlap, fully_contains, overlapping_work};
    const SAMPLE: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test() {
        assert_eq!(overlapping_work(SAMPLE, fully_contains), 2);
        assert_eq!(overlapping_work(SAMPLE, any_overlap), 4);
    }
}
