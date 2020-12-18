use structopt::StructOpt;

mod day01;

#[derive(Debug, StructOpt)]
#[structopt(name = "Advent of Code 2020", about = "Yearly challenge")]
struct CommandLineParams {
    #[structopt(short = "-d", long)]
    day: i32,

    input_file: String,
}

fn main() {
    let params = CommandLineParams::from_args();

    println!("Advent of Code 2020");
    println!();

    match params.day {
        1 => match day01::solve(&params.input_file) {
            Ok(()) => (),
            Err(error) => {
                println!("Problem found: {:?}", error);
                std::process::exit(1);
            }
        },
        _ => {
            println!("Day {:?} not implemented.", params.day);
            std::process::exit(1);
        }
    }
}
