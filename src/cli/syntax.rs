//! Syntax highlighting for code blocks
//!
//! Uses syntect to provide syntax highlighting for code blocks in presentations.

use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;

/// A highlighted text segment with color
#[derive(Debug, Clone)]
pub struct HighlightedSegment {
    pub text: String,
    pub color: String, // RGB hex color
    pub bold: bool,
    pub italic: bool,
}

/// Highlight code with syntax coloring
pub fn highlight_code(code: &str, language: &str) -> Vec<Vec<HighlightedSegment>> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    // Map common language names to syntect syntax names
    let syntax_name = match language.to_lowercase().as_str() {
        "rust" | "rs" => "Rust",
        "python" | "py" => "Python",
        "javascript" | "js" => "JavaScript",
        "typescript" | "ts" => "TypeScript",
        "java" => "Java",
        "c" => "C",
        "cpp" | "c++" => "C++",
        "csharp" | "c#" | "cs" => "C#",
        "go" => "Go",
        "ruby" | "rb" => "Ruby",
        "php" => "PHP",
        "swift" => "Swift",
        "kotlin" | "kt" => "Kotlin",
        "scala" => "Scala",
        "html" => "HTML",
        "css" => "CSS",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "xml" => "XML",
        "sql" => "SQL",
        "bash" | "sh" | "shell" => "Bourne Again Shell (bash)",
        "powershell" | "ps1" => "PowerShell",
        "markdown" | "md" => "Markdown",
        "toml" => "TOML",
        _ => "Plain Text",
    };
    
    let syntax = ps.find_syntax_by_name(syntax_name)
        .or_else(|| ps.find_syntax_by_extension(language))
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    
    // Use Solarized (dark) theme for vibrant syntax colors
    let theme = &ts.themes["Solarized (dark)"];
    let mut highlighter = HighlightLines::new(syntax, theme);
    
    let mut lines = Vec::new();
    
    for line in code.lines() {
        let ranges = highlighter.highlight_line(line, &ps).unwrap_or_default();
        let segments: Vec<HighlightedSegment> = ranges.iter().map(|(style, text)| {
            HighlightedSegment {
                text: text.to_string(),
                color: style_to_hex(style),
                bold: style.font_style.contains(syntect::highlighting::FontStyle::BOLD),
                italic: style.font_style.contains(syntect::highlighting::FontStyle::ITALIC),
            }
        }).collect();
        lines.push(segments);
    }
    
    lines
}

/// Convert syntect Style to hex color
fn style_to_hex(style: &Style) -> String {
    format!("{:02X}{:02X}{:02X}", style.foreground.r, style.foreground.g, style.foreground.b)
}

/// Generate PPTX XML for highlighted code
pub fn generate_highlighted_code_xml(code: &str, language: &str) -> String {
    let highlighted = highlight_code(code, language);
    let mut xml = String::new();
    
    for line_segments in highlighted {
        xml.push_str("<a:p><a:pPr algn=\"l\"/>");
        
        if line_segments.is_empty() {
            // Empty line - use Solarized base0 color
            xml.push_str(r#"<a:r><a:rPr lang="en-US" sz="1400" dirty="0"><a:latin typeface="Consolas"/><a:solidFill><a:srgbClr val="839496"/></a:solidFill></a:rPr><a:t> </a:t></a:r>"#);
        } else {
            for segment in line_segments {
                let bold = if segment.bold { r#" b="1""# } else { "" };
                let italic = if segment.italic { r#" i="1""# } else { "" };
                let text = escape_xml(&segment.text);
                
                xml.push_str(&format!(
                    r#"<a:r><a:rPr lang="en-US" sz="1400" dirty="0"{}{}><a:latin typeface="Consolas"/><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:rPr><a:t>{}</a:t></a:r>"#,
                    bold, italic, segment.color, text
                ));
            }
        }
        
        xml.push_str("</a:p>");
    }
    
    xml
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let highlighted = highlight_code(code, "rust");
        assert_eq!(highlighted.len(), 3);
        assert!(!highlighted[0].is_empty());
    }

    #[test]
    fn test_highlight_python() {
        let code = "def hello():\n    print('Hello')";
        let highlighted = highlight_code(code, "python");
        assert_eq!(highlighted.len(), 2);
    }

    #[test]
    fn test_highlight_unknown() {
        let code = "some text";
        let highlighted = highlight_code(code, "unknown");
        assert_eq!(highlighted.len(), 1);
    }

    #[test]
    fn test_generate_xml() {
        let xml = generate_highlighted_code_xml("let x = 1;", "rust");
        assert!(xml.contains("<a:p>"));
        assert!(xml.contains("Consolas"));
    }
}
