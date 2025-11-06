use doc2quarto::*;

#[test]
fn test_convert_frontmatter() {
    let input = vec![
        "title: \"Test\"",
        "sidebar_position: 1",
    ];
    
    let result = convert_frontmatter(&input);
    assert!(result.contains("order: 1"));
}
