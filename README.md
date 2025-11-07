# doc2quarto

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A CLI tool to convert Docusaurus markdown files to Quarto format, preserving structure and transforming syntax for seamless migration.

## Overview

`doc2quarto` automates the conversion of Docusaurus documentation to Quarto, handling:
- ✅ Frontmatter transformation (`sidebar_position` → `order`)
- ✅ Admonition conversion (`:::note` → Quarto callout blocks)
- ✅ Directory structure preservation
- ✅ Image folder copying
- ✅ Batch processing with progress tracking

## Why Migrate to Quarto?

As documentation needs evolve, Quarto offers:
- **Academic Publishing**: Native support for citations, cross-references, and LaTeX
- **Multi-format Output**: Generate PDF, HTML, DOCX from the same source
- **Jupyter Integration**: Embed executable code and visualizations
- **Flexibility**: Better suited for technical documentation and research outputs

Read more about the migration rationale in [this blog post](https://qubitai.in/qscribe-docs/posts/Quarto/quarto.html).

## Installation

### Option 1: Install from crates.io (Coming Soon)
```bash
cargo install doc2quarto
```

### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/doc2quarto.git
cd doc2quarto

# Build release binary
cargo build --release

# Binary will be at: ./target/release/doc2quarto
```

### Option 3: Install Locally
```bash
# Build and install to ~/.cargo/bin
cargo install --path .

# Now you can run 'doc2quarto' from anywhere
doc2quarto --help
```

## Usage

```bash
# Basic usage
doc2quarto --source ./docs --dest ./quarto-docs

# Using short flags
doc2quarto -s ./docs -d ./quarto-docs
```

### Arguments

| Flag | Short | Description | Required |
|------|-------|-------------|----------|
| `--source` | `-s` | Source directory containing Docusaurus markdown files | Yes |
| `--dest` | `-d` | Destination directory for Quarto files | Yes |

## Conversion Details

### Frontmatter Transformation

**Docusaurus:**
```yaml
---
title: "Getting Started"
sidebar_position: 1
---
```

**Quarto:**
```yaml
---
title: "Getting Started"
order: 1
---
```

### Admonition Conversion

**Docusaurus:**
```markdown
:::note Important Information
This is a note with a custom title
:::

:::warning
Be careful with this operation
:::
```

**Quarto:**
```markdown
:::: {.callout-note}
## Important Information
This is a note with a custom title
::::

:::: {.callout-warning}
Be careful with this operation
::::
```

### Supported Admonition Types

| Docusaurus | Quarto |
|------------|--------|
| `note` | `note` |
| `tip` | `tip` |
| `info` | `note` |
| `caution` | `caution` |
| `warning` | `warning` |
| `danger` | `important` |

## Example

### Before Conversion (Docusaurus)
```
docs/
├── intro.md
├── guide/
│   ├── quickstart.md
│   └── img/
│       └── diagram.png
└── api/
    └── reference.md
```

### After Conversion (Quarto)
```
quarto-docs/
├── intro.qmd
├── guide/
│   ├── quickstart.qmd
│   └── img/
│       └── diagram.png
└── api/
    └── reference.qmd
```

## Development

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Running Tests
```bash
cargo test
```

### Running with Development Build
```bash
cargo run -- --source ./test-docs --dest ./output
```

## Project Structure

```
doc2quarto/
├── src/
│   └── main.rs          # Main conversion logic
├── tests/
│   ├── integration_test.rs
│   └── fixtures/        # Test markdown files
├── Cargo.toml
└── README.md
```

## Limitations

- Content between admonitions and frontmatter is preserved as-is
- Custom Docusaurus components are not converted
- MDX features are not supported
- This is a one-time migration tool, not a continuous sync solution

## Roadmap

- [ ] Support for more frontmatter fields
- [ ] MDX component detection and warnings
- [ ] Dry-run mode
- [ ] Configuration file support
- [ ] Parallel processing for large documentation sets


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for CLI parsing
- Progress tracking via [Indicatif](https://github.com/console-rs/indicatif)
- Inspired by the need to bridge Docusaurus and Quarto ecosystems

---

**Note**: After migration, review the generated `.qmd` files and test rendering with Quarto before archiving your Docusaurus setup.
