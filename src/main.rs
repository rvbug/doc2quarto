use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;

/// CLI tool to convert markdown .md to quarto .qmd format

#[derive(Parser, Debug)]
#[command(name="doc2quarto")]
#[command(about="Converts markdown.md to Quarto .qmd format", long_about=None)]
struct Args {
    /// source directory containing Docusaurus markdown files
    #[arg(short, long)]
    source: PathBuf,

    /// destination directory for converted Quarto files
    #[arg(short, long)]
    dest: PathBuf,

}


fn main() {

    /// Entry point for the doc2quarto CLI application.
    ///
    /// Orchestrates the conversion process from Docusaurus to Quarto format:
    /// 1. Parses command-line arguments
    /// 2. Validates source directory existence
    /// 3. Creates destination directory structure
    /// 4. Discovers all markdown files recursively
    /// 5. Processes each file with progress tracking
    /// 6. Reports conversion statistics
    ///
    /// # Exit Codes
    /// - 0: Success
    /// - 1: Source directory not found or destination creation failed


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
   
    let mut success_count = 0;
    let mut error_count = 0;


    // Process each markdown file
    for md_file in md_files {
        let file_name = md_file.file_name().unwrap().to_string_lossy();
        pb.set_message(format!("Processing: {}", file_name));

        match process_files(&md_file, &args.source, &args.dest) {
            Ok(_) => {
                success_count += 1;
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



fn process_files(source_file: &Path, source_root: &Path, dest_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    
    /// Processes a single markdown file from Docusaurus to Quarto format.
    ///
    /// This function handles the complete conversion pipeline for a single file:
    /// - Reads the source markdown file
    /// - Converts content (frontmatter and admonitions)
    /// - Preserves directory structure in destination
    /// - Changes file extension from .md to .qmd
    /// - Copies associated img folders
    ///
    /// # Arguments
    /// - `source_file`: Path to the source .md file
    /// - `source_root`: Root directory of the source files (for calculating relative paths)
    /// - `dest_root`: Root directory where converted files will be written
    ///
    /// # Returns
    /// - `Ok(())` on successful conversion and write
    /// - `Err` if file reading, path manipulation, or writing fails
    ///
    /// # Example
    /// ```rust
    /// process_file(
    ///     Path::new("docs/guide/intro.md"),
    ///     Path::new("docs"),
    ///     Path::new("output")
    /// )?;
    /// ```

   
    // Read the entire file  content as a String
    let content = fs::read_to_string(source_file)?;
    println!("  📖 Read {} bytes from {:?}", content.len(), source_file);

    // Convert the content from Docusaurus to Quarto format
    let converted = convert_content(&content);
    println!("  🔄 Converted content: {} bytes", converted.len());

    // Calculate the relative path from source root
    let relative_path = source_file.strip_prefix(source_root)?;
    println!("  📍 Relative path: {:?}", relative_path);

    // Create destination path with .qmd extension
    let mut dest_path = dest_root.join(relative_path);
    dest_path.set_extension("qmd");
    println!("  📝 Destination path: {:?}", dest_path);

    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
        println!("  📁 Created parent directory: {:?}", parent);
    }

    // Write converted content to destination file
    fs::write(&dest_path, converted)?;
    println!("  ✅ Written to: {:?}", dest_path);

    // Copy img folder if it exists in the same directory
    copy_img_folder(source_file, &dest_path)?;


    Ok(())
}

/// Convert Docusourus markdown content to Quarto format
fn convert_content(content: &str) -> String {

    let mut result = String::new();
    let mut in_frontmatter = false;
    let mut frontmatter_lines = Vec::new();

    
    // Process the file line by line
    for line in content.lines() {
        // Handle frontmatter (All YAML between these "---" markers)
        if line == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                continue;
            } else {
                // End of frontmatter - convert and add to result
                result.push_str("---\n");
                result.push_str(&convert_frontmatter(&frontmatter_lines));
                // result.push_str("---\n");
                frontmatter_lines.clear();
                continue;
            }
        }

        if in_frontmatter {
            // Collect frontmatter lines for processing
            frontmatter_lines.push(line);
        } else {
            // Convert admonitions in the content
            let converted_line = convert_admonitions(line);
            result.push_str(&converted_line);
            result.push('\n');
        }
    }

    result
}




fn convert_frontmatter(lines: &[&str]) -> String {

    let mut result = String::new();

    for line in lines {
        // Convert 'sidebar_position' to 'order'
        if line.trim().starts_with("sidebar_position") {
            let value = line.split(':').nth(1).unwrap_or("").trim();
            result.push_str(&format!("order: {}\n", value));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
 }


fn convert_admonitions(line: &str) -> String {

    // Regex to match Docusourus admonitions :::note :::tip etc
    let admonition_start = Regex::new(r"^:::(\w)+(.*)$").unwrap();
    let admonition_end = Regex::new(r"^:::$").unwrap();

    // Convert opening admonitin syntax
    if let Some(caps) = admonition_start.captures(line) {
        let admonition_type = &caps[1];
        let title = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");

        // Map Docusaurus admonitions to Quarto callout types
        let quarto_type = match admonition_type.to_lowercase().as_str() {
            "note" => "note",
            "tip" => "tip",
            "info" => "note",
            "caution" => "caution",
            "warning" => "warning",
            "danger" => "important",
            _ => admonition_type,
        };

        // Build Quarto callout syntax
        if title.is_empty() {
            format!(":::: {{{}}}", quarto_type)
        } else {
            format!(":::: {{.callout-{}}}\n## {}", quarto_type, title)
        }
     }

    // Conver closing admonition syntax
    else if admonition_end.is_match(line) {
        "::::".to_string()
    }
    // Return line unchanged if it is not admonition
    else {
        line.to_string()
    }
} //end of function

/// Copy the img folder from source to destination
fn copy_img_folder(source_file: &Path, dest_file: &Path) -> Result<(), std::io::Error> {

    // Get the parent directory of the source file
    if let Some(source_parent) = source_file.parent() {
        let img_folder = source_parent.join("img");
        
        // Check if img folder exists
        if img_folder.exists() && img_folder.is_dir() {
            // Get destination parent directory
            if let Some(dest_parent) = dest_file.parent() {
                let dest_img = dest_parent.join("img");
                
                // Create destination img folder
                fs::create_dir_all(&dest_img)?;
                
                // Copy all files from source img to dest img
                for entry in fs::read_dir(&img_folder)? {
                    let entry = entry?;
                    let file_name = entry.file_name();
                    let dest_file_path = dest_img.join(&file_name);
                    fs::copy(entry.path(), dest_file_path)?;
                }
            }
        }
    }


    Ok(())
    
}












