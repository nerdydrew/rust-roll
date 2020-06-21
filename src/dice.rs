use regex::Regex;
use crate::dice::DiceTerm::{Constant, Dice};

#[derive(Debug)]
pub enum DiceTerm {
    /// A dice roll like `2d20` or `-d20`.
    Dice { count: i32, sides: u32 },
    /// A constant offset to a roll like `2` or `-5`.
    Constant(i32)
}

impl DiceTerm {
    pub fn parse(s: &str) -> Vec<DiceTerm> {
        let s = s.to_lowercase();

        let dice_regex = Regex::new(r"(?x)
(?P<dice>(?P<count>[+-]?\d*)d(?P<sides>\d+))
|
(?P<constant>[+-]?\d+)
").unwrap();

        dice_regex.captures_iter(&s)
            .filter_map(|captures| {
                if let Some(_) = captures.name("dice") {
                    // Matches [count]d[sides]
                    let count = captures.name("count")
                        .map(|x| x.as_str().parse().ok())
                        .flatten()
                        .unwrap_or(1);
                    let sides = captures.name("sides")
                        .map(|x| x.as_str().parse().unwrap())
                        .unwrap();
                    Some(Dice { count, sides })
                } else if let Some(constant) = captures.name("constant") {
                    // Matches [constant]
                    Some(Constant(constant.as_str().parse().unwrap()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the expected average value of this term.
    pub fn average(&self) -> f64 {
        match self {
            Dice{count, sides} => (*count as f64) * (*sides as f64 + 1.0) / 2.0,
            Constant(constant) => *constant as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_averages() {
        assert_eq!(10.5, Dice{count: 1, sides: 20}.average());
        assert_eq!(-10.5, Dice{count: -3, sides: 6}.average());
        assert_eq!(28.0, Dice{count: 8, sides: 6}.average());
    }

    #[test]
    fn test_constant_averages() {
        assert_eq!(10.0, Constant(10).average());
        assert_eq!(-2.0, Constant(-2).average());
    }
}