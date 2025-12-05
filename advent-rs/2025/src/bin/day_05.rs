use common::read_input_as_string;
use regex::Regex;
use std::ops::RangeInclusive;

type Ingredient = usize;
type FreshIngredientRanges = RangeInclusive<usize>;

fn parse(input: &str) -> (Vec<FreshIngredientRanges>, Vec<Ingredient>) {
    let re = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();

    let mut fresh_ingredient_ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.split('\n') {
        let line = line.trim();
        if let Some(captures) = re.captures(line) {
            let start = captures.get(1).unwrap().as_str().parse().unwrap();
            let end = captures.get(2).unwrap().as_str().parse().unwrap();
            fresh_ingredient_ranges.push(start..=end);
        } else if let Ok(ingredient) = line.parse() {
            ingredients.push(ingredient);
        }
    }

    (fresh_ingredient_ranges, ingredients)
}

fn part1(fresh_ingredient_ranges: &[FreshIngredientRanges], ingredients: &[Ingredient]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ingredient_ranges
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count()
}

fn unify_ranges(fresh_ingredient_ranges: &[FreshIngredientRanges]) -> Vec<FreshIngredientRanges> {
    let mut fresh_ingredient_ranges = fresh_ingredient_ranges.to_vec();

    'outer: loop {
        for i in 0..(fresh_ingredient_ranges.len() - 1) {
            for j in (i + 1)..fresh_ingredient_ranges.len() {
                let range1 = &fresh_ingredient_ranges[i];
                let range2 = &fresh_ingredient_ranges[j];

                if range1.contains(range2.start()) || range2.contains(range1.start()) {
                    // merge overlapping ranges. push the new range to the end and remove the 2
                    // pre-existing ranges.
                    let start = range1.start().min(range2.start());
                    let end = range1.end().max(range2.end());
                    fresh_ingredient_ranges.push(*start..=*end);
                    fresh_ingredient_ranges.swap_remove(j);
                    fresh_ingredient_ranges.swap_remove(i);
                    continue 'outer;
                }
            }
        }

        // never found a range to merge, so we're done
        return fresh_ingredient_ranges;
    }
}

fn range_length(range: FreshIngredientRanges) -> usize {
    range.end() - range.start() + 1
}

fn part2(fresh_ingredient_ranges: &[FreshIngredientRanges]) -> usize {
    let merged_ranges = unify_ranges(fresh_ingredient_ranges);
    merged_ranges.into_iter().map(range_length).sum()
}

fn main() {
    let (fresh_ingredient_ranges, ingredients) = parse(&read_input_as_string(2025, 5).unwrap());
    assert_eq!(part1(&fresh_ingredient_ranges, &ingredients), 664);
    assert_eq!(part2(&fresh_ingredient_ranges), 350_780_324_308_385);
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};
    const SAMPLE: &str = "\
        3-5
        10-14
        16-20
        12-18
        12-12

        1
        5
        8
        11
        17
        32";

    #[test]
    fn test_part1() {
        let (fresh_ingredient_ranges, ingredients) = parse(SAMPLE);
        assert_eq!(part1(&fresh_ingredient_ranges, &ingredients), 3);
    }

    #[test]
    fn test_part2() {
        let (fresh_ingredient_ranges, _ingredients) = parse(SAMPLE);
        assert_eq!(part2(&fresh_ingredient_ranges), 14);
    }
}
