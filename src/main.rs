mod arguments;
mod dice;

use structopt::StructOpt;

fn main() {
    let args = arguments::Arguments::from_args();
    dice::run(&args);
}
