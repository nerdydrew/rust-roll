use structopt::StructOpt;
mod arguments;
mod dice;

fn main() {
    let args = arguments::Arguments::from_args();
    run(&args);
}

fn run(args: &arguments::Arguments) {
    let rolled: i32 = args.dice()
        .into_iter()
        .map(|d| d.roll())
        .sum();
    println!("Rolled {}", rolled);
    if args.average {
        let average: f64 = args.dice()
            .into_iter()
            .map(|d| d.average())
            .sum();
        println!("Average: {}", average);
    }
}