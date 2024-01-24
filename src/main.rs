use clap::Parser;


/// simple program to greet
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the person
    #[clap(short, long)]
    name: String,
    /// age of the person
    #[clap(short, long)]
    age: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.age {
        println!("Hello, {}!", args.name);
    }
}
