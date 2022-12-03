pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n")
        .map(|round| {
            if round.is_empty() {
                Some(0)
            } else {
                match round.split_at(1) {
                    ("A", " X") => Some(1 + 3),
                    ("A", " Y") => Some(2 + 6),
                    ("A", " Z") => Some(3 + 0),
                    ("B", " X") => Some(1 + 0),
                    ("B", " Y") => Some(2 + 3),
                    ("B", " Z") => Some(3 + 6),
                    ("C", " X") => Some(1 + 6),
                    ("C", " Y") => Some(2 + 0),
                    ("C", " Z") => Some(3 + 3),
                    _ => Some(0),
                }
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n")
        .map(|round| {
            if round.is_empty() {
                Some(0)
            } else {
                match round.split_at(1) {
                    ("A", " X") => Some(3 + 0),
                    ("A", " Y") => Some(1 + 3),
                    ("A", " Z") => Some(2 + 6),
                    ("B", " X") => Some(1 + 0),
                    ("B", " Y") => Some(2 + 3),
                    ("B", " Z") => Some(3 + 6),
                    ("C", " X") => Some(2 + 0),
                    ("C", " Y") => Some(3 + 3),
                    ("C", " Z") => Some(1 + 6),
                    _ => Some(0),
                }
            }
        })
        .sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
