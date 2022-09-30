use clap::Parser;

/// {{name}} command-line application.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Who to greet
    #[arg(default_value = "world")]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
}
