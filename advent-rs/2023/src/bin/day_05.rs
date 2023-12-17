// If you give a seed a fertilizer

use common::read_input_as_string;

#[derive(Clone, Debug)]
struct CategoryMap {
    start_src: u64,
    start_dst: u64,
    range_len: u64,
}

impl CategoryMap {
    fn from_str(line: &str) -> Option<Self> {
        let elems: Vec<u64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if elems.len() != 3 {
            return None;
        }

        Some(Self {
            start_dst: elems[0],
            start_src: elems[1],
            range_len: elems[2],
        })
    }
}

#[derive(Clone, Debug)]
struct CategoryMapSet(Vec<CategoryMap>);

impl CategoryMapSet {
    fn containing_range(&self, source: u64) -> Option<CategoryMap> {
        self.0
            .iter()
            .find(|cat_map| {
                cat_map.start_src <= source && source < cat_map.start_src + cat_map.range_len
            })
            .cloned()
    }

    fn closest_starting_value_above(&self, source: u64) -> Option<u64> {
        self.0
            .iter()
            .map(|cat_map| cat_map.start_src)
            .filter(|s| s >= &source)
            .min()
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seed_ranges: Vec<(u64, u64)>,
    map_set_chain: Vec<CategoryMapSet>,
}

impl Almanac {
    fn from_str(almanac: &str, is_part_2: bool) -> Self {
        let lines: Vec<_> = almanac.split('\n').collect();
        let mut i = 0;

        let mut seed_ranges = Vec::default();
        let mut map_set_chain = Vec::default();

        fn extract_maps(almanac_lines: &[&str], mut index: usize) -> (Vec<CategoryMap>, usize) {
            let mut cat_maps = Vec::default();
            while index < almanac_lines.len() && almanac_lines[index].trim() != "" {
                if let Some(cat_map) = CategoryMap::from_str(almanac_lines[index].trim()) {
                    cat_maps.push(cat_map);
                }
                index += 1;
            }

            (cat_maps, index)
        }

        while i < lines.len() {
            let current_line = lines[i].trim();
            if current_line.contains("seeds:") {
                let seed_values: Vec<u64> = current_line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|x| x.parse().ok())
                    .collect();
                if is_part_2 {
                    for sr in seed_values.chunks(2) {
                        seed_ranges.push((sr[0], sr[1]));
                    }
                } else {
                    for s in seed_values {
                        seed_ranges.push((s, 1));
                    }
                }
                i += 1;
                continue;
            }

            if current_line.contains("map:") {
                let (cat_maps, end_index) = extract_maps(&lines, i + 1);
                i = end_index;
                map_set_chain.push(CategoryMapSet(cat_maps));
                continue;
            }

            i += 1;
        }

        Self {
            seed_ranges,
            map_set_chain,
        }
    }

    fn lowest_location_number(&self) -> u64 {
        let mut src_ranges = self.seed_ranges.clone();
        let mut dst_ranges = Vec::default();

        for cat_map_set in &self.map_set_chain {
            // translate ranges into new ranges for next set...
            for (mut src_range_start, mut src_range_len) in src_ranges.drain(..) {
                while src_range_len > 0 {
                    // put new ranges into dst_ranges
                    if let Some(cat_map) = cat_map_set.containing_range(src_range_start) {
                        let start_dst = cat_map.start_dst + (src_range_start - cat_map.start_src);
                        let remaining_len_in_cat_map =
                            cat_map.range_len - (src_range_start - cat_map.start_src);
                        if src_range_len <= remaining_len_in_cat_map {
                            // whole remaining range will fit!
                            dst_ranges.push((start_dst, src_range_len));
                            src_range_len = 0;
                        } else {
                            // take part of src_range that fits in remaining, then handle the rest
                            dst_ranges.push((start_dst, remaining_len_in_cat_map));
                            src_range_start += remaining_len_in_cat_map;
                            src_range_len -= remaining_len_in_cat_map;
                        }
                    } else {
                        // this number isn't in the catmap... figure out a new range of things that all aren't
                        if let Some(closest_above) =
                            cat_map_set.closest_starting_value_above(src_range_start)
                        {
                            let num_matches = if closest_above - src_range_start < src_range_len {
                                closest_above - src_range_start
                            } else {
                                src_range_len
                            };
                            dst_ranges.push((src_range_start, num_matches));
                            src_range_start += num_matches;
                            src_range_len -= num_matches;
                        } else {
                            // nothing above this, so everything will map to itself
                            dst_ranges.push((src_range_start, src_range_len));
                            src_range_len = 0;
                        }
                    }
                }
            }

            src_ranges = dst_ranges.clone();
            dst_ranges.clear();
        }

        // src_ranges will end up containing the locations - find the minimum value there
        src_ranges
            .drain(..)
            .map(|(start, _)| start)
            .min()
            .unwrap_or(u64::MAX)
    }
}

fn main() {
    let almanac_str = read_input_as_string(2023, 5).unwrap();
    let almanac = Almanac::from_str(&almanac_str, false);
    println!("{}", almanac.lowest_location_number());

    let almanac = Almanac::from_str(&almanac_str, true);
    println!("{}", almanac.lowest_location_number());
}

#[cfg(test)]
mod tests {
    use crate::Almanac;

    const SAMPLE: &str = r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    "#;

    #[test]
    fn test() {
        let almanac = Almanac::from_str(SAMPLE, false);
        assert_eq!(35, almanac.lowest_location_number());

        let almanac = Almanac::from_str(SAMPLE, true);
        assert_eq!(46, almanac.lowest_location_number());
    }
}
