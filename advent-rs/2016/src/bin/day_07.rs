// day 7: internet protocol version 7
use advent_rs_2016::read_input::read_file;
use std::collections::HashSet;

fn is_abba(substr: &str) -> bool {
    let char_hashset: HashSet<char> = substr.chars().collect();
    let reversed_substr: String = substr.chars().rev().collect::<String>();

    substr == &reversed_substr && char_hashset.len() > 1
}

fn contains_abba(possible_abba: &str) -> bool {
    for start in 0..(possible_abba.len() - 3) {
        if is_abba(&possible_abba[start..start + 4]) {
            return true;
        }
    }

    false
}

fn supports_tls(ip_address: &str) -> bool {
    let mut supernet_seqs: Vec<&str> = Vec::new();
    let mut hypernet_seqs: Vec<&str> = Vec::new();
    for (index, component) in ip_address.split(['[', ']']).enumerate() {
        // odd indexes are inside brackets (aka hypernet_seqs)
        if index % 2 == 0 {
            supernet_seqs.push(component);
        } else {
            hypernet_seqs.push(component);
        }
    }

    hypernet_seqs
        .iter()
        .filter(|component| contains_abba(component))
        .count()
        == 0
        && supernet_seqs
            .iter()
            .filter(|component| contains_abba(component))
            .count()
            > 0
}

fn main() {
    println!(
        "{}",
        read_file("07.txt")
            .split('\n')
            .filter(|ip_address| supports_tls(ip_address))
            .count()
    )
}

#[cfg(test)]
mod tests {
    use super::{contains_abba, is_abba, supports_tls};

    #[test]
    fn test_is_abba() {
        assert!(is_abba("abba"));
        assert!(!is_abba("abca"));
    }

    #[test]
    fn test_contains_abba() {
        assert!(contains_abba("abba"));
        assert!(contains_abba("abbaa"));
        assert!(!contains_abba("abca"));
        assert!(!contains_abba("ababa"));
    }

    #[test]
    fn test_supports_tls() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }
}
