use super::term::DiceTerm;
use std::fmt;

/// The result of dice rolls.
pub struct Roll {
    rolls: Vec<SingleRoll>
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rolled {}", self.total())
    }
}

impl Roll {
    /// Parses the given string as dice terms. Any unknown values are skipped.
    pub fn parse(s: &str) -> Roll {
        let rolls = DiceTerm::parse(s)
            .into_iter()
            .map(|term| SingleRoll {
                roll: term.roll(),
                term,
            })
            .collect();
        Roll { rolls }
    }

    pub fn total(&self) -> i32 {
        self.rolls.iter().map(|r| r.roll).sum()
    }

    pub fn average(&self) -> f64 {
        self.rolls.iter().map(|r| r.term.average()).sum()
    }
}

/// The rolled result of a single DiceTerm.
struct SingleRoll {
    term: DiceTerm,
    roll: i32
}

impl fmt::Display for SingleRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.term {
            DiceTerm::Dice { count, sides } => write!(f, "{}d{} ({})", count, sides, self.roll),
            DiceTerm::Constant(constant) => write!(f, "{}", constant)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_display() {
        let first_roll = SingleRoll { term: DiceTerm::Dice { count: 2, sides: 8 }, roll: 7 };
        let second_roll = SingleRoll { term: DiceTerm::Constant(-2), roll: -2 };
        let roll = Roll { rolls: vec![first_roll, second_roll] };
        assert_eq!("Rolled 5", format!("{}", roll));
    }

    #[test]
    fn test_single_roll_display() {
        let roll = SingleRoll { term: DiceTerm::Dice { count: -2, sides: 8 }, roll: -6 };
        assert_eq!("-2d8 (-6)", format!("{}", roll));

        let roll = SingleRoll { term: DiceTerm::Constant(5), roll: 5 };
        assert_eq!("5", format!("{}", roll));
    }
}