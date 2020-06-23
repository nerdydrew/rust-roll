use super::term::DiceTerm;

/// The result of dice rolls.
pub struct Roll {
    rolls: Vec<SingleRoll>
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
