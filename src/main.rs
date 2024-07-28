use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,

    #[arg(value_name = "TEXT", required = true, help = "Input text")]
    inputs: Vec<String>,

}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);
}
