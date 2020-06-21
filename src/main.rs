use structopt::StructOpt;
mod arguments;

fn main() {
    let args = arguments::Arguments::from_args();
    println!("{:#?}", args);
}
