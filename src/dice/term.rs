use rand::Rng;
use regex::Regex;
use std::convert::TryInto;
use std::fmt;

/// Represents a single term in a dice expression.
#[derive(Debug, PartialEq)]
pub enum DiceTerm {
    /// A dice roll like `2d20` or `-d20`.
    Dice { count: i32, sides: u32 },
    /// A constant offset to a roll like `2` or `-5`.
    Constant(i32)
}

impl DiceTerm {
    /// Parses the given string as dice terms. Returns an error message if any value cannot be parsed.
    pub fn parse(s: &str) -> Result<Vec<DiceTerm>, &'static str> {
        // Convert to lowercase and strip whitespace for simplicity in parsing.
        let mut s = s.to_lowercase();
        s.retain(|c| !c.is_whitespace());

        let dice_regex = Regex::new(r"(?x)
(?P<dice>(?P<count_sign>[+-])?(?P<count>\d+)?d(?P<sides>\d+)) # XdY, where X is optional and can be negative
|
(?P<constant>[+-]?\d+) # a constant offset, which can be negative
|
.+ # Any unrecognized characters (error)
").unwrap();

        dice_regex
            .captures_iter(&s)
            .map(|captures| {
                if let Some(_) = captures.name("dice") {
                    // Matches [count_sign][count]d[sides]

                    let mut count = captures
                        .name("count")
                        .map(|x| x.as_str())
                        .unwrap_or("1")
                        .to_owned();
                    // Prepend the sign if present.
                    if let Some(sign) = captures.name("count_sign") {
                        count.insert_str(0, sign.as_str());
                    }
                    // Convert to an integer now that we've accounted for optional values.
                    let count: i32 = count.parse().unwrap();

                    let sides = captures
                        .name("sides")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap();
                    Ok(DiceTerm::Dice { count, sides })
                } else if let Some(constant) = captures.name("constant") {
                    // Matches [constant]
                    Ok(DiceTerm::Constant(constant.as_str().parse().unwrap()))
                } else {
                    Err("Could not parse dice expression.")
                }
            })
            .collect()
    }

    /// Rolls the dice, returning a random value for each dice.
    pub fn roll(&self) -> Vec<i32> {
        match self {
            DiceTerm::Dice { count, sides } => {
                let max_value = (sides+1).try_into().unwrap();
                (0..(*count).abs())
                    .map(|_| rand::thread_rng().gen_range(1, max_value))
                    .map(|x| x * count.signum())
                    .collect::<Vec<i32>>()
            }
            DiceTerm::Constant(constant) => vec![*constant]
        }
    }

    /// Returns the expected average value of this term.
    pub fn average(&self) -> f64 {
        match self {
            DiceTerm::Dice { count, sides } => (*count as f64) * (*sides as f64 + 1.0) / 2.0,
            DiceTerm::Constant(constant) => *constant as f64
        }
    }
}

impl fmt::Display for DiceTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiceTerm::Dice { count, sides } => write!(f, "{}d{}", count, sides),
            DiceTerm::Constant(constant) => write!(f, "{}", constant)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::term::DiceTerm::{Constant, Dice};

    #[test]
    fn test_constant_parsing() {
        assert_eq!(Ok(vec![Constant(10)]), DiceTerm::parse("10"));
        assert_eq!(Ok(vec![Constant(10)]), DiceTerm::parse("+10"));
        assert_eq!(Ok(vec![Constant(10)]), DiceTerm::parse(" + 10"));
        assert_eq!(Ok(vec![Constant(-10)]), DiceTerm::parse("-10"));
        assert_eq!(Ok(vec![Constant(-10)]), DiceTerm::parse(" - 10"));
    }

    #[test]
    fn test_dice_parsing() {
        assert_eq!(Ok(vec![Dice { count: 1, sides: 20 }]), DiceTerm::parse("d20"));
        assert_eq!(Ok(vec![Dice { count: 1, sides: 20 }]), DiceTerm::parse("1d20"));
        assert_eq!(Ok(vec![Dice { count: -1, sides: 20 }]), DiceTerm::parse("-1d20"));
        assert_eq!(Ok(vec![Dice { count: -1, sides: 20 }]), DiceTerm::parse("-d20"));
        assert_eq!(Ok(vec![Dice { count: -1, sides: 20 }]), DiceTerm::parse("- d20"));
        assert_eq!(Ok(vec![Dice { count: -1, sides: 20 }]), DiceTerm::parse("- 1d20"));
        assert_eq!(Ok(vec![Dice { count: 3, sides: 8 }]), DiceTerm::parse("3d8"));
    }

    #[test]
    fn test_compound_dice_parsing() {
        assert_eq!(Ok(vec![Dice { count: 2, sides: 20 }, Constant(5)]), DiceTerm::parse("2d20 +5"));
        assert_eq!(Ok(vec![Dice { count: 2, sides: 20 }, Constant(-5)]), DiceTerm::parse("2d20 -5"));
        assert_eq!(Ok(vec![Dice { count: 2, sides: 20 }, Constant(5)]), DiceTerm::parse("2d20 + 5"));
        assert_eq!(Ok(vec![Dice { count: 2, sides: 20 }, Constant(-5)]), DiceTerm::parse("2d20 - 5"));
    }

    #[test]
    fn test_invalid_parsing() {
        assert!(DiceTerm::parse("no").is_err());
        assert!(DiceTerm::parse("2d20 + -5").is_err());
        assert!(DiceTerm::parse("2d20 + 5 + no").is_err());
        assert!(DiceTerm::parse("2c20").is_err());
    }

    #[test]
    fn test_dice_rolls() {
        let rolls = Dice { count: 1, sides: 20, }.roll();
        assert_eq!(1, rolls.len(), "1 die should have been rolled.");
        let roll = rolls[0];
        assert!(roll <= 20, "Roll {} should be no more than 20.", roll);
        assert!(roll >= 1, "Roll {} should be at least 1.", roll);
    }

    #[test]
    fn test_negative_dice_rolls() {
        let rolls = Dice { count: -3, sides: 6, }.roll();
        assert_eq!(3, rolls.len(), "3 dice should have been rolled.");

        for roll in rolls {
            assert!(roll >= -6, "Roll {} should be at least -18.", roll);
            assert!(roll <= -1, "Roll {} should be no more than -3.", roll);
        }
    }

    #[test]
    fn test_constant_rolls() {
        assert_eq!(vec![10], Constant(10).roll());
        assert_eq!(vec![-2], Constant(-2).roll());
    }

    #[test]
    fn test_dice_averages() {
        assert_eq!(10.5, Dice { count: 1, sides: 20 }.average());
        assert_eq!(-10.5, Dice { count: -3, sides: 6 }.average());
        assert_eq!(28.0, Dice { count: 8, sides: 6 }.average());
    }

    #[test]
    fn test_constant_averages() {
        assert_eq!(10.0, Constant(10).average());
        assert_eq!(-2.0, Constant(-2).average());
    }

    #[test]
    fn test_display() {
        assert_eq!("1d20", format!("{}", Dice { count: 1, sides: 20 }));
        assert_eq!("-3d8", format!("{}", Dice { count: -3, sides: 8 }));
        assert_eq!("5", format!("{}", Constant(5)));
        assert_eq!("-2", format!("{}", Constant(-2)));
    }
}
