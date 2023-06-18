use std::str::FromStr;

use chrono::{Datelike, NaiveDateTime, NaiveTime, ParseResult, SubsecRound, Weekday};
use chronoutil::RelativeDuration;
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{digit1, one_of},
    combinator::{map_res, opt},
    sequence::tuple,
    IResult,
};

use crate::prelude::*;

pub fn parse_given_datetime(date_code: &str, datetime: &NaiveDateTime) -> Result<NaiveDateTime> {
    if let Ok(d) = NaiveDateTime::parse_from_str(date_code, "%Y-%m-%d %H%M") {
        Ok(d)
    } else {
        let date_code = DateCode::parse(date_code)?;
        Ok(date_code.due_datetime(datetime))
    }
}

fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Frequency {
    Day,
    Week,
    Month,
    Year,
}

impl FromStr for Frequency {
    type Err = ToDoError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d" => Ok(Self::Day),
            "w" => Ok(Self::Week),
            "m" => Ok(Self::Month),
            "y" => Ok(Self::Year),
            _ => Err(ToDoError::Generic(
                "Unrecognised date code frequency.".to_string(),
            )),
        }
    }
}

impl Frequency {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (next_input, freq_code) = one_of("dwmyDWMY")(input)?;
        match freq_code.to_string().parse() {
            Ok(f) => Ok((next_input, f)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Char,
            ))),
        }
    }

    fn duration(&self, length: u32) -> RelativeDuration {
        match self {
            Frequency::Day => RelativeDuration::days(length as i64),
            Frequency::Week => RelativeDuration::weeks(length as i64),
            Frequency::Month => RelativeDuration::months(length as i32),
            Frequency::Year => RelativeDuration::years(length as i32),
        }
    }
}

fn parse_time_four_figures(input: &str) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(input, "%H%M")
}

fn parse_time(input: &str) -> IResult<&str, NaiveTime> {
    map_res(digit1, parse_time_four_figures)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Offset {
    length: u32,
    freq: Frequency,
    time: Option<NaiveTime>,
}

impl Offset {
    #[allow(dead_code)]
    fn new(length: u32, freq: Frequency, time: Option<NaiveTime>) -> Self {
        Self { length, freq, time }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((parse_numbers, Frequency::parse, opt(parse_time)))(input).map(|(next_input, res)| {
            let (length, freq, time) = res;
            (next_input, Self { length, freq, time })
        })
    }

    fn due_datetime(&self, datetime: &NaiveDateTime) -> NaiveDateTime {
        let relative_duration = self.freq.duration(self.length);
        let due_datetime = *datetime + relative_duration;

        match self.time {
            Some(time) => NaiveDateTime::new(due_datetime.date(), time),
            None => due_datetime.round_subsecs(0),
        }
    }
}

fn parse_weekday(input: &str) -> IResult<&str, Weekday> {
    let (next_input, weekday) = alt((
        tag_no_case("mon"),
        tag_no_case("tue"),
        tag_no_case("wed"),
        tag_no_case("thu"),
        tag_no_case("fri"),
        tag_no_case("sat"),
        tag_no_case("sun"),
    ))(input)?;
    match weekday.parse() {
        Ok(w) => Ok((next_input, w)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NextDay {
    day: Weekday,
    time: Option<NaiveTime>,
}

impl NextDay {
    #[allow(dead_code)]
    fn new(day: Weekday, time: Option<NaiveTime>) -> Self {
        Self { day, time }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((parse_weekday, opt(parse_time)))(input).map(|(next_input, res)| {
            let (day, time) = res;
            (next_input, Self { day, time })
        })
    }

    fn due_datetime(&self, datetime: &NaiveDateTime) -> NaiveDateTime {
        let mut comparison_day = datetime.weekday();

        let mut offset = 1;
        loop {
            comparison_day = comparison_day.succ();
            if self.day == comparison_day {
                break;
            } else {
                offset += 1;
            }
        }

        let due_datetime = *datetime + RelativeDuration::days(offset);

        match self.time {
            Some(time) => NaiveDateTime::new(due_datetime.date(), time),
            None => due_datetime.round_subsecs(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DateCode {
    Offset(Offset),
    NextDay(NextDay),
}

impl DateCode {
    fn parse(input: &str) -> Result<Self> {
        if let Ok(("", o)) = Offset::parse(input) {
            return Ok(Self::Offset(o));
        };
        if let Ok(("", n)) = NextDay::parse(input) {
            return Ok(Self::NextDay(n));
        };
        Err(ToDoError::Generic("Date code parse error".to_string()))
    }

    fn due_datetime(&self, datetime: &NaiveDateTime) -> NaiveDateTime {
        match self {
            Self::Offset(o) => o.due_datetime(datetime),
            Self::NextDay(n) => n.due_datetime(datetime),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use rstest::{fixture, rstest};

    use crate::due::{parse_given_datetime, Frequency, Offset};

    #[fixture]
    fn now() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
    }

    #[rstest]
    #[case("2020-01-01 1700", (2020, 1, 1, 17, 0, 0))]
    #[case("2d", (2020, 1, 3, 12, 0, 0))]
    #[case("2d1700", (2020, 1, 3, 17, 0, 0))]
    #[case("1w", (2020, 1, 8, 12, 0, 0))]
    #[case("Mon", (2020, 1, 6, 12, 0, 0))]
    #[case("Mon1700", (2020, 1, 6, 17, 0, 0))]
    fn parse_explicit_date(
        #[case] date_code: &str,
        #[case] expected: (i32, u32, u32, u32, u32, u32),
        now: NaiveDateTime,
    ) {
        assert_eq!(
            parse_given_datetime(date_code, &now).unwrap(),
            NaiveDate::from_ymd_opt(expected.0, expected.1, expected.2)
                .unwrap()
                .and_hms_opt(expected.3, expected.4, expected.5)
                .unwrap()
        )
    }

    #[test]
    fn parse_frequency() {
        assert_eq!(Frequency::parse("dz"), Ok(("z", Frequency::Day)));
        assert_eq!(Frequency::parse("w"), Ok(("", Frequency::Week)));
        assert_eq!(Frequency::parse("m17"), Ok(("17", Frequency::Month)));
        assert_eq!(Frequency::parse("y00"), Ok(("00", Frequency::Year)));
    }

    #[test]
    fn parse_offset() {
        assert_eq!(
            Offset::parse("2dz"),
            Ok(("z", Offset::new(2, Frequency::Day, None)))
        );
        assert_eq!(
            Offset::parse("3w0900"),
            Ok((
                "",
                Offset::new(
                    3,
                    Frequency::Week,
                    Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
                )
            ))
        );
    }
}
