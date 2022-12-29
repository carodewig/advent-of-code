/// day 16: proboscidea volcanium
use common::read_input_as_string;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

type ValveName = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: ValveName,
    flow_rate: u32,
    connecting_valves: Vec<ValveName>,
}

impl Valve {
    fn name(&self) -> ValveName {
        self.name.clone()
    }
    fn try_from_line(line: &str) -> Option<Self> {
        let re = Regex::new(
            "^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-z, ]+)$",
        )
        .ok()?;

        let caps = re.captures(line.trim())?;

        let valve_name = caps.get(1)?.as_str().to_string();
        let flow_rate = caps.get(2)?.as_str().parse().ok()?;

        let connecting_valves = caps
            .get(3)?
            .as_str()
            .split(", ")
            .map(|valve_name| valve_name.to_string())
            .collect();

        Some(Valve {
            name: valve_name,
            flow_rate,
            connecting_valves,
        })
    }
}

fn parse_input(input: &str) -> HashMap<ValveName, Valve> {
    input
        .split('\n')
        .filter_map(Valve::try_from_line)
        .map(|valve| (valve.name.clone(), valve))
        .collect()
}

/// floyd warshall impl
/// https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
fn minimum_time_between_valves(
    valves: &HashMap<ValveName, Valve>,
) -> HashMap<ValveName, HashMap<ValveName, u32>> {
    let mut times: HashMap<(ValveName, ValveName), u32> = HashMap::default();
    for valve in valves.values() {
        times.insert((valve.name(), valve.name()), 0);
        for connecting_valve in valve.connecting_valves.iter().filter_map(|v| valves.get(v)) {
            times.insert((valve.name(), connecting_valve.name()), 1);
        }
    }

    for interim_valve in valves.values() {
        for start_valve in valves.values() {
            for end_valve in valves.values() {
                let path_name = (start_valve.name(), end_valve.name());
                let time_se = times.get(&path_name);
                let time_si = times.get(&(start_valve.name(), interim_valve.name()));
                let time_ie = times.get(&(interim_valve.name(), end_valve.name()));
                if let (Some(time_si), Some(time_ie)) = (time_si, time_ie) {
                    if let Some(time_se) = time_se {
                        if time_se > &(time_si + time_ie) {
                            times.insert(path_name, time_si + time_ie);
                        }
                    } else {
                        times.insert(path_name, time_si + time_ie);
                    }
                }
            }
        }
    }

    // expand map structure so that it's easier to reason about (ie start -> end -> duration)
    let mut nested_times = HashMap::default();
    for ((start, end), time) in times.into_iter() {
        let start_entry: &mut HashMap<String, u32> = nested_times.entry(start).or_default();
        start_entry.insert(end, time);
    }

    nested_times
}

/// I originally didn't think it was a good idea to enumerate all possibilities, but after playing with a recursive
/// approach for ages it seemed like it was worth a try. It's significantly faster!
fn pressure_relief_for_all_possible_valve_visits(
    valves: &HashMap<ValveName, Valve>,
    duration: u32,
) -> HashMap<Vec<ValveName>, u32> {
    let minimum_valve_distances = minimum_time_between_valves(valves);
    let mut maximum_pressure_relief: HashMap<Vec<ValveName>, u32> = HashMap::default();

    let mut to_visit: VecDeque<(String, u32, u32, Vec<ValveName>)> = VecDeque::default();
    to_visit.push_back(("AA".to_string(), duration, 0, Vec::default()));

    while let Some((valve, minutes_remaining, pressure_relief, opened_valves)) =
        to_visit.pop_front()
    {
        let entry = maximum_pressure_relief
            .entry(opened_valves.clone())
            .or_insert(0);
        *entry = (*entry).max(pressure_relief);

        for (neighbor_name, time) in minimum_valve_distances.get(&valve).unwrap() {
            // if the neighbor is open, its flow is 0, or if it would take too long to get there, skip it
            let neighbor = valves.get(neighbor_name).unwrap();
            if neighbor.flow_rate == 0
                || opened_valves.contains(neighbor_name)
                || time >= &minutes_remaining
            {
                continue;
            }

            // otherwise add this neighbor to the 'to visit' list
            let minutes_remaining = minutes_remaining - time - 1;

            let mut opened_valves = opened_valves.clone();
            opened_valves.push(neighbor.name());
            opened_valves.sort();

            to_visit.push_back((
                neighbor.name(),
                minutes_remaining,
                pressure_relief + minutes_remaining * neighbor.flow_rate,
                opened_valves,
            ));
        }
    }

    maximum_pressure_relief
}

fn part1(valves: &HashMap<ValveName, Valve>) -> u32 {
    let relief = pressure_relief_for_all_possible_valve_visits(valves, 30);
    *relief.values().max().unwrap_or(&0)
}

fn part2(valves: &HashMap<ValveName, Valve>) -> u32 {
    let relief: Vec<(HashSet<String>, u32)> =
        pressure_relief_for_all_possible_valve_visits(valves, 26)
            .into_iter()
            .map(|(valves, total)| (HashSet::from_iter(valves.into_iter()), total))
            .collect();

    // need to find the highest pair of sets which do not have overlapping values...
    let mut highest_combined_total = 0;
    for i in 0..relief.len() {
        let (set1, t1) = relief.get(i).unwrap();
        for j in i..relief.len() {
            let (set2, t2) = relief.get(j).unwrap();
            if set1.is_disjoint(set2) {
                highest_combined_total = highest_combined_total.max(t1 + t2);
            }
        }
    }

    highest_combined_total
}

fn main() {
    let valves = parse_input(&read_input_as_string(2022, 16).unwrap());
    println!("{}", part1(&valves));
    println!("{}", part2(&valves));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II"
    };

    #[test]
    fn test() {
        let valves = parse_input(SAMPLE);
        assert_eq!(1651, part1(&valves));
        assert_eq!(1707, part2(&valves));
    }
}
