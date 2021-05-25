use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{eof, map, map_res, recognize},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Kilobyte(u64),
    Megabyte(u64),
    Gigabyte(u64),
    Terabyte(u64),
    Petabyte(u64),
}

impl Size {
    pub fn to_bytes(&self) -> u64 {
        match self {
            Size::Kilobyte(v) => v * 1024,
            Size::Megabyte(v) => v * 1024_u64.pow(2),
            Size::Gigabyte(v) => v * 1024_u64.pow(3),
            Size::Terabyte(v) => v * 1024_u64.pow(4),
            Size::Petabyte(v) => v * 1024_u64.pow(5),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SizeComparison {
    GreaterThan(Size),
    LessThan(Size),
    EqualTo(Size),
}

impl SizeComparison {
    pub fn check(&self, bytes: u64) -> bool {
        match self {
            SizeComparison::GreaterThan(v) => bytes > v.to_bytes(),
            SizeComparison::LessThan(v) => bytes < v.to_bytes(),
            SizeComparison::EqualTo(v) => bytes > v.to_bytes() - 512 && bytes < v.to_bytes() + 512,
        }
    }
}

impl std::str::FromStr for SizeComparison {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s) {
            Ok((_, size_comparison)) => Ok(size_comparison),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, SizeComparison> {
    terminated(
        alt((
            map(parse_size, SizeComparison::EqualTo),
            map(preceded(tag("+"), parse_size), SizeComparison::GreaterThan),
            map(preceded(tag("-"), parse_size), SizeComparison::LessThan),
        )),
        eof,
    )(input)
}

fn parse_size(input: &str) -> IResult<&str, Size> {
    alt((
        map(terminated(parse_u64, tag("k")), Size::Kilobyte),
        map(terminated(parse_u64, tag("M")), Size::Megabyte),
        map(terminated(parse_u64, tag("G")), Size::Gigabyte),
        map(terminated(parse_u64, tag("T")), Size::Terabyte),
        map(terminated(parse_u64, tag("P")), Size::Petabyte),
    ))(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kb() {
        assert_eq!(
            parse("3k").unwrap().1,
            SizeComparison::EqualTo(Size::Kilobyte(3))
        );
    }

    #[test]
    fn test_parse_greater_than() {
        assert_eq!(
            parse("+4G").unwrap().1,
            SizeComparison::GreaterThan(Size::Gigabyte(4))
        );
    }

    #[test]
    fn test_parse_less_than() {
        assert_eq!(
            parse("-150M").unwrap().1,
            SizeComparison::LessThan(Size::Megabyte(150))
        );
    }
}
