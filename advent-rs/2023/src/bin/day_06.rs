// Wait for it

use common::read_input_as_string;

mod part1 {
    fn parse(records: &str) -> Vec<(u32, u32)> {
        let time_str = records.split('\n').nth(0).unwrap();
        let distance_str = records.split('\n').nth(1).unwrap();

        let str_parser = |s: &str| {
            s.split_whitespace()
                .skip(1)
                .filter_map(|x| x.parse().ok())
                .collect()
        };

        let times: Vec<u32> = str_parser(time_str);
        let distances: Vec<u32> = str_parser(distance_str);
        times.into_iter().zip(distances.into_iter()).collect()
    }

    fn number_of_ways_to_beat_record(race_time: u32, record_distance: u32) -> u32 {
        let mut wins = 0;
        let mut hold_button_time = 1;
        while hold_button_time < race_time {
            let distance_travelled = (race_time - hold_button_time) * hold_button_time;
            if distance_travelled > record_distance {
                wins += 1;
            }
            hold_button_time += 1;
        }
        wins
    }

    pub fn multiplied_record_beaters(records_str: &str) -> u32 {
        parse(records_str)
            .into_iter()
            .map(|(t, d)| number_of_ways_to_beat_record(t, d))
            .fold(1, |acc, e| acc * e)
    }
}

mod part2 {
    use itertools::Itertools;

    fn parse(records: &str) -> (f64, f64) {
        let time_str = records.split('\n').nth(0).unwrap();
        let distance_str = records.split('\n').nth(1).unwrap();

        let str_parser = |s: &str| s.split_whitespace().skip(1).join("").parse().unwrap();

        (str_parser(time_str), str_parser(distance_str))
    }

    /// Some initial thoughts -- this chart (f(x) = (N - x) * x) is just an upside-down parabola
    /// That means we can take a derivative and figure out where the maximum distance is (f'(x) = 0), then
    /// binary search from there
    /// Alternatively, we can also tweak the equation so that we just need to figure out where f(x) = 0 to find
    /// min/max button hold time!
    ///     f(x) = (race_time - x) * x - record_distance = Tx - x^2 - D => crossing point is where this eq 0
    ///     quadratic -> a=-1, b=T, c=-D
    ///     f(x) = 0 where x = (T +/- sqrt(T^2 - 4D)) / (2)
    pub fn number_of_ways_to_beat_record(record: &str) -> u32 {
        let (race_time, record_distance) = parse(record);
        let b1 = (race_time - (race_time.powi(2) - (4.0 * record_distance)).powf(0.5)) / 2.0;
        let b2 = (race_time + (race_time.powi(2) - (4.0 * record_distance)).powf(0.5)) / 2.0;

        (b2.floor() - b1.ceil()) as u32 + 1
    }
}

fn main() {
    let records_str = read_input_as_string(2023, 6).unwrap();
    println!("{:?}", part1::multiplied_record_beaters(&records_str));
    println!("{:?}", part2::number_of_ways_to_beat_record(&records_str));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test() {
        assert_eq!(288, part1::multiplied_record_beaters(SAMPLE));
        assert_eq!(71503, part2::number_of_ways_to_beat_record(SAMPLE));
    }
}
