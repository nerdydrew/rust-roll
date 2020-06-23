use self::roll::Roll;
use crate::arguments::Arguments;

mod roll;
mod term;

pub fn run(args: &Arguments) {
    let roll = Roll::parse(&args.dice_terms.join(" "));

    println!("Rolled {}", roll.total());
    if args.average {
        println!("Average: {}", roll.average());
    }
}
