use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{PathBuf};
use walkdir::WalkDir;
use doc2quarto::{process_files};

#[derive(Parser, Debug)]
#[command(name="doc2quarto")]
#[command(about="Converts markdown.md to Quarto .qmd format", long_about=None)]
pub struct Args {
    
    /// source directory containing Docusaurus markdown files
    #[arg(short, long)]
   pub source: PathBuf,

    /// destination directory for converted Quarto files
    #[arg(short, long)]
    pub dest: PathBuf,

}


pub fn main() {

    let args = Args::parse();
    println!("\n");
    println!("{}","Doc2Quarto - Docusaurus to Quarto Converter".bright_cyan().bold());
    println!("{}", "=".repeat(45).bright_black());

    // check if Source directory if exists
    if !args.source.exists() {
        eprintln!("{} Source directory does not exists: {:?}", "x".red(), args.source);
        std::process::exit(1);
    }

    //check if the destination directory exists
    // if !args.dest.exists() {
    //     eprintln!("{} Destination directory does not exists: {:?}", "x".red(), args.dest);
    //     println!("creating the directory");
    //     // create destination directory if it does not exist
    //     if let Err(e) = fs::create_dir_all(&args.dest) {
    //         eprintln!("{} Failed to create destination directory: {}", "x".red(), e);
    //         std::process::exit(1);
    //     }
    // }


    // Create destination directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&args.dest) {
        eprintln!("{} Failed to create destination directory: {}", "✗".red(), e);
        std::process::exit(1);
    }


    // collect all .md files from source director

    let md_files: Vec<PathBuf> = WalkDir::new(&args.source)
                 .into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
                .map(|e| e.path().to_path_buf())           
                .collect();

    if md_files.len() == 0 {
        eprintln!("{} No .md files found in source directory", "x".red());
        std::process::exit(1);
    }
    println!("{} Found {} .md files in source directory", "✓".green(), md_files.len());
    println!("\n{} Found {} markdown files", "ℹ".blue(), md_files.len());

    //create progress bar for visual feedback
    let pb = ProgressBar::new(md_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {post}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
   
    // let mut success_count = 0;
    // let error_count = 0;

    // Process each markdown file
    for md_file in md_files {
        let file_name = md_file.file_name().unwrap().to_string_lossy();
        pb.set_message(format!("Processing: {}", file_name));

        match process_files(&md_file, &args.source, &args.dest) {
            Ok(_) => {
                // success_count += 1;
                pb.println(format!("{} Processed: {}", "✓".green(), file_name));
            }
            Err(e) => {
                eprintln!("{} Failed to process file: {}", "x".red(), e);
                pb.inc(1);
            }   
        }
   
        pb.inc(1);

    }
    pb.finish_with_message("Conversion completed!");

    // Display Summary

} // end of function



