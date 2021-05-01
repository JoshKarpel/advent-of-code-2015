use utils::get_input;

fn part_1(instructions: &str) -> i64 {
    let mut floor = 0;
    for char in instructions.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            c => panic!("Unrecognized instruction '{}'!", c),
        }
    }

    floor
}

fn part_2(instructions: &str) -> i64 {
    let mut floor: i64 = 0;
    for (idx, char) in instructions.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            c => panic!("Unrecognized instruction '{}'!", c),
        }

        if floor < 0 {
            return (idx + 1) as i64;
        }
    }

    panic!("Didn't find the solution!")
}

pub fn solve() {
    let input = get_input(2015, 1);

    println!("Day 01, Part 1: {}", part_1(&input));
    println!("Day 01, Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);

        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);

        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);

        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(")"), 1);

        assert_eq!(part_2("()())"), 5);
    }
}
