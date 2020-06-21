use structopt::StructOpt;
mod arguments;
mod dice;

fn main() {
    let args = arguments::Arguments::from_args();
    println!("{:#?}", args);
    println!("{:#?}", args.dice());
}
