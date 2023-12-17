// Cube Conundrum

use common::read_input_as_string;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_GAME_INDEX: Regex = Regex::new(r"Game ([0-9]+)").unwrap();
    static ref REGEX_RED: Regex = Regex::new(r"([0-9]+) red").unwrap();
    static ref REGEX_GREEN: Regex = Regex::new(r"([0-9]+) green").unwrap();
    static ref REGEX_BLUE: Regex = Regex::new(r"([0-9]+) blue").unwrap();
}

type Rgb = (u32, u32, u32);

fn extract_number_from_regex(re: &Regex, haystack: &str) -> Option<u32> {
    let caps = re.captures(haystack)?;
    let group_str = caps.get(1)?;
    group_str.as_str().parse().ok()
}

fn parse_line(line: &str) -> Option<(u32, Vec<Rgb>)> {
    let game_index = extract_number_from_regex(&REGEX_GAME_INDEX, line)?;

    let mut draws: Vec<_> = Vec::default();
    for draw_str in line.split(':').nth(1).unwrap().split(';') {
        let red = extract_number_from_regex(&REGEX_RED, draw_str).unwrap_or(0);
        let green = extract_number_from_regex(&REGEX_GREEN, draw_str).unwrap_or(0);
        let blue = extract_number_from_regex(&REGEX_BLUE, draw_str).unwrap_or(0);
        draws.push((red, green, blue));
    }

    Some((game_index, draws))
}

fn cubes_needed(draws: &[Rgb]) -> Rgb {
    let red = draws.iter().max_by_key(|x| x.0).unwrap().0;
    let green = draws.iter().max_by_key(|x| x.1).unwrap().1;
    let blue = draws.iter().max_by_key(|x| x.2).unwrap().2;
    (red, green, blue)
}

fn sufficient_cubes(draws: &[Rgb], cubes: Rgb) -> bool {
    let (d_red, d_green, d_blue) = cubes_needed(draws);
    let (c_red, c_green, c_blue) = cubes;
    d_red <= c_red && d_green <= c_green && d_blue <= c_blue
}

fn power(draws: &[Rgb]) -> u32 {
    let (red, green, blue) = cubes_needed(draws);
    red * green * blue
}

fn sum_possible_games(lines: &str) -> u32 {
    let mut total = 0;
    let have_cubes = (12, 13, 14);
    for line in lines.split('\n') {
        if let Some((game_index, draws)) = parse_line(line) {
            if sufficient_cubes(&draws, have_cubes) {
                total += game_index;
            }
        }
    }

    total
}

fn sum_power(lines: &str) -> u32 {
    let mut total = 0;
    for line in lines.split('\n') {
        if let Some((_, draws)) = parse_line(line) {
            total += power(&draws);
        }
    }

    total
}

fn main() {
    let games = read_input_as_string(2023, 2).unwrap();
    println!("{}", sum_possible_games(&games));
    println!("{}", sum_power(&games));
}

#[cfg(test)]
mod tests {
    use super::{sum_possible_games, sum_power};

    const SAMPLE: &str = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test() {
        assert_eq!(8, sum_possible_games(SAMPLE));
        assert_eq!(2286, sum_power(SAMPLE));
    }
}
