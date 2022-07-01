// day 3: squares with three sides
use advent_rs_2016::read_input::read_file;

fn triangle_is_possible(mut sides: Vec<u32>) -> bool {
    sides.sort_unstable();
    sides[0] + sides[1] > sides[2]
}

fn possible_triangles(triangles: &[Vec<u32>]) -> usize {
    triangles
        .iter()
        .filter(|sides| triangle_is_possible((*sides).clone()))
        .count()
}

fn parse_triangles_part1<S: AsRef<str>>(triangles: S) -> Vec<Vec<u32>> {
    triangles
        .as_ref()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn parse_triangles_part2<S: AsRef<str>>(triangles: S) -> Vec<Vec<u32>> {
    let mut parsed_triangles: Vec<Vec<u32>> = Vec::new();
    let mut sides: Vec<u32> = Vec::new();

    for column in 0..3 {
        for line in triangles
            .as_ref()
            .split('\n')
            .filter(|line| !line.is_empty())
        {
            let side = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[column];

            sides.push(side);
            if sides.len() == 3 {
                parsed_triangles.push(sides);
                sides = Vec::new();
            }
        }
    }

    parsed_triangles
}

fn main() {
    let triangles = read_file("03.txt");
    println!("{}", possible_triangles(&parse_triangles_part1(&triangles)));
    println!("{}", possible_triangles(&parse_triangles_part2(&triangles)));
}

#[cfg(test)]
mod tests {
    use super::{parse_triangles_part1, parse_triangles_part2, triangle_is_possible};

    #[test]
    fn test_triangle_is_possible() {
        assert!(!triangle_is_possible(Vec::from([123, 4, 12])));
        assert!(!triangle_is_possible(Vec::from([16, 4, 12])));
        assert!(triangle_is_possible(Vec::from([15, 4, 12])));
    }

    #[test]
    fn test_parse_triangles_part1() {
        assert_eq!(
            Vec::from([
                Vec::from([101, 301, 501]),
                Vec::from([102, 302, 502]),
                Vec::from([103, 303, 503])
            ]),
            parse_triangles_part1("101 301 501\n102 302 502\n103 303 503")
        );
    }

    #[test]
    fn test_parse_triangles_part2() {
        assert_eq!(
            Vec::from([
                Vec::from([101, 102, 103]),
                Vec::from([301, 302, 303]),
                Vec::from([501, 502, 503])
            ]),
            parse_triangles_part2("101 301 501\n102 302 502\n103 303 503")
        );
    }
}
