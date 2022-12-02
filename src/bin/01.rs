pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .filter_map(|elf_list| {
            Some(
                elf_list
                    .split("\n")
                    .filter_map(|calorie_value| {
                        if calorie_value.is_empty() {
                            None
                        } else {
                            Some(calorie_value.parse::<u32>().unwrap())
                        }
                    })
                    .sum::<u32>(),
            )
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = input
        .split("\n\n")
        .filter_map(|elf_list| {
            Some(
                elf_list
                    .split("\n")
                    .filter_map(|calorie_value| {
                        if calorie_value.is_empty() {
                            None
                        } else {
                            Some(calorie_value.parse::<u32>().unwrap())
                        }
                    })
                    .sum::<u32>(),
            )
        })
        .collect::<Vec<u32>>();

    calories.sort();
    calories.reverse();

    Some(calories.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
