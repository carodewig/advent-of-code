// Garden Groups

use common::{Location, read_input_as_string};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let map = PlantMap::from(&read_input_as_string(2024, 12).unwrap());
    println!("{}", map.price_fencing_part1());
    println!("{}", map.price_fencing_part2());
}

fn area(region: &Region) -> usize {
    region.len()
}

fn perimeter(region: &Region) -> usize {
    let segments = fence_segments(region);
    segments.len()
}

fn fence_segments(region: &Region) -> Vec<Segment> {
    let mut segments = Vec::default();
    for plot in region {
        // index segments so that top left corner has same location as plot
        let top_left = *plot;
        let top_right = Location::new(plot.x, plot.y + 1);
        let bottom_left = Location::new(plot.x + 1, plot.y);
        let bottom_right = Location::new(plot.x + 1, plot.y + 1);

        segments.push(Segment::new(top_left, top_right));
        segments.push(Segment::new(top_right, bottom_right));
        segments.push(Segment::new(top_left, bottom_left));
        segments.push(Segment::new(bottom_left, bottom_right));
    }

    // if the same segment is in the vec twice, that means the plots are adjacent
    // and neither fence is needed
    let mut i = 0;
    while i < segments.len() - 1 {
        let mut found_match = false;
        let mut j = i + 1;
        while j < segments.len() {
            if segments[i] == segments[j] {
                segments.remove(j);
                segments.remove(i);
                found_match = true;
                break;
            }
            j += 1;
        }

        if !found_match {
            i += 1;
        }
    }

    segments
}

fn sides(region: &Region) -> usize {
    let mut segments = fence_segments(region);

    let mut changed = true;
    while changed {
        changed = false;

        // merge segments together
        let mut i = 0;
        while i < segments.len() - 1 {
            let mut j = i + 1;
            while j < segments.len() {
                let matching_directions =
                    segments[i].is_horizontal() == segments[j].is_horizontal();
                if !matching_directions {
                    j += 1;
                    continue;
                }

                if segments[i].location1 == segments[j].location2 {
                    if would_merge_plus_sign(&segments, segments[i].location1) {
                        j += 1;
                        continue;
                    }

                    segments[i].location1 = segments[j].location1;
                    segments.remove(j);
                    changed = true;
                    break;
                }

                if segments[i].location2 == segments[j].location1 {
                    if would_merge_plus_sign(&segments, segments[i].location2) {
                        j += 1;
                        continue;
                    }

                    segments[i].location2 = segments[j].location2;
                    segments.remove(j);
                    changed = true;
                    break;
                }

                j += 1;
            }

            i += 1;
        }
    }

    segments.len()
}

fn would_merge_plus_sign(segments: &[Segment], location: Location) -> bool {
    // see if there are four segments at this location
    let segments_into_this_location = segments
        .iter()
        .filter(|s| s.location1 == location || s.location2 == location)
        .count();
    segments_into_this_location == 4
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Segment {
    location1: Location,
    location2: Location,
}
impl Segment {
    fn new(location1: Location, location2: Location) -> Self {
        Self {
            location1,
            location2,
        }
    }
    fn is_horizontal(&self) -> bool {
        self.location1.y == self.location2.y
    }
}

type Region = HashSet<Location>;

#[derive(Default, Debug, Clone)]
struct PlantMap(HashMap<Location, char>);
impl PlantMap {
    fn price_fencing_part1(&self) -> usize {
        self.extract_regions()
            .values()
            .flatten()
            .map(|region| area(region) * perimeter(region))
            .sum()
    }

    fn price_fencing_part2(&self) -> usize {
        self.extract_regions()
            .values()
            .flatten()
            .map(|region| area(region) * sides(region))
            .sum()
    }

    fn extract_regions(&self) -> HashMap<char, Vec<Region>> {
        self.get_plants()
            .into_iter()
            .map(|plant| (plant, self.extract_regions_for_single_plant(plant)))
            .collect()
    }

    fn extract_regions_for_single_plant(&self, plant: char) -> Vec<Region> {
        let mut regions = Vec::default();
        let mut plots = self.get_plots(plant);

        while !plots.is_empty() {
            let mut active_region = HashSet::default();
            let mut to_search = Vec::default();

            let starting_plot = *plots.iter().next().unwrap();
            plots.remove(&starting_plot);
            to_search.push(starting_plot);

            while let Some(plot) = to_search.pop() {
                active_region.insert(plot);
                for neighbor in plot.neighbors() {
                    if plots.remove(&neighbor) {
                        to_search.push(neighbor);
                    }
                }
            }

            regions.push(active_region);
        }

        regions
    }

    fn get_plants(&self) -> Vec<char> {
        self.0.values().unique().copied().collect()
    }

    fn get_plots(&self, plant: char) -> HashSet<Location> {
        self.0
            .iter()
            .filter(|(_, p)| p == &&plant)
            .map(|(location, _)| location)
            .copied()
            .collect()
    }
}

impl<S: AsRef<str>> From<S> for PlantMap {
    #[allow(clippy::cast_possible_wrap)]
    fn from(input: S) -> Self {
        let mut map = HashMap::default();
        for (row, line) in input
            .as_ref()
            .split_whitespace()
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            for (column, char) in line.chars().enumerate() {
                map.insert(Location::new(row as isize, column as isize), char);
            }
        }

        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::PlantMap;

    const SAMPLE1: &str = "
        AAAA
        BBCD
        BBCC
        EEEC";

    const SAMPLE2: &str = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO";

    const SAMPLE3: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE";

    const SAMPLE4: &str = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE";

    const SAMPLE5: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA";

    #[test]
    fn test_part1() {
        assert_eq!(PlantMap::from(SAMPLE1).price_fencing_part1(), 140);
        assert_eq!(PlantMap::from(SAMPLE2).price_fencing_part1(), 772);
        assert_eq!(PlantMap::from(SAMPLE3).price_fencing_part1(), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(PlantMap::from(SAMPLE1).price_fencing_part2(), 80);
        assert_eq!(PlantMap::from(SAMPLE2).price_fencing_part2(), 436);
        assert_eq!(PlantMap::from(SAMPLE3).price_fencing_part2(), 1206);
        assert_eq!(PlantMap::from(SAMPLE4).price_fencing_part2(), 236);
        assert_eq!(PlantMap::from(SAMPLE5).price_fencing_part2(), 368);
    }
}
