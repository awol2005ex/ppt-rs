//! Enhanced Markdown parser for md2ppt
//!
//! Supports:
//! - Headings (# creates new slides)
//! - Bullet points (-, *, +)
//! - Numbered lists
//! - Tables (GFM style)
//! - Code blocks (``` fenced)
//! - Mermaid diagrams (```mermaid)
//! - Bold, italic, inline code
//! - Images (as placeholders)
//! - Horizontal rules (slide breaks)
//! - Speaker notes (> blockquotes)

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use crate::generator::{SlideContent, TableBuilder, TableRow, TableCell, Shape, ShapeType, ShapeFill, CodeBlock};

/// Mermaid diagram types
#[derive(Debug, Clone, Copy, PartialEq)]
enum MermaidType {
    Flowchart,
    Sequence,
    Gantt,
    ClassDiagram,
    StateDiagram,
    ErDiagram,
    Journey,
    Pie,
    Mindmap,
    Timeline,
    Quadrant,
    Git,
    Unknown,
}

/// Parse markdown content into slides
pub fn parse_markdown(content: &str) -> Result<Vec<SlideContent>, String> {
    let mut parser = MarkdownParser::new();
    parser.parse(content)
}

/// State machine for markdown parsing
struct MarkdownParser {
    slides: Vec<SlideContent>,
    current_slide: Option<SlideContent>,
    // Text accumulation
    current_text: String,
    // List state
    in_list: bool,
    list_items: Vec<String>,
    // Table state
    in_table: bool,
    table_rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
    in_table_head: bool,
    // Code block state
    in_code_block: bool,
    code_content: String,
    code_language: Option<String>,
    // Formatting state
    is_bold: bool,
    is_italic: bool,
    // Blockquote (speaker notes)
    in_blockquote: bool,
    blockquote_text: String,
    // Image state
    pending_image: Option<(String, String)>, // (url, alt)
}

impl MarkdownParser {
    fn new() -> Self {
        Self {
            slides: Vec::new(),
            current_slide: None,
            current_text: String::new(),
            in_list: false,
            list_items: Vec::new(),
            in_table: false,
            table_rows: Vec::new(),
            current_row: Vec::new(),
            current_cell: String::new(),
            in_table_head: false,
            in_code_block: false,
            code_content: String::new(),
            code_language: None,
            is_bold: false,
            is_italic: false,
            in_blockquote: false,
            blockquote_text: String::new(),
            pending_image: None,
        }
    }

    fn parse(&mut self, content: &str) -> Result<Vec<SlideContent>, String> {
        let options = Options::ENABLE_TABLES 
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS;
        
        let parser = Parser::new_ext(content, options);
        
        for event in parser {
            self.handle_event(event);
        }
        
        // Finalize any pending content
        self.finalize_current_slide();
        
        if self.slides.is_empty() {
            return Err("No slides found in markdown file".to_string());
        }
        
        Ok(std::mem::take(&mut self.slides))
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            // Headings create new slides
            Event::Start(Tag::Heading { level, .. }) => {
                if level == HeadingLevel::H1 {
                    self.finalize_current_slide();
                }
                self.current_text.clear();
            }
            Event::End(TagEnd::Heading(level)) => {
                let title = std::mem::take(&mut self.current_text).trim().to_string();
                if level == HeadingLevel::H1 {
                    self.current_slide = Some(SlideContent::new(&title));
                } else if let Some(ref mut slide) = self.current_slide {
                    // H2+ become bold bullets
                    let formatted = format!("**{}**", title);
                    *slide = slide.clone().add_bullet(&formatted);
                }
            }
            
            // Lists
            Event::Start(Tag::List(_)) => {
                self.in_list = true;
                self.list_items.clear();
            }
            Event::End(TagEnd::List(_)) => {
                self.in_list = false;
                self.flush_list_items();
            }
            Event::Start(Tag::Item) => {
                self.current_text.clear();
            }
            Event::End(TagEnd::Item) => {
                let item = std::mem::take(&mut self.current_text).trim().to_string();
                if !item.is_empty() {
                    self.list_items.push(item);
                }
            }
            
            // Tables
            Event::Start(Tag::Table(_)) => {
                self.in_table = true;
                self.table_rows.clear();
                self.in_table_head = false;
            }
            Event::End(TagEnd::Table) => {
                self.in_table = false;
                self.flush_table();
            }
            Event::Start(Tag::TableHead) => {
                self.in_table_head = true;
                self.current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                self.in_table_head = false;
                if !self.current_row.is_empty() {
                    self.table_rows.push(std::mem::take(&mut self.current_row));
                }
            }
            Event::Start(Tag::TableRow) => {
                self.current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !self.current_row.is_empty() {
                    self.table_rows.push(std::mem::take(&mut self.current_row));
                }
            }
            Event::Start(Tag::TableCell) => {
                self.current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                self.current_row.push(std::mem::take(&mut self.current_cell).trim().to_string());
            }
            
            // Code blocks
            Event::Start(Tag::CodeBlock(kind)) => {
                self.in_code_block = true;
                self.code_content.clear();
                self.code_language = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        if lang_str.is_empty() { None } else { Some(lang_str) }
                    }
                    _ => None,
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                self.in_code_block = false;
                self.flush_code_block();
            }
            
            // Blockquotes (speaker notes)
            Event::Start(Tag::BlockQuote) => {
                self.in_blockquote = true;
                self.blockquote_text.clear();
            }
            Event::End(TagEnd::BlockQuote) => {
                self.in_blockquote = false;
                self.flush_blockquote();
            }
            
            // Inline formatting
            Event::Start(Tag::Strong) => self.is_bold = true,
            Event::End(TagEnd::Strong) => self.is_bold = false,
            Event::Start(Tag::Emphasis) => self.is_italic = true,
            Event::End(TagEnd::Emphasis) => self.is_italic = false,
            Event::Code(code) => {
                // Inline code
                let formatted = format!("`{}`", code);
                self.push_text(&formatted);
            }
            
            // Images
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                self.pending_image = Some((dest_url.to_string(), title.to_string()));
            }
            Event::End(TagEnd::Image) => {
                if let Some((url, alt)) = self.pending_image.take() {
                    self.add_image_placeholder(&url, &alt);
                }
            }
            
            // Horizontal rule = slide break
            Event::Rule => {
                self.finalize_current_slide();
                // Create a new slide with continuation
                if let Some(last) = self.slides.last() {
                    let title = format!("{} (continued)", last.title);
                    self.current_slide = Some(SlideContent::new(&title));
                }
            }
            
            // Text content
            Event::Text(text) => {
                self.push_text(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                self.push_text(" ");
            }
            
            // Paragraphs
            Event::Start(Tag::Paragraph) => {
                if !self.in_list && !self.in_table && !self.in_blockquote && !self.in_code_block {
                    self.current_text.clear();
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if !self.in_list && !self.in_table && !self.in_blockquote && !self.in_code_block {
                    let text = std::mem::take(&mut self.current_text).trim().to_string();
                    if !text.is_empty() {
                        self.add_paragraph(&text);
                    }
                }
            }
            
            _ => {}
        }
    }

    fn push_text(&mut self, text: &str) {
        let formatted = if self.is_bold && self.is_italic {
            format!("***{}***", text)
        } else if self.is_bold {
            format!("**{}**", text)
        } else if self.is_italic {
            format!("*{}*", text)
        } else {
            text.to_string()
        };
        
        if self.in_code_block {
            self.code_content.push_str(text);
        } else if self.in_table {
            self.current_cell.push_str(&formatted);
        } else if self.in_blockquote {
            self.blockquote_text.push_str(&formatted);
        } else {
            self.current_text.push_str(&formatted);
        }
    }

    fn add_paragraph(&mut self, text: &str) {
        if let Some(ref mut slide) = self.current_slide {
            *slide = slide.clone().add_bullet(text);
        } else {
            // Create default slide
            let mut slide = SlideContent::new("Slide");
            slide = slide.add_bullet(text);
            self.current_slide = Some(slide);
        }
    }

    fn flush_list_items(&mut self) {
        if self.list_items.is_empty() {
            return;
        }
        
        let items = std::mem::take(&mut self.list_items);
        
        if let Some(ref mut slide) = self.current_slide {
            for item in items {
                *slide = slide.clone().add_bullet(&item);
            }
        } else {
            let mut slide = SlideContent::new("Slide");
            for item in items {
                slide = slide.add_bullet(&item);
            }
            self.current_slide = Some(slide);
        }
    }

    fn flush_table(&mut self) {
        if self.table_rows.is_empty() {
            return;
        }
        
        let rows = std::mem::take(&mut self.table_rows);
        
        // Calculate column widths (equal distribution)
        let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(1);
        let col_width = 8000000u32 / col_count as u32; // Total width ~8 inches
        let col_widths: Vec<u32> = vec![col_width; col_count];
        
        let mut builder = TableBuilder::new(col_widths);
        
        for (i, row_data) in rows.iter().enumerate() {
            let cells: Vec<TableCell> = row_data.iter().enumerate().map(|(_, cell_text)| {
                let mut cell = TableCell::new(cell_text);
                if i == 0 {
                    // Header row styling
                    cell = cell.bold().background_color("4472C4").text_color("FFFFFF");
                }
                cell
            }).collect();
            
            // Pad row if needed
            let mut cells = cells;
            while cells.len() < col_count {
                cells.push(TableCell::new(""));
            }
            
            builder = builder.add_row(TableRow::new(cells));
        }
        
        let table = builder.position(500000, 1800000).build();
        
        if let Some(ref mut slide) = self.current_slide {
            slide.table = Some(table);
            slide.has_table = true;
        } else {
            let mut slide = SlideContent::new("Data Table");
            slide.table = Some(table);
            slide.has_table = true;
            self.current_slide = Some(slide);
        }
    }

    fn flush_code_block(&mut self) {
        if self.code_content.is_empty() {
            return;
        }
        
        let code = std::mem::take(&mut self.code_content);
        let lang = self.code_language.take();
        let lang_str = lang.as_deref().unwrap_or("text");
        
        // Check if this is a Mermaid diagram
        if lang_str == "mermaid" {
            self.add_mermaid_diagram(&code);
            return;
        }
        
        // Create a code block with syntax highlighting
        let code_block = CodeBlock::new(code.trim(), lang_str);
        
        if let Some(ref mut slide) = self.current_slide {
            slide.code_blocks.push(code_block);
        } else {
            let mut slide = SlideContent::new("Code");
            slide.code_blocks.push(code_block);
            self.current_slide = Some(slide);
        }
    }

    fn add_mermaid_diagram(&mut self, code: &str) {
        // Parse mermaid diagram type from the first line
        let diagram_type = Self::detect_mermaid_type(code);
        
        // Create appropriate shape based on diagram type
        let (fill_color, border_color, title, icon) = match diagram_type {
            MermaidType::Flowchart => ("E3F2FD", "1565C0", "Flowchart", "🔀"),
            MermaidType::Sequence => ("F3E5F5", "7B1FA2", "Sequence Diagram", "↔️"),
            MermaidType::Gantt => ("E8F5E9", "2E7D32", "Gantt Chart", "📅"),
            MermaidType::ClassDiagram => ("FFF3E0", "E65100", "Class Diagram", "📦"),
            MermaidType::StateDiagram => ("E0F7FA", "00838F", "State Diagram", "🔄"),
            MermaidType::ErDiagram => ("FCE4EC", "C2185B", "ER Diagram", "🗃️"),
            MermaidType::Journey => ("F1F8E9", "558B2F", "User Journey", "🚶"),
            MermaidType::Pie => ("FFF8E1", "FF8F00", "Pie Chart", "🥧"),
            MermaidType::Mindmap => ("E8EAF6", "3949AB", "Mind Map", "🧠"),
            MermaidType::Timeline => ("EFEBE9", "5D4037", "Timeline", "⏱️"),
            MermaidType::Quadrant => ("E1F5FE", "0277BD", "Quadrant Chart", "📊"),
            MermaidType::Git => ("ECEFF1", "455A64", "Git Graph", "🌿"),
            MermaidType::Unknown => ("F5F5F5", "757575", "Diagram", "📈"),
        };
        
        // Extract key elements from the mermaid code
        let elements = Self::extract_mermaid_elements(code, &diagram_type);
        
        // Create header shape with icon and title
        let header = Shape::new(ShapeType::Rectangle, 1000000, 1600000, 7000000, 600000)
            .with_fill(ShapeFill::new(border_color))
            .with_text(&format!("{} {}", icon, title));
        
        // Create body shape with extracted elements
        let body = Shape::new(ShapeType::Rectangle, 1000000, 2200000, 7000000, 3600000)
            .with_fill(ShapeFill::new(fill_color))
            .with_text(&elements);
        
        if let Some(ref mut slide) = self.current_slide {
            slide.shapes.push(header);
            slide.shapes.push(body);
        } else {
            let mut slide = SlideContent::new(title);
            slide.shapes.push(header);
            slide.shapes.push(body);
            self.current_slide = Some(slide);
        }
    }
    
    fn extract_mermaid_elements(code: &str, diagram_type: &MermaidType) -> String {
        match diagram_type {
            MermaidType::Flowchart => Self::extract_flowchart_elements(code),
            MermaidType::Sequence => Self::extract_sequence_elements(code),
            MermaidType::Pie => Self::extract_pie_elements(code),
            MermaidType::Gantt => Self::extract_gantt_elements(code),
            MermaidType::ErDiagram => Self::extract_er_elements(code),
            _ => Self::summarize_mermaid(code),
        }
    }
    
    fn extract_flowchart_elements(code: &str) -> String {
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        
        for line in code.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() || line.starts_with("%%") {
                continue;
            }
            
            // Extract node definitions like A[Text] or B((Circle))
            if let Some(bracket_start) = line.find('[') {
                if let Some(bracket_end) = line.find(']') {
                    let node_text = &line[bracket_start + 1..bracket_end];
                    if !nodes.contains(&node_text.to_string()) {
                        nodes.push(node_text.to_string());
                    }
                }
            }
            
            // Extract connections
            if line.contains("-->") || line.contains("---") || line.contains("==>") {
                connections.push("→".to_string());
            }
        }
        
        let nodes_str = if nodes.is_empty() {
            "No nodes defined".to_string()
        } else {
            format!("Nodes: {}", nodes.iter().take(6).cloned().collect::<Vec<_>>().join(" → "))
        };
        
        format!("{}\n\nConnections: {}", nodes_str, connections.len())
    }
    
    fn extract_sequence_elements(code: &str) -> String {
        let mut participants = Vec::new();
        let mut messages = 0;
        
        for line in code.lines().skip(1) {
            let line = line.trim();
            if line.starts_with("participant") {
                if let Some(name) = line.split_whitespace().nth(1) {
                    participants.push(name.to_string());
                }
            } else if line.contains("->>") || line.contains("-->>") || line.contains("-x") {
                messages += 1;
            }
        }
        
        format!("Participants: {}\n\nMessages: {}", 
            if participants.is_empty() { "Auto-detected".to_string() } else { participants.join(", ") },
            messages)
    }
    
    fn extract_pie_elements(code: &str) -> String {
        let mut slices = Vec::new();
        
        for line in code.lines().skip(1) {
            let line = line.trim();
            if line.contains(':') && !line.starts_with("title") {
                if let Some((label, _)) = line.split_once(':') {
                    slices.push(label.trim().trim_matches('"').to_string());
                }
            }
        }
        
        format!("Slices:\n{}", slices.iter().map(|s| format!("• {}", s)).collect::<Vec<_>>().join("\n"))
    }
    
    fn extract_gantt_elements(code: &str) -> String {
        let mut tasks = Vec::new();
        let mut sections = Vec::new();
        
        for line in code.lines().skip(1) {
            let line = line.trim();
            if line.starts_with("section") {
                sections.push(line.replace("section", "").trim().to_string());
            } else if line.contains(':') && !line.starts_with("title") && !line.starts_with("dateFormat") {
                if let Some((task, _)) = line.split_once(':') {
                    tasks.push(task.trim().to_string());
                }
            }
        }
        
        format!("Sections: {}\nTasks: {}", 
            if sections.is_empty() { "None".to_string() } else { sections.join(", ") },
            tasks.len())
    }
    
    fn extract_er_elements(code: &str) -> String {
        let mut entities = Vec::new();
        let mut relationships = 0;
        
        for line in code.lines().skip(1) {
            let line = line.trim();
            if line.contains("||") || line.contains("}|") || line.contains("|{") {
                relationships += 1;
                // Extract entity names
                let parts: Vec<&str> = line.split(|c| c == '|' || c == '{' || c == '}' || c == 'o').collect();
                for part in parts {
                    let entity = part.trim();
                    if !entity.is_empty() && entity.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        if !entities.contains(&entity.to_string()) {
                            entities.push(entity.to_string());
                        }
                    }
                }
            }
        }
        
        format!("Entities: {}\n\nRelationships: {}", 
            if entities.is_empty() { "None".to_string() } else { entities.join(", ") },
            relationships)
    }

    fn detect_mermaid_type(code: &str) -> MermaidType {
        let first_line = code.lines().next().unwrap_or("").trim().to_lowercase();
        
        if first_line.starts_with("graph") || first_line.starts_with("flowchart") {
            MermaidType::Flowchart
        } else if first_line.starts_with("sequencediagram") || first_line.starts_with("sequence") {
            MermaidType::Sequence
        } else if first_line.starts_with("gantt") {
            MermaidType::Gantt
        } else if first_line.starts_with("classdiagram") || first_line.starts_with("class") {
            MermaidType::ClassDiagram
        } else if first_line.starts_with("statediagram") || first_line.starts_with("state") {
            MermaidType::StateDiagram
        } else if first_line.starts_with("erdiagram") || first_line.starts_with("er") {
            MermaidType::ErDiagram
        } else if first_line.starts_with("journey") {
            MermaidType::Journey
        } else if first_line.starts_with("pie") {
            MermaidType::Pie
        } else if first_line.starts_with("mindmap") {
            MermaidType::Mindmap
        } else if first_line.starts_with("timeline") {
            MermaidType::Timeline
        } else if first_line.starts_with("quadrantchart") || first_line.starts_with("quadrant") {
            MermaidType::Quadrant
        } else if first_line.starts_with("gitgraph") || first_line.starts_with("git") {
            MermaidType::Git
        } else {
            MermaidType::Unknown
        }
    }

    fn summarize_mermaid(code: &str) -> String {
        // Extract key elements from mermaid code for display
        let lines: Vec<&str> = code.lines()
            .filter(|l| !l.trim().is_empty())
            .take(8)
            .collect();
        
        if lines.len() <= 6 {
            lines.join("\n")
        } else {
            format!("{}\n...\n({} more lines)", 
                lines[..5].join("\n"),
                code.lines().count() - 5)
        }
    }

    fn flush_blockquote(&mut self) {
        if self.blockquote_text.is_empty() {
            return;
        }
        
        let notes = std::mem::take(&mut self.blockquote_text).trim().to_string();
        
        if let Some(ref mut slide) = self.current_slide {
            // Add as speaker notes
            slide.notes = Some(notes);
        }
    }

    fn add_image_placeholder(&mut self, url: &str, alt: &str) {
        // Create an image placeholder shape
        let label = if alt.is_empty() { url } else { alt };
        
        let shape = Shape::new(ShapeType::Rectangle, 2000000, 2000000, 5000000, 3000000)
            .with_fill(ShapeFill::new("E0E0E0"))
            .with_text(&format!("[Image: {}]", label));
        
        if let Some(ref mut slide) = self.current_slide {
            slide.shapes.push(shape);
        } else {
            let mut slide = SlideContent::new("Image");
            slide.shapes.push(shape);
            self.current_slide = Some(slide);
        }
    }

    fn finalize_current_slide(&mut self) {
        // Flush any pending content
        self.flush_list_items();
        
        if let Some(slide) = self.current_slide.take() {
            self.slides.push(slide);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_headings() {
        let md = "# Slide 1\n- Bullet 1\n\n# Slide 2\n- Bullet 2";
        let slides = parse_markdown(md).unwrap();
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].title, "Slide 1");
        assert_eq!(slides[1].title, "Slide 2");
    }

    #[test]
    fn test_bullets() {
        let md = "# Test\n- Item 1\n- Item 2\n- Item 3";
        let slides = parse_markdown(md).unwrap();
        assert_eq!(slides[0].content.len(), 3);
    }

    #[test]
    fn test_table() {
        let md = "# Data\n\n| A | B |\n|---|---|\n| 1 | 2 |";
        let slides = parse_markdown(md).unwrap();
        assert!(slides[0].table.is_some());
    }

    #[test]
    fn test_code_block() {
        let md = "# Code\n\n```rust\nfn main() {}\n```";
        let slides = parse_markdown(md).unwrap();
        assert!(!slides[0].code_blocks.is_empty());
        assert_eq!(slides[0].code_blocks[0].language, "rust");
    }

    #[test]
    fn test_speaker_notes() {
        let md = "# Slide\n- Content\n\n> Speaker notes here";
        let slides = parse_markdown(md).unwrap();
        assert!(slides[0].notes.is_some());
    }

    #[test]
    fn test_formatting() {
        let md = "# Test\n- **Bold** and *italic*";
        let slides = parse_markdown(md).unwrap();
        assert!(slides[0].content[0].contains("**Bold**"));
    }

    #[test]
    fn test_mermaid_flowchart() {
        let md = "# Process\n\n```mermaid\nflowchart LR\n    A --> B --> C\n```";
        let slides = parse_markdown(md).unwrap();
        assert!(!slides[0].shapes.is_empty());
    }

    #[test]
    fn test_mermaid_sequence() {
        let md = "# Sequence\n\n```mermaid\nsequenceDiagram\n    Alice->>Bob: Hello\n```";
        let slides = parse_markdown(md).unwrap();
        assert!(!slides[0].shapes.is_empty());
    }

    #[test]
    fn test_mermaid_type_detection() {
        assert_eq!(MarkdownParser::detect_mermaid_type("flowchart LR"), MermaidType::Flowchart);
        assert_eq!(MarkdownParser::detect_mermaid_type("graph TD"), MermaidType::Flowchart);
        assert_eq!(MarkdownParser::detect_mermaid_type("sequenceDiagram"), MermaidType::Sequence);
        assert_eq!(MarkdownParser::detect_mermaid_type("gantt"), MermaidType::Gantt);
        assert_eq!(MarkdownParser::detect_mermaid_type("pie"), MermaidType::Pie);
        assert_eq!(MarkdownParser::detect_mermaid_type("classDiagram"), MermaidType::ClassDiagram);
        assert_eq!(MarkdownParser::detect_mermaid_type("stateDiagram"), MermaidType::StateDiagram);
        assert_eq!(MarkdownParser::detect_mermaid_type("erDiagram"), MermaidType::ErDiagram);
        assert_eq!(MarkdownParser::detect_mermaid_type("mindmap"), MermaidType::Mindmap);
        assert_eq!(MarkdownParser::detect_mermaid_type("timeline"), MermaidType::Timeline);
        assert_eq!(MarkdownParser::detect_mermaid_type("gitGraph"), MermaidType::Git);
    }
}
