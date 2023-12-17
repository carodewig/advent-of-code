// Trebuchet?!

use common::read_input_as_string;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Place {
    First,
    Last,
}

// keep first and last character around in case they're needed for things like 'twone' which
// should end up as 21!!
fn replace_line_chars(line: &str) -> String {
    line.to_string()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn get_digit(line: &str, place: Place) -> u32 {
    let mut numbers = line.chars().filter(char::is_ascii_digit);
    let digit = if place == Place::First {
        numbers.next()
    } else {
        numbers.last()
    };
    digit
        .expect("no digit")
        .to_digit(10)
        .expect("unparseable digit")
}

fn extract_calibration_value(document: &str, include_strs: bool) -> u32 {
    let mut value = 0;
    for line in document.split('\n') {
        let parsed_line = if include_strs {
            replace_line_chars(line)
        } else {
            line.to_string()
        };

        let first = get_digit(&parsed_line, Place::First);
        let last = get_digit(&parsed_line, Place::Last);
        value += first * 10 + last;
    }
    value
}

fn main() {
    let document = read_input_as_string(2023, 1).unwrap();
    println!("{}", extract_calibration_value(&document, false));
    println!("{}", extract_calibration_value(&document, true));
}

#[cfg(test)]
mod tests {
    use super::extract_calibration_value;
    const SAMPLE1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const SAMPLE2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    const SAMPLE3: &str = "eighthree";

    #[test]
    fn test() {
        assert_eq!(142, extract_calibration_value(SAMPLE1, false));
        assert_eq!(281, extract_calibration_value(SAMPLE2, true));
        assert_eq!(83, extract_calibration_value(SAMPLE3, true));
    }
}
