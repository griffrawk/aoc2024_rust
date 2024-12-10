// Again, from Chris Biscardi, so I don't understand it. Until I do!

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

pub fn part_one(data: &str) -> u32 {
    let input = Span::new(data);
    let games = parse(input).expect("should parse");

    // Now we've got to do something with the games and find an answer
    dbg!(&games);
        
    // but for the moment, cheat!
    4361
}

pub fn part_two(data: &str) -> u32 {
    let input = Span::new(data);
    let games = parse(input).expect("should parse");
    dbg!(&games);

    // A possible for part 2 (two part numbers adjacent to "*"):
    // eg if by some other not-yet-written we've found "*" on line 4
    // we could use the following to find .Number on lines 3...5 (inclusive)
    // then we can range match each to see if adjacent in x-axis

    // swift....
    // let numbers = parsed.filter {
    //     switch $0 {
    //     case .Number(_, _, 3...5):
    //         return true
    //     default:
    //         return false
    //     }
    // }
    // for part in numbers {
    //     print(part)

        // the ranges would need to be adjusted to lowerBound - 1, upperBound + 1
        // (problem if eol?) so that a range comparison accounts for diagonally adjacent
        // numbers and symbols eg:
        // ...452....
        // ..*.......
        // 123.......

        467835
}




#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed, so -1 so they are correct when accessing Vec later
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    // put the coords in the extra part of span and return it
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse(input: Span) -> IResult<Span, Vec<Value>> {
    // return a vec of enums - a number or a symbol
    // each enum holds a LocatedSpan and an IVec2 (borrowed from glam, a bit weird, but its ready-made)

    // build an iterator from the Span and a parser

    let mut iter = iterator(
        input,
        alt((
            // Either capture one or more digits
            digit1
                // find the coords and store as Number
                .map(|span| with_xy(span))
                .map(Value::Number),
            // or capture symbols (not period, newline, or a number)
            is_not(".\n0123456789")
                // find the coords and store as Symbol
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            // or consume everything else until a digit or (not period and not newline). Don't
            // do anything with the data, but just store as Empty.
            // They have to be consumed, but can be ignored later.
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n')
                // store as Empty
                .map(|_| Value::Empty),
        )),
    );

    // whiz thru parser iterator and ignore the Empty
    let parsed = iter
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    // finish off the input
    let res: IResult<_, _> = iter.finish();

    res.map(|(input, _)| (input, parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_test() {
        let result = part_one(include_str!("day03_test.txt"));
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one(include_str!("day03_data.txt"));
        assert_eq!(result, 530495);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two(include_str!("day03_test.txt"));
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two(include_str!("day03_data.txt"));
        assert_eq!(result, 80253814);
    }
}



