use std::collections::HashMap;
use utils::get_input;

fn houses_visited<M: Iterator<Item = char>>(moves: M) -> HashMap<(i64, i64), u64> {
    let mut visited = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    visited.insert((x, y), 1);

    for mv in moves {
        match mv {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {
                panic!("Unknown move {}!", mv)
            }
        }
        let counter = visited.entry((x, y)).or_insert(0);
        *counter += 1;
    }

    return visited;
}

fn part_1(moves: &str) -> usize {
    houses_visited(moves.chars()).len()
}

fn part_2(moves: &str) -> usize {
    let santa = houses_visited(moves.chars().step_by(2));
    let robot = houses_visited(moves.chars().skip(1).step_by(2));

    let mut combined = HashMap::new();
    combined.extend(santa.into_iter());
    combined.extend(robot.into_iter());
    combined.len()
}

pub fn solve() {
    let moves = get_input(2015, 3);

    println!("Day 03, Part 1: {}", part_1(&moves));
    println!("Day 03, Part 2: {}", part_2(&moves));
}

#[cfg(test)]
mod day_03_tests {
    use super::*;

    #[test]
    fn day_03_part_1_examples() {
        assert_eq!(houses_visited(">").len(), 2);
        assert_eq!(houses_visited("^>v<").len(), 4);
        assert_eq!(houses_visited("^v^v^v^v^v").len(), 2);
    }

    #[test]
    fn day_03_part_2_examples() {
        assert_eq!(houses_visited(">").len(), 2);
        assert_eq!(houses_visited("^>v<").len(), 4);
        assert_eq!(houses_visited("^v^v^v^v^v").len(), 2);
    }
}
