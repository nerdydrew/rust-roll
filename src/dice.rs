use rand::Rng;
use regex::Regex;
use crate::dice::DiceTerm::{Constant, Dice};
use std::convert::TryInto;

#[derive(Debug)]
pub enum DiceTerm {
    /// A dice roll like `2d20` or `-d20`.
    Dice { count: i32, sides: u32 },
    /// A constant offset to a roll like `2` or `-5`.
    Constant(i32)
}

impl DiceTerm {
    /// Parses the given string as dice terms. Any unknown values are skipped.
    pub fn parse(s: &str) -> Vec<DiceTerm> {
        let s = s.to_lowercase();

        let dice_regex = Regex::new(r"(?x)
(?P<dice>(?P<count>[+-]?\d*)d(?P<sides>\d+)) // XdY, where X is optional and can be negative
|
(?P<constant>[+-]?\d+) // a constant offset, which can be negative
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

    /// Rolls the dice, returning a random value.
    pub fn roll(&self) -> i32 {
        match self {
            Dice{count, sides} => {
                let max_value = (sides+1).try_into().unwrap();
                let total: i32 = (0..(*count).abs())
                    .map(|_| rand::thread_rng().gen_range(1, max_value))
                    .sum();
                total * sign(*count)
            },
            Constant(constant) => *constant
        }
    }

    /// Returns the expected average value of this term.
    pub fn average(&self) -> f64 {
        match self {
            Dice{count, sides} => (*count as f64) * (*sides as f64 + 1.0) / 2.0,
            Constant(constant) => *constant as f64
        }
    }
}

/// Returns `+1`, `-1`, or `0` according to the sign of the input.
fn sign(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x > 0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_rolls() {
        let roll = Dice{count: 1, sides: 20}.roll();
        assert!(roll <= 20, "Roll {} should be no more than 20.", roll);
        assert!(roll >= 1, "Roll {} should be at least 1.", roll);
    }

    #[test]
    fn test_negative_dice_rolls() {
        let roll = Dice{count: -3, sides: 6}.roll();
        assert!(roll >= -18, "Roll {} should be at least -18.", roll);
        assert!(roll <= -3, "Roll {} should be no more than -3.", roll);
    }

    #[test]
    fn test_constant_rolls() {
        assert_eq!(10, Constant(10).roll());
        assert_eq!(-2, Constant(-2).roll());
    }

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