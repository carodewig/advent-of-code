// Red-Nosed Reports

use common::read_input_as_lines;
use itertools::Itertools;

type Report = Vec<u16>;

fn main() {
    println!("{}", count_safe_reports_part1(fetch_reports()));
    println!("{}", count_safe_reports_part2(fetch_reports()));
}

fn fetch_reports() -> Vec<Report> {
    let lines = read_input_as_lines(2024, 2).expect("Unable to fetch input");

    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|elem| elem.parse().ok())
                .collect()
        })
        .collect()
}

fn report_is_safe(report: &Report) -> bool {
    if report.is_empty() {
        return false;
    }

    // must be either increasing or decreasing -- ie if you sort it, it should
    // equal the input or its reverse
    let mut sorted_report: Report = report.iter().copied().sorted_unstable().collect();
    let increasing = report == &sorted_report;

    sorted_report.reverse();
    let decreasing = report == &sorted_report;

    if !(increasing || decreasing) {
        return false;
    }

    for (a, b) in report.iter().zip(report.iter().skip(1)) {
        let delta = a.abs_diff(*b);
        if delta == 0 || delta > 3 {
            return false;
        }
    }

    true
}

fn report_is_safe_within_tolerance(report: &Report) -> bool {
    if report.is_empty() {
        return false;
    }

    if report_is_safe(report) {
        return true;
    }

    // iterate through vec, removing one item at a time, to see if any of
    // those sub-reports are safe
    for index in 0..report.len() {
        let mut subreport = report.clone();
        subreport.remove(index);
        if report_is_safe(&subreport) {
            return true;
        }
    }

    false
}

fn count_safe_reports_part1(reports: Vec<Report>) -> usize {
    reports.into_iter().filter(report_is_safe).count()
}

fn count_safe_reports_part2(reports: Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(report_is_safe_within_tolerance)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{Report, count_safe_reports_part1, count_safe_reports_part2};

    fn get_reports() -> Vec<Report> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 3, 6, 7, 9],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
        ]
    }

    #[test]
    fn test_report_safety_part1() {
        assert_eq!(count_safe_reports_part1(get_reports()), 2);
    }

    #[test]
    fn test_report_safety_part2() {
        assert_eq!(count_safe_reports_part2(get_reports()), 4);
    }
}
