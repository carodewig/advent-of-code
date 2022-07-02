// day 6: signals and noise

use advent_rs_2016::read_input::read_file;
use std::collections::HashMap;

fn decode_messages(messages: String, most_freq: bool) -> String {
    let mut char_frequency: HashMap<usize, HashMap<char, usize>> = HashMap::new();
    let mut message_length: usize = 0;

    for message in messages.split('\n') {
        message_length = message.len();
        for (index, letter) in message.chars().enumerate() {
            *char_frequency
                .entry(index)
                .or_insert_with(HashMap::new)
                .entry(letter)
                .or_insert(0) += 1;
        }
    }

    let mut decoded_message = String::new();
    for index in 0..message_length {
        let mut letter_freq: Vec<(char, usize)> =
            char_frequency.remove(&index).unwrap().drain().collect();
        letter_freq.sort_by_key(|(_, freq)| *freq);

        if most_freq {
            decoded_message.push(letter_freq.last().unwrap().0);
        } else {
            decoded_message.push(letter_freq.first().unwrap().0);
        }
    }

    decoded_message
}

fn main() {
    // umcvzsmw
    println!("{}", decode_messages(read_file("06.txt"), true));
    // rwqoacfz
    println!("{}", decode_messages(read_file("06.txt"), false));
}

#[cfg(test)]
mod tests {
    use super::decode_messages;

    #[test]
    fn test_decode_messages() {
        let messages = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!("easter", &decode_messages(String::from(messages), true));
        assert_eq!("advent", &decode_messages(String::from(messages), false));
    }
}
