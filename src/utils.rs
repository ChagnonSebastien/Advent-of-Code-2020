use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Part {
  PartOne,
  PartTwo,
}

impl Part {
  pub fn from_number(number: &str) -> Result<Part, String> {
    match number.parse::<u8>().unwrap_or(0) {
      1 => Ok(Part::PartOne),
      2 => Ok(Part::PartTwo),
      _ => Err(format!("Invalid part number. Got {}. Expected a value between 1 and 2.", number)),
    }
  }
}

impl Display for Part {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", match self {
      Part::PartOne => "Part 1",
      Part::PartTwo => "Part 2",
    })
  }
}

pub enum Day {
  DayOne,
  DayTwo,
  DayThree,
  DayFour,
  DayFive,
  DaySix,
  DaySeven,
  DayEight,
  DayNine,
  DayTen,
  DayEleven,
  DayTwelve,
  DayThirteen,
  DayFourteen,
  DayFifteen,
  DaySixteen,
  DaySeventeen,
  DayEighteen,
  DayNineteen,
  DayTwenty,
  DayTwentyOne,
  DayTwentyTwo,
  DayTwentyThree,
  DayTwentyFour,
  DayTwentyFive,
}

impl Day  {
  pub fn from_number(number: &str) -> Result<Day, String> {
    match number.parse::<u8>().unwrap_or(0) {
      1 => Ok(Day::DayOne),
      2 => Ok(Day::DayTwo),
      3 => Ok(Day::DayThree),
      4 => Ok(Day::DayFour),
      5 => Ok(Day::DayFive),
      6 => Ok(Day::DaySix),
      7 => Ok(Day::DaySeven),
      8 => Ok(Day::DayEight),
      9 => Ok(Day::DayNine),
      10 => Ok(Day::DayTen),
      11 => Ok(Day::DayEleven),
      12 => Ok(Day::DayTwelve),
      13 => Ok(Day::DayThirteen),
      14 => Ok(Day::DayFourteen),
      15 => Ok(Day::DayFifteen),
      16 => Ok(Day::DaySixteen),
      17 => Ok(Day::DaySeventeen),
      18 => Ok(Day::DayEighteen),
      19 => Ok(Day::DayNineteen),
      20 => Ok(Day::DayTwenty),
      21 => Ok(Day::DayTwentyOne),
      22 => Ok(Day::DayTwentyTwo),
      23 => Ok(Day::DayTwentyThree),
      24 => Ok(Day::DayTwentyFour),
      25 => Ok(Day::DayTwentyFive),
      _ => Err(format!("Invalid day number. Got {}. Expected a value between 1 and 25.", number)),
    }
  }
}

impl Display for Day {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", match self {
      Day::DayOne => "01",
      Day::DayTwo => "02",
      Day::DayThree => "03",
      Day::DayFour => "04",
      Day::DayFive => "05",
      Day::DaySix => "06",
      Day::DaySeven => "07",
      Day::DayEight => "08",
      Day::DayNine => "09",
      Day::DayTen => "10",
      Day::DayEleven => "11",
      Day::DayTwelve => "12",
      Day::DayThirteen => "13",
      Day::DayFourteen => "14",
      Day::DayFifteen => "15",
      Day::DaySixteen => "16",
      Day::DaySeventeen => "17",
      Day::DayEighteen => "18",
      Day::DayNineteen => "19",
      Day::DayTwenty => "20",
      Day::DayTwentyOne => "21",
      Day::DayTwentyTwo => "22",
      Day::DayTwentyThree => "23",
      Day::DayTwentyFour => "24",
      Day::DayTwentyFive => "25",
    })
  }
}
