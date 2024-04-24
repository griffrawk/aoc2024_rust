// Mostly copied from Chris Biscardi AOC2023, to see how it would be done with nom
// https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust/day-02-parsing

// Part 2 is all my own work, but remarkably similar!

use std::{
    cmp,
    collections::BTreeMap,
    // ops::Not,
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// Oh lifetimes.. because were using &str in structs

#[derive(Debug)]
struct Cube<'a> {
    amount: u32,
    colour: &'a str,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    // check this game's cubes amounts are valid against the rules in map, return Some(id) if true
    fn valid_for_cube_set(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            // CB github version uses any, prob some reason CB used any... any & all both short-circuit, so
            // maybe so it fails as soon as possible, rather than wait for all to be checked
            // Actually he first writes as all, then it gets changed at some point
            // The video leaves it with all, and doesn't cover the change
            // .any(|round| {
            .all(|round| {
                // round.iter().any(|shown_cube| {
                round.iter().all(|shown_cube| {
                    // shown_cube.amount > *map.get(shown_cube.colour).expect("a valid cube")
                    shown_cube.amount <= *map.get(shown_cube.colour).expect("a valid cube")
                })
            })
            // .not()
            .then_some(
                // Return Some(id) if amount is <= limit, else None
                self.id
                    .parse::<u32>()
                    .expect("game id should be a parsable u32"),
            )
    }

    // minimum number of cubes of each colour to play this game
    fn minimum_cube_set(&self) -> u32 {
        let mut max_per_colour = BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        // CB uses fold on the map
        // I'm not interested in individual rounds, so flatten them
        self.rounds.iter().flatten().for_each(|cube| {
            max_per_colour.insert(
                cube.colour,
                cmp::max(max_per_colour[cube.colour], cube.amount),
            );
        });
        max_per_colour.values().product::<u32>()
    }
}

// 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, colour)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { amount, colour }))
}

// 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue;
// 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds, id }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn part_one(input: &str) -> u32 {
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = parse_games(input).expect("should parse");

    games
        .1
        .iter()
        .filter_map(|game| game.valid_for_cube_set(&map))
        .sum()
}

fn part_two(input: &str) -> u32 {
    let games = parse_games(input).expect("should parse");

    games.1.iter().map(|game| game.minimum_cube_set()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_test() {
        assert_eq!(8, part_one(include_str!("day02_test.txt")));
    }

    #[test]
    fn test_part_two_test() {
        assert_eq!(2286, part_two(include_str!("day02_test.txt")));
    }

    #[test]
    fn test_part_one_data() {
        assert_eq!(2105, part_one(include_str!("day02_data.txt")));
    }

    #[test]
    fn test_part_two_data() {
        assert_eq!(72422, part_two(include_str!("day02_data.txt")));
    }
}
