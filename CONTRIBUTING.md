# Contributing to doc2quarto

Thank you for your interest in contributing! 🎉

## How to Contribute

### Reporting Bugs
1. Check if the bug has already been reported in [Issues](https://github.com/rvbug/doc2quarto/issues)
2. If not, create a new issue using the Bug Report template
3. Provide as much detail as possible

### Suggesting Features
1. Check if the feature has already been suggested in [Issues](https://github.com/rvbug/doc2quarto/issues)
2. Create a new issue using the Feature Request template
3. Explain your use case clearly

### Contributing Code

#### First Time Contributors
1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/doc2quarto.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test: `cargo test`
6. Commit: `git commit -m "Add your feature"`
7. Push: `git push origin feature/your-feature-name`
8. Create a Pull Request

#### Development Setup
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/doc2quarto.git
cd doc2quarto

# Build
cargo build

# Run tests
cargo test

# Run with your changes
cargo run -- --source ./test --dest ./output
```

#### Code Style
- Follow Rust standard formatting: `cargo fmt`
- Check for issues: `cargo clippy`
- Add tests for new functionality
- Update documentation for public APIs

#### Commit Messages
- Use present tense: "Add feature" not "Added feature"
- Use imperative mood: "Fix bug" not "Fixes bug"
- Reference issues: "Fix #123: Resolve path handling issue"

### Pull Request Process
1. Ensure all tests pass: `cargo test`
2. Update the README.md if needed
3. Add tests for new features
4. One feature/fix per PR
5. Respond to review feedback

## Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Assume good intentions

## Questions?
Feel free to open an issue for any questions!

## License
By contributing, you agree that your contributions will be licensed under the MIT License.
