use clap::Parser;
use firmex_rs::parsers::MRVL;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to firmware file
    #[arg(short, long)]
    file_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mrvl = MRVL::read(&args.file_path)?;

    println!("{}", mrvl);

    Ok(())
}
