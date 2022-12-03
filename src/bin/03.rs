use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n").map(|rucksack| {
        if rucksack.is_empty() {
            0
        } else {
            let (one, two) = rucksack.split_at(rucksack.chars().count() / 2);
            let compartment_one: HashSet<char> = HashSet::from_iter(one.chars().collect::<Vec<char>>());
            let compartment_two: HashSet<char> = HashSet::from_iter(two.chars().collect::<Vec<char>>());

            compartment_one.intersection(&compartment_two).map(|item| {
                let priority = *item as u32 - 38;

                if priority > 52 {
                    priority - 58
                } else {
                    priority
                }
            }).last().unwrap()
        }
    }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("\n").filter_map(|rucksack: &str| -> Option<HashSet<char>> {
        if rucksack.is_empty() {
            None
        } else {
            Some(HashSet::from_iter(rucksack.chars().collect::<Vec<char>>()))
        }
    }).collect::<Vec<HashSet<char>>>().chunks(3).map(|group| -> u32 {
        let one: HashSet<&char> = HashSet::from_iter(group[0].intersection(&group[1]).collect::<Vec<&char>>());
        let two: HashSet<&char> = HashSet::from_iter(group[2].intersection(&group[1]).collect::<Vec<&char>>());

        one.intersection(&two).map(|item| {
            let priority = **item as u32 - 38;

            if priority > 52 {
                priority - 58
            } else {
                priority
            }
        }).last().unwrap()
    }).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
