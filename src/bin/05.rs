pub fn parse_state(input: &str) -> Vec<Vec<char>> {
    let binding = input
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();
    let rows = binding.iter().rev();
    let mut state = Vec::new();

    for (i, row) in rows.enumerate() {
        for (j, supply_crate) in row.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if i == 0 {
                state.push(Vec::new());
            } else if supply_crate[1] != ' ' {
                state[j].push(supply_crate[1]);
            }
        }
    }

    state
}

pub fn part_one(input: &str) -> Option<String> {
    let mut state = parse_state(input);

    for instruction in input.split("\n\n").skip(1).next().unwrap().split("\n") {
        let values: Vec<usize> = instruction
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|num| -> usize { num.parse::<usize>().unwrap() })
            .collect();

        if values.len() > 0 {
            for _ in 0..values[0] {
                let supply_crate = state[values[1] - 1].pop().unwrap();
                state[values[2] - 1].push(supply_crate);
            }
        }
    }

    Some(
        state
            .iter()
            .map(|col| col.last().unwrap())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut state = parse_state(input);

    for instruction in input.split("\n\n").skip(1).next().unwrap().split("\n") {
        let values: Vec<usize> = instruction
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|num| -> usize { num.parse::<usize>().unwrap() })
            .collect();

        if values.len() > 0 {
            let mut supply_crates = Vec::new();

            for _ in 0..values[0] {
                supply_crates.insert(0, state[values[1] - 1].pop().unwrap());
            }

            state[values[2] - 1].append(&mut supply_crates);
        }
    }

    Some(
        state
            .iter()
            .map(|col| col.last().unwrap())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(
            parse_state(&input),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
