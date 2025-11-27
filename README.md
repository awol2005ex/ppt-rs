# PPTX-RS

A Rust library for creating, reading, and updating PowerPoint 2007+ (.pptx) files.

## Features

- Create new presentations from scratch
- Read and modify existing .pptx files
- Add slides, shapes, text, and images
- Manipulate presentation properties
- Support for charts, tables, and media
- Full XML manipulation capabilities
- Type-safe enumeration system

### Module Structure

- **`core`** - Core traits (`ToXml`, `Positioned`, `Styled`) and XML utilities
- **`generator`** - PPTX generation with slides, tables, charts, images
- **`api`** - High-level `Presentation` builder API
- **`opc`** - Open Packaging Convention (ZIP) handling
- **`integration`** - High-level builders (`PresentationBuilder`, `SlideBuilder`)
- **`cli`** - Command-line interface

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Usage

### Creating a Presentation

#### Using the CLI

```bash
# Create a simple presentation
cargo run -- create my_presentation.pptx

# Create with custom title and slides
cargo run -- create my_presentation.pptx --title "My Title" --slides 5
```

#### Using the Library - Simple Presentation

```rust
use pptx_rs::generator;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a PPTX with 5 slides
    let pptx_data = generator::create_pptx("My Presentation", 5)?;
    
    // Write to file
    fs::write("presentation.pptx", pptx_data)?;
    
    Ok(())
}
```

#### Using the Library - Complex Presentation with Content

```rust
use pptx_rs::generator::{SlideContent, create_pptx_with_content};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create slides with content
    let slides = vec![
        SlideContent::new("Introduction")
            .add_bullet("Welcome to the presentation")
            .add_bullet("Today's agenda"),
        SlideContent::new("Key Points")
            .add_bullet("First important point")
            .add_bullet("Second important point")
            .add_bullet("Third important point"),
        SlideContent::new("Conclusion")
            .add_bullet("Summary of key takeaways")
            .add_bullet("Next steps"),
    ];
    
    // Generate PPTX with content
    let pptx_data = create_pptx_with_content("My Presentation", slides)?;
    
    // Write to file
    fs::write("presentation.pptx", pptx_data)?;
    
    Ok(())
}
```

#### Slide Layouts

The library supports multiple slide layouts for different presentation needs:

```rust
use pptx_rs::generator::{SlideContent, SlideLayout, create_pptx_with_content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slides = vec![
        // Title only (no content area)
        SlideContent::new("Title Slide")
            .layout(SlideLayout::TitleOnly),
        
        // Centered title (good for cover slides)
        SlideContent::new("Cover Slide")
            .layout(SlideLayout::CenteredTitle)
            .title_size(60),
        
        // Standard layout (title + content)
        SlideContent::new("Standard Layout")
            .add_bullet("Content point 1")
            .add_bullet("Content point 2")
            .layout(SlideLayout::TitleAndContent),
        
        // Large content area (smaller title)
        SlideContent::new("Big Content")
            .add_bullet("More space for content")
            .add_bullet("Maximized content area")
            .layout(SlideLayout::TitleAndBigContent),
        
        // Two column layout
        SlideContent::new("Comparison")
            .add_bullet("Left column content")
            .add_bullet("Right column content")
            .layout(SlideLayout::TwoColumn),
        
        // Blank slide
        SlideContent::new("")
            .layout(SlideLayout::Blank),
    ];
    
    let pptx_data = create_pptx_with_content("Layout Demo", slides)?;
    std::fs::write("layouts.pptx", pptx_data)?;
    
    Ok(())
}
```

**Available Layouts:**
- `TitleOnly` - Title at top, no content area
- `CenteredTitle` - Title centered on slide (good for cover slides)
- `TitleAndContent` - Standard layout with title and bullet points (default)
- `TitleAndBigContent` - Smaller title, larger content area
- `TwoColumn` - Title at top, content split into left and right columns (bullets auto-split)
- `Blank` - Empty slide

#### Creating Tables

```rust
use pptx_rs::generator::{SlideContent, Table, TableRow, TableCell, TableBuilder, create_pptx_with_content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a table using builder
    let table = TableBuilder::new(vec![2000000, 2000000, 2000000])
        .position(500000, 1500000)
        .add_simple_row(vec!["Name", "Department", "Status"])
        .add_simple_row(vec!["Alice", "Engineering", "Active"])
        .add_simple_row(vec!["Bob", "Sales", "Active"])
        .build();
    
    // Create slide with table
    let slides = vec![
        SlideContent::new("Employee Data")
            .table(table),
    ];
    
    let pptx_data = create_pptx_with_content("Tables Demo", slides)?;
    std::fs::write("tables.pptx", pptx_data)?;
    
    Ok(())
}
```

**Table Features:**
- Simple table creation with `TableBuilder`
- Cell formatting: bold, background colors
- Automatic column width management
- Positioned anywhere on slide

#### Creating Charts

```rust
use pptx_rs::generator::{ChartBuilder, ChartType, ChartSeries};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a bar chart
    let chart = ChartBuilder::new("Sales by Quarter", ChartType::Bar)
        .categories(vec!["Q1", "Q2", "Q3", "Q4"])
        .add_series(ChartSeries::new("2023", vec![100.0, 150.0, 120.0, 200.0]))
        .add_series(ChartSeries::new("2024", vec![120.0, 180.0, 160.0, 240.0]))
        .position(500000, 1500000)
        .size(5000000, 3500000)
        .build();
    
    // Chart is ready to be integrated into slides
    Ok(())
}
```

#### Reading PPTX Files

```rust
use pptx_rs::opc::package::Package;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open an existing PPTX file
    let package = Package::open("presentation.pptx")?;
    
    // Inspect package contents
    println!("Total parts: {}", package.part_count());
    
    // Get specific parts
    if let Some(content) = package.get_part("ppt/presentation.xml") {
        println!("Presentation XML: {} bytes", content.len());
    }
    
    // List all parts
    for path in package.part_paths() {
        println!("Part: {}", path);
    }
    
    Ok(())
}
```

### PPTX Generation Approach

The library generates proper Microsoft PowerPoint 2007+ (.pptx) files by:

1. **Creating a complete ZIP package** with all required ECMA-376 compliant components
2. **Generating XML documents** for presentation, slides, layouts, masters, and themes
3. **Managing relationships** between all package parts
4. **Including metadata** (title, creation date, slide count, etc.)
5. **Packaging into ZIP** with proper compression and structure

The generated files are:
- Valid Microsoft PowerPoint 2007+ format (recognized by `file` command)
- Readable by PowerPoint, LibreOffice, Google Slides, and other Office applications
- Fully compliant with ECMA-376 Office Open XML standard

See [ARCHITECTURE.md](ARCHITECTURE.md#pptx-generation-approach) for detailed technical documentation.

## Architecture

The library follows a layered architecture:

1. **API Layer** (`api.rs`) - User-facing functions
2. **Package Layer** (`package.rs`) - ZIP file handling
3. **Parts Layer** (`parts/`) - Individual package components
4. **OXML Layer** (`oxml/`) - XML element manipulation
5. **Utility Layer** (`util.rs`, `shared.rs`) - Common utilities

## Dependencies

- `zip` - ZIP file handling
- `xml-rs` - XML parsing
- `image` - Image handling
- `serde` - Serialization
- `thiserror` - Error handling
- `uuid` - Unique identifiers
- `regex` - Regular expressions
- `chrono` - Date/time handling

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## References

- [ECMA-376 Office Open XML Standard](http://www.ecma-international.org/publications/standards/Ecma-376.htm)
- [Microsoft Office Open XML Formats](https://docs.microsoft.com/en-us/office/open-xml/open-xml-overview)
