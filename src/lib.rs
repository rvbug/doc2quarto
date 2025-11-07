/// Converts a Docusaurus markdown file to Quarto format.
///
/// This function reads the source file, transforms frontmatter and admonitions,
/// and writes the output to the destination with a .qmd extension. The directory
/// structure is preserved relative to the source root.
///
/// # Arguments
///
/// * `source_file` - Path to the source .md file
/// * `source_root` - Root directory of source files (for calculating relative paths)
/// * `dest_root` - Root directory where converted files will be written
///
/// # Returns
///
/// * `Ok(())` - Conversion successful
/// * `Err` - File reading, path manipulation, or writing failed
///
/// # Errors
///
/// Returns an error if:
/// - Source file cannot be read
/// - Path manipulation fails (e.g., source_file not under source_root)
/// - Destination directories cannot be created
/// - Output file cannot be written
///
use regex::Regex;
use std::fs;
use std::path::Path;


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
pub fn process_files(source_file: &Path, source_root: &Path, dest_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
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


/// Converts Docusaurus markdown content to Quarto format.
///
/// Performs two main transformations:
/// 1. Frontmatter: Converts Docusaurus YAML frontmatter to Quarto format
/// 2. Admonitions: Converts Docusaurus-style admonitions (:::note) to Quarto callout blocks
///
/// The function uses a state machine to track whether it's currently processing
/// frontmatter (between --- markers) or regular content.
///
/// # Arguments
/// - `content`: The complete content of the markdown file as a string
///
/// # Returns
/// A new String containing the converted content in Quarto format
pub fn convert_content(content: &str) -> String {

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

/// Converts Docusaurus frontmatter fields to Quarto equivalents.
///
/// Currently handles the following conversions:
/// - `sidebar_position` → `order`
/// - All other fields are preserved as-is
///
/// # Arguments
/// - `lines`: Slice of string slices representing frontmatter lines (without --- delimiters)
///
/// # Returns
/// A String containing the converted frontmatter (without --- delimiters)
///
/// # Note
/// Future enhancements could include additional field mappings such as:
/// - `sidebar_label` → `title` (if title is not present)
/// - Custom metadata transformations
pub fn convert_frontmatter(lines: &[&str]) -> String {
    
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


/// Converts a single line from Docusaurus admonition syntax to Quarto callout syntax.
///
/// Docusaurus uses `:::type Title` syntax, while Quarto uses `:::: {.callout-type}` syntax.
///
/// # Supported Admonition Types
/// - note → note
/// - tip → tip
/// - info → note
/// - caution → caution
/// - warning → warning
/// - danger → important
///
/// # Arguments
/// - `line`: A single line from the markdown file
///
/// # Returns
/// - Converted callout syntax if the line matches an admonition pattern
/// - Original line unchanged if no pattern matches
///

pub fn convert_admonitions(line: &str) -> String {
    

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


/// Copies the img folder from source directory to destination directory.
///
/// Docusaurus projects often have img folders alongside markdown files containing
/// referenced images. This function preserves that structure in the output.
///
/// # Arguments
/// - `source_file`: Path to the source markdown file
/// - `dest_file`: Path to the destination markdown file
///
/// # Returns
/// - `Ok(())` if img folder doesn't exist or is successfully copied
/// - `Err` if directory creation or file copying fails
///
/// # Behavior
/// - If no img folder exists in the source directory, the function succeeds silently
/// - If img folder exists, creates it in destination and copies all files
/// - Preserves original filenames
///
pub fn copy_img_folder(source_file: &Path, dest_file: &Path) -> Result<(), std::io::Error> {
    
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













