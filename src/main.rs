use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,

    #[arg(value_name = "TEXT", help = "Input text")]
    inputs: Option<Vec<String>>,

}

fn main() {
    let args = Args::parse();
    if args.inputs.is_some() {
        let inputs = args.inputs.unwrap().join(" ");
        if args.omit_newline {
            print!("{}", inputs);
        }
        else {
            println!("{}", inputs);
        }
    }
    else if !args.omit_newline {
        println!();
    }
}
