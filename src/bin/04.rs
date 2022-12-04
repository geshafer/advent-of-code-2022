use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .filter_map(|pair: &str| -> Option<&str> {
                if pair.is_empty() {
                    None
                } else {
                    let ranges = pair
                        .split(",")
                        .flat_map(|range: &str| range.split("-").map(|v| v.parse::<u32>().unwrap()))
                        .collect::<Vec<u32>>();

                    if (ranges[0] <= ranges[2] && ranges[1] >= ranges[3])
                        || (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
                    {
                        Some(pair)
                    } else {
                        None
                    }
                }
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .filter_map(|pair: &str| -> Option<&str> {
                if pair.is_empty() {
                    None
                } else {
                    let ranges = pair
                        .split(",")
                        .map(|range: &str| -> HashSet<u32> {
                            let values = range
                                .split("-")
                                .map(|v: &str| -> u32 { v.parse::<u32>().unwrap() })
                                .collect::<Vec<u32>>();
                            HashSet::from_iter(std::ops::Range {
                                start: values[0],
                                end: values[1] + 1,
                            })
                        })
                        .collect::<Vec<HashSet<u32>>>();

                    if ranges[0].intersection(&ranges[1]).count() > 0 {
                        Some(pair)
                    } else {
                        None
                    }
                }
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
