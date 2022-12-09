#[derive(Debug)]
pub struct FsEntry {
    size: u64,
    children: Vec<FsEntry>,
}

impl PartialEq for FsEntry {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.children == other.children
    }
}

impl FsEntry {
    fn all_sizes(&self) -> Vec<u64> {
        let mut child_sizes = self
            .children
            .iter()
            .flat_map(|child| child.all_sizes())
            .collect::<Vec<u64>>();
        child_sizes.push(self.size);
        child_sizes
    }
}

pub fn parse_state(input: &str) -> FsEntry {
    let mut stack = vec![FsEntry {
        size: 0,
        children: vec![],
    }];

    for line in input.split("\n") {
        let command = line.split(" ").collect::<Vec<&str>>();
        match command[0] {
            "$" => match command[1] {
                "cd" => match command[2] {
                    "/" => {}
                    ".." => {
                        let child = stack.pop().unwrap();
                        let parent = stack.last_mut().unwrap();
                        let new_size = parent.size + child.size;

                        parent.size = new_size;
                        parent.children.push(child);
                    }
                    _ => {
                        stack.push(FsEntry {
                            size: 0,
                            children: vec![],
                        });
                    }
                },
                _ => {}
            },
            "dir" => {}
            "" => {}
            _ => {
                let new_size = stack.last().unwrap().size + command[0].parse::<u64>().unwrap();
                stack.last_mut().unwrap().size = new_size;
            }
        }
    }

    while stack.len() > 1 {
        let child = stack.pop().unwrap();
        let parent = stack.last_mut().unwrap();
        let new_size = parent.size + child.size;

        parent.size = new_size;
        parent.children.push(child);
    }

    stack.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_state(input)
            .all_sizes()
            .iter()
            .filter(|size| **size <= (100000 as u64))
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let root = parse_state(input);
    let space_to_free = 30000000 - (70000000 - root.size);

    parse_state(input)
        .all_sizes()
        .iter()
        .filter(|size| **size >= space_to_free)
        .min()
        .copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(
            parse_state(&input),
            FsEntry {
                size: 48381165,
                children: vec![
                    FsEntry {
                        size: 94853,
                        children: vec![FsEntry {
                            size: 584,
                            children: vec![],
                        },],
                    },
                    FsEntry {
                        size: 24933642,
                        children: vec![],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
