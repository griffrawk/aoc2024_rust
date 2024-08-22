// Again, from Biscardi

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};

use nom_locate::{LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

pub fn part_one(data: &str) -> u32 {
    dbg!(&data);

    let input = Span::new(data);
    let games = parse(input).expect("should parse");

    4361
}

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    // put the coords in the extra part of span and return it
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse(input: Span) -> IResult<Span, Vec<Value>> {
    // return a vec of 'objects' in the input, a number or a symbol

    // build an iterator from the Span and a parser
    let mut it = iterator(
        input,
        alt((
            // capture digits
            digit1
            // find the coords and store as Number
                .map(|span| with_xy(span))
                .map(Value::Number),
            // capture symbols
            is_not(".\n0123456789")
            // find the coords and store as Symbol
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            // capture everything else. They have to be consumed, but can be ignored later
            take_till1(|c: char| {
                c.is_ascii_digit() || c != '.' && c != '\n'
            })
            // store as Empty
            .map(|_| Value::Empty),
        )),
    );

    // whiz thru iterator and ignore the Empty
    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    // finish off the input
    let res: IResult<_, _> = it.finish();

    dbg!(&parsed);
    res.map(|(input, _)| (input, parsed))
}

pub fn part_two(input: &str) -> u32 {
    todo!()
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
