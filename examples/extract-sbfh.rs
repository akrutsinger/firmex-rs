use anyhow::Context;
use clap::Parser;
use firmex_rs::parsers::SBFH;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to firmware file
    #[arg(short, long)]
    file_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sbfh = SBFH::read(&args.file_path)?;

    println!("{}", sbfh);

    let mut out_filename = args.file_path.clone();
    out_filename.push_str("_extracted.bin");

    let path = Path::new(&args.file_path);
    let path_dir = path.parent().context("not a directory")?.join(out_filename);

    let mut out_buf = File::create(&path_dir)?;
    out_buf.write(&sbfh.firmware_data)?;

    Ok(())
}
