// copied from Chris Biscardi AOC2023
// https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust/day-02-parsing

// a parser written by hand in nom
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game<'a> {
    pub id: &'a str,
    pub rounds: Vec<Round>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))(input)
}
fn cube(input: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(complete::u32, space1, parse_color)(input)
}
fn round(input: &str) -> IResult<&str, Round> {
    fold_many1(
        terminated(cube, opt(tag(", "))),
        Round::default,
        |mut round, (count, color)| {
            match color {
                Color::Red => {
                    round.red = count;
                }
                Color::Green => {
                    round.green = count;
                }
                Color::Blue => {
                    round.blue = count;
                }
            }
            round
        },
    )(input)
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn game(input: &str) -> IResult<&str, Game> {
    // parse "Game " at least 1 digit, a tag ": "
    let (input, id) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    // then using what remains of input...
    // parse a list of rounds, each delimited by "; " or eol
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    // parse lines
    separated_list1(line_ending, game)(input)
}

#[cfg(test)]
mod tests {
    // use crate::tests::game_output;
    use super::parse;

    #[test]
    fn test_parse() {
        pub const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let (input, game) = parse(INPUT).unwrap();
        // assert all the input was used
        assert_eq!(input, "");
        // assert the result looks like its correctly parsed (do I need to do this?)
        // assert_eq!(game_output::output(), &game);
        println!("{:?}", game);
    }
}
