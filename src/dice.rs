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
            Dice{count, sides} => count * (sides + 1) / 2.0,
            Constant(constant) => constant as f64
        }
    }
}