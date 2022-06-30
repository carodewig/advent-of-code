// day 4: security through obscurity
use advent_rs_2016::read_input::read_file;
use regex::Regex;
use std::collections::HashMap;

const ROOM_PATTERN: &str = r"^([a-z\-]+)-([0-9]+)\[([a-z]{5})\]$";

#[derive(Debug, PartialEq)]
struct Room {
    letters: Vec<char>,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn parse(room_str: &str) -> Option<Self> {
        let re = Regex::new(ROOM_PATTERN).ok()?;
        let caps = re.captures(room_str)?;

        let letters = caps.get(1)?.as_str().chars().collect();
        let sector_id = caps.get(2)?.as_str().parse::<u32>().ok()?;
        let checksum = caps.get(3)?.as_str().to_string();
        Some(Self {
            letters,
            sector_id,
            checksum,
        })
    }

    fn is_real(&self) -> bool {
        let mut letter_freq_map: HashMap<char, i32> = HashMap::new();
        for letter in self.letters.iter().filter(|&c| c != &'-') {
            *letter_freq_map.entry(letter.clone()).or_insert(0) += 1;
        }

        let mut letter_freq: Vec<(char, i32)> = letter_freq_map.drain().collect();
        letter_freq.sort_by_key(|(letter_a, freq_a)| (-1 * freq_a, letter_a.clone()));

        let checksum = &letter_freq[..5].iter().map(|(l, _)| l).collect::<String>();
        checksum == &self.checksum
    }

    fn decrypt_name(&self) -> String {
        let alphabet = (b'a'..=b'z').map(|c| c as char);
        let translated_alphabet = (b'a'..=b'z')
            .map(|c| c as char)
            .cycle()
            .skip((self.sector_id % 26) as usize)
            .take(26);

        let mut char_map: HashMap<char, char> =
            HashMap::from_iter(alphabet.zip(translated_alphabet));
        char_map.insert('-', ' ');

        self.letters
            .iter()
            .map(|c| char_map.get(c).unwrap())
            .collect::<String>()
    }
}

fn main() {
    let total: u32 = read_file("04.txt")
        .split('\n')
        .filter_map(|room_str| Room::parse(room_str))
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum();
    println!("{}", total);

    read_file("04.txt")
        .split('\n')
        .filter_map(|room_str| Room::parse(room_str))
        .filter(|room| room.is_real())
        .map(|room| format!("{}: {}", room.decrypt_name(), room.sector_id))
        .filter(|s| s.contains("north"))
        .for_each(|s| println!("{}", s));
}

#[cfg(test)]
mod tests {
    use super::Room;

    #[test]
    fn test_room_parsing() {
        assert_eq!(
            Room {
                letters: "ab-c-de".chars().collect(),
                sector_id: 123,
                checksum: String::from("abcde")
            },
            Room::parse("ab-c-de-123[abcde]").unwrap()
        );
    }

    #[test]
    fn test_room_is_real() {
        assert!(Room::parse("aaaaa-bbb-z-y-x-123[abxyz]").unwrap().is_real());
        assert!(Room::parse("a-b-c-d-e-f-g-h-987[abcde]").unwrap().is_real());
        assert!(Room::parse("not-a-real-room-404[oarel]").unwrap().is_real());
        assert!(!Room::parse("totally-real-room-200[decoy]")
            .unwrap()
            .is_real());
    }

    #[test]
    fn test_room_name_decryption_works() {
        assert_eq!(
            "very encrypted name",
            &Room::parse("qzmt-zixmtkozy-ivhz-343[asdaf]")
                .unwrap()
                .decrypt_name()
        );
    }
}
