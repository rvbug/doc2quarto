use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name="doc2quarto")]
#[command(about="convert docusorous to quarto", long_about="** This tool helps to convert all your docusorus blogs to quarto format**")]
struct Args {
    // #[arg(...)] defines how each field is a CLI arguments
    
    #[arg(short, long, help="Source Docusaurus Directory")]
    source: PathBuf,

    #[arg(short, long, help="Target Quarto Directory")]
    target: PathBuf,

    #[arg(short, long, help="Dry run - do not write files")]
    dry_run: bool,

    #[arg(short, long, help="Verbose output")]
    verbose: bool,

}

fn main() -> Result<()> {

    let args = Args::parse();
    println!("Source: {:?}", args.source);
    println!("Target: {:?}", args.target);
    println!("Dry Run: {}", args.dry_run);
    println!("Verbose: {}", args.verbose);

    if args.dry_run {
        println!("Dry run is enabled, no files wil be written");
    }

    if args.verbose {
        println!("verbos mode enabled");
    }


    Ok(())
}
