use itertools::Itertools;
use utils::get_input;

fn paper_required(dims: &Vec<u64>) -> u64 {
    let areas: Vec<u64> = dims
        .iter()
        .combinations(2)
        .map(|dims| dims.into_iter().product::<u64>())
        .collect();
    let sides: u64 = areas.iter().sum();
    let slack: u64 = *areas.iter().min().unwrap();
    (2 * sides) + slack
}

fn ribbon_required(dims: &Vec<u64>) -> u64 {
    let smallest_half_perimeter: u64 = dims
        .iter()
        .combinations(2)
        .map(|dims| dims.into_iter().sum::<u64>())
        .min()
        .unwrap();

    let volume: u64 = dims.iter().product();

    (2 * smallest_half_perimeter) + volume
}

fn part_1(dimensions: &Vec<Vec<u64>>) -> u64 {
    dimensions.iter().map(|dims| paper_required(dims)).sum()
}

fn part_2(dimensions: &Vec<Vec<u64>>) -> u64 {
    dimensions.iter().map(|dims| ribbon_required(dims)).sum()
}

pub fn solve() {
    let dimensions: Vec<Vec<u64>> = get_input(2015, 2)
        .lines()
        .map(|line| line.split('x').map(|c| c.parse::<u64>().unwrap()).collect())
        .collect();

    println!("Day 02, Part 1: {}", part_1(&dimensions));
    println!("Day 02, Part 2: {}", part_2(&dimensions));
}

#[cfg(test)]
mod day_02_tests {
    use super::*;

    #[test]
    fn day_02_part_1_examples() {
        assert_eq!(paper_required(&vec![2, 3, 4]), 58);
        assert_eq!(paper_required(&vec![1, 1, 10]), 43);
    }

    #[test]
    fn day_02_part_2_examples() {
        assert_eq!(ribbon_required(&vec![2, 3, 4]), 34);
        assert_eq!(ribbon_required(&vec![1, 1, 10]), 14);
    }
}
