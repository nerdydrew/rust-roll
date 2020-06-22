use structopt::StructOpt;
use crate::dice::term::DiceTerm;

#[derive(StructOpt, Debug)]
pub struct Arguments {
    /// Also calculate the expected average value
    #[structopt(long="avg")]
    pub average: bool,

    /// The dice rolls to calculate (like `2d4` or `d20+5`)
    #[structopt(required=true)]
    dice: Vec<String>
}

impl Arguments {
    pub fn dice(&self) -> Vec<DiceTerm> {
        DiceTerm::parse(&self.dice.join(" "))
    }
}