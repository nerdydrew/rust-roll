use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Arguments {
    /// Also calculate the expected average value
    #[structopt(long="avg")]
    pub average: bool,

    /// The dice rolls to calculate (like `2d4` or `d20+5`)
    #[structopt(name="dice", required=true)]
    pub dice_terms: Vec<String>
}
