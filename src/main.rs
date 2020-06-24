mod arguments;
mod dice;

use structopt::StructOpt;

fn main() {
    let args = arguments::Arguments::from_args();
    if let Err(e) = dice::run(&args) {
        eprintln!("{}", e);
    }
}
