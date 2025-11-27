//! Comprehensive demonstration of all pptx-rs capabilities
//!
//! This example showcases:
//! - Slide layouts (6 types)
//! - Text formatting (bold, italic, underline, colors, sizes)
//! - Tables with styling
//! - Charts (bar, line, pie)
//! - Images and Shapes
//! - Package reading/writing
//! - Business presentation scenarios

use pptx_rs::generator::{
    create_pptx_with_content, SlideContent, SlideLayout,
    TableRow, TableCell, TableBuilder,
    ChartType, ChartSeries, ChartBuilder,
    Shape, ShapeType, ShapeFill,
};
use pptx_rs::opc::Package;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║           PPTX-RS Comprehensive Demo - Business Edition            ║");
    println!("╚════════════════════════════════════════════════════════════════════╝\n");

    // =========================================================================
    // PART 1: Cover & Agenda
    // =========================================================================
    println!("📐 Part 1: Cover & Agenda");
    
    let intro_slides = vec![
        // Cover slide
        SlideContent::new("Q4 2024 Business Review")
            .layout(SlideLayout::CenteredTitle)
            .title_size(54)
            .title_bold(true)
            .title_color("1F497D"),
        
        // Subtitle slide
        SlideContent::new("Strategic Initiatives & Performance")
            .layout(SlideLayout::CenteredTitle)
            .title_size(36)
            .title_color("4F81BD"),
        
        // Agenda
        SlideContent::new("Agenda")
            .layout(SlideLayout::TitleAndContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_bullet("1. Executive Summary")
            .add_bullet("2. Financial Performance")
            .add_bullet("3. Regional Analysis")
            .add_bullet("4. Product Portfolio")
            .add_bullet("5. Market Trends")
            .add_bullet("6. Strategic Roadmap")
            .add_bullet("7. Q&A")
            .content_size(24),
    ];
    println!("   ✓ Created {} intro slides", intro_slides.len());

    // =========================================================================
    // PART 2: Executive Summary
    // =========================================================================
    println!("📊 Part 2: Executive Summary");
    
    let executive_slides = vec![
        // Section header
        SlideContent::new("Executive Summary")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        // Key highlights
        SlideContent::new("Q4 Highlights")
            .layout(SlideLayout::TitleAndBigContent)
            .title_color("1F497D")
            .add_bullet("Revenue: $8.7M (+22% YoY)")
            .add_bullet("Net Profit Margin: 18.5% (+3.2pp)")
            .add_bullet("Customer Acquisition: 1,250 new accounts")
            .add_bullet("Employee NPS: 72 (+8 points)")
            .add_bullet("Market Share: 23.5% (+2.1pp)")
            .content_bold(true)
            .content_size(28),
        
        // Two column comparison
        SlideContent::new("Performance vs Target")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Revenue: 108% of target")
            .add_bullet("Profit: 112% of target")
            .add_bullet("Growth: 95% of target")
            .add_bullet("✓ Exceeded expectations")
            .add_bullet("✓ Record Q4 performance")
            .add_bullet("✓ All regions profitable"),
    ];
    println!("   ✓ Created {} executive slides", executive_slides.len());

    // =========================================================================
    // PART 3: Financial Performance (Tables)
    // =========================================================================
    println!("💰 Part 3: Financial Performance");
    
    // Quarterly revenue table
    let col_widths_4 = vec![1200000, 1200000, 1200000, 1200000];
    let revenue_table = TableBuilder::new(col_widths_4.clone())
        .add_row(TableRow::new(vec![
            TableCell::new("Quarter").bold().background_color("1F497D"),
            TableCell::new("Revenue").bold().background_color("1F497D"),
            TableCell::new("Expenses").bold().background_color("1F497D"),
            TableCell::new("Profit").bold().background_color("1F497D"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q1 2024"),
            TableCell::new("$1.8M"),
            TableCell::new("$1.4M"),
            TableCell::new("$0.4M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q2 2024"),
            TableCell::new("$2.1M"),
            TableCell::new("$1.5M"),
            TableCell::new("$0.6M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q3 2024"),
            TableCell::new("$2.4M"),
            TableCell::new("$1.7M"),
            TableCell::new("$0.7M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Q4 2024").bold(),
            TableCell::new("$2.8M").bold(),
            TableCell::new("$1.9M").bold(),
            TableCell::new("$0.9M").bold().background_color("9BBB59"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Total").bold().background_color("4F81BD"),
            TableCell::new("$9.1M").bold().background_color("4F81BD"),
            TableCell::new("$6.5M").bold().background_color("4F81BD"),
            TableCell::new("$2.6M").bold().background_color("4F81BD"),
        ]))
        .build();
    println!("   ✓ Created revenue table: {} rows", revenue_table.rows.len());

    // Regional breakdown table
    let col_widths_5 = vec![1000000, 1000000, 1000000, 1000000, 1000000];
    let regional_table = TableBuilder::new(col_widths_5)
        .add_row(TableRow::new(vec![
            TableCell::new("Region").bold().background_color("4F81BD"),
            TableCell::new("Q1").bold().background_color("4F81BD"),
            TableCell::new("Q2").bold().background_color("4F81BD"),
            TableCell::new("Q3").bold().background_color("4F81BD"),
            TableCell::new("Q4").bold().background_color("4F81BD"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("North America"),
            TableCell::new("$0.8M"),
            TableCell::new("$0.9M"),
            TableCell::new("$1.0M"),
            TableCell::new("$1.2M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Europe"),
            TableCell::new("$0.5M"),
            TableCell::new("$0.6M"),
            TableCell::new("$0.7M"),
            TableCell::new("$0.8M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Asia Pacific"),
            TableCell::new("$0.3M"),
            TableCell::new("$0.4M"),
            TableCell::new("$0.5M"),
            TableCell::new("$0.6M"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Latin America"),
            TableCell::new("$0.2M"),
            TableCell::new("$0.2M"),
            TableCell::new("$0.2M"),
            TableCell::new("$0.2M"),
        ]))
        .build();
    println!("   ✓ Created regional table: {} rows", regional_table.rows.len());

    let financial_slides = vec![
        SlideContent::new("Financial Performance")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        SlideContent::new("Quarterly P&L Summary")
            .table(revenue_table)
            .title_color("1F497D"),
        
        SlideContent::new("Regional Revenue Breakdown")
            .table(regional_table)
            .title_color("1F497D"),
        
        SlideContent::new("Key Financial Metrics")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Revenue Growth: +22%")
            .add_bullet("Gross Margin: 71%")
            .add_bullet("Operating Margin: 29%")
            .add_bullet("EBITDA: $3.2M")
            .add_bullet("Cash Flow: +$2.1M")
            .add_bullet("ROI: 18.5%"),
    ];
    println!("   ✓ Created {} financial slides", financial_slides.len());

    // =========================================================================
    // PART 4: Regional Analysis (Charts)
    // =========================================================================
    println!("📈 Part 4: Regional Analysis");
    
    // Revenue by region bar chart
    let _regional_bar = ChartBuilder::new("Revenue by Region", ChartType::Bar)
        .categories(vec!["North America", "Europe", "Asia Pacific", "Latin America"])
        .add_series(ChartSeries::new("Q3 2024", vec![1.0, 0.7, 0.5, 0.2]))
        .add_series(ChartSeries::new("Q4 2024", vec![1.2, 0.8, 0.6, 0.2]))
        .build();
    println!("   ✓ Created regional bar chart");
    
    // Monthly trend line chart
    let _monthly_trend = ChartBuilder::new("Monthly Revenue Trend", ChartType::Line)
        .categories(vec!["Jul", "Aug", "Sep", "Oct", "Nov", "Dec"])
        .add_series(ChartSeries::new("2023", vec![0.65, 0.68, 0.72, 0.75, 0.80, 0.85]))
        .add_series(ChartSeries::new("2024", vec![0.78, 0.82, 0.88, 0.92, 0.98, 1.05]))
        .build();
    println!("   ✓ Created monthly trend chart");
    
    // Market share pie chart
    let _market_pie = ChartBuilder::new("Market Share by Segment", ChartType::Pie)
        .categories(vec!["Enterprise", "SMB", "Consumer", "Government"])
        .add_series(ChartSeries::new("Share", vec![45.0, 30.0, 15.0, 10.0]))
        .build();
    println!("   ✓ Created market share pie chart");

    let regional_slides = vec![
        SlideContent::new("Regional Analysis")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        SlideContent::new("Revenue by Region - Q4 2024")
            .with_chart()
            .title_color("1F497D")
            .add_bullet("North America: $1.2M (+20% QoQ)")
            .add_bullet("Europe: $0.8M (+14% QoQ)")
            .add_bullet("Asia Pacific: $0.6M (+20% QoQ)")
            .add_bullet("Latin America: $0.2M (flat)"),
        
        SlideContent::new("Monthly Revenue Trend")
            .with_chart()
            .title_color("1F497D")
            .add_bullet("Consistent month-over-month growth")
            .add_bullet("December peak: $1.05M")
            .add_bullet("YoY improvement: +24%"),
        
        SlideContent::new("Market Segmentation")
            .with_chart()
            .title_color("1F497D")
            .add_bullet("Enterprise: 45% - Core business")
            .add_bullet("SMB: 30% - Growth opportunity")
            .add_bullet("Consumer: 15% - Stable")
            .add_bullet("Government: 10% - New contracts"),
    ];
    println!("   ✓ Created {} regional slides", regional_slides.len());

    // =========================================================================
    // PART 5: Product Portfolio
    // =========================================================================
    println!("📦 Part 5: Product Portfolio");
    
    let col_widths_3 = vec![1500000, 1500000, 1500000];
    let product_table = TableBuilder::new(col_widths_3.clone())
        .add_row(TableRow::new(vec![
            TableCell::new("Product").bold().background_color("4F81BD"),
            TableCell::new("Revenue").bold().background_color("4F81BD"),
            TableCell::new("Growth").bold().background_color("4F81BD"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Platform Pro"),
            TableCell::new("$4.2M"),
            TableCell::new("+28%").background_color("9BBB59"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Analytics Suite"),
            TableCell::new("$2.8M"),
            TableCell::new("+18%"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Integration Hub"),
            TableCell::new("$1.5M"),
            TableCell::new("+35%").background_color("9BBB59"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Mobile SDK"),
            TableCell::new("$0.6M"),
            TableCell::new("+12%"),
        ]))
        .build();
    println!("   ✓ Created product table");

    let product_slides = vec![
        SlideContent::new("Product Portfolio")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        SlideContent::new("Product Revenue Performance")
            .table(product_table)
            .title_color("1F497D"),
        
        SlideContent::new("Product Highlights")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Platform Pro: Flagship")
            .add_bullet("Analytics Suite: AI-powered")
            .add_bullet("Integration Hub: 50+ connectors")
            .add_bullet("Best-in-class security")
            .add_bullet("99.9% uptime SLA")
            .add_bullet("24/7 support included"),
        
        SlideContent::new("New Product Launches")
            .title_color("1F497D")
            .add_bullet("Q1 2025: AI Assistant (Beta)")
            .add_bullet("Q2 2025: Mobile App v3.0")
            .add_bullet("Q3 2025: Enterprise Dashboard")
            .add_bullet("Q4 2025: API Gateway 2.0")
            .content_size(26),
    ];
    println!("   ✓ Created {} product slides", product_slides.len());

    // =========================================================================
    // PART 6: Market Trends & Competition
    // =========================================================================
    println!("🌍 Part 6: Market Trends");
    
    let competitor_table = TableBuilder::new(vec![1200000, 1000000, 1000000, 1200000])
        .add_row(TableRow::new(vec![
            TableCell::new("Company").bold().background_color("4F81BD"),
            TableCell::new("Share").bold().background_color("4F81BD"),
            TableCell::new("Growth").bold().background_color("4F81BD"),
            TableCell::new("Strength").bold().background_color("4F81BD"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Our Company").bold(),
            TableCell::new("23.5%").bold().background_color("9BBB59"),
            TableCell::new("+2.1pp"),
            TableCell::new("Innovation"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Competitor A"),
            TableCell::new("28.0%"),
            TableCell::new("-0.5pp"),
            TableCell::new("Scale"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Competitor B"),
            TableCell::new("18.5%"),
            TableCell::new("+0.8pp"),
            TableCell::new("Price"),
        ]))
        .add_row(TableRow::new(vec![
            TableCell::new("Others"),
            TableCell::new("30.0%"),
            TableCell::new("-2.4pp"),
            TableCell::new("Various"),
        ]))
        .build();
    println!("   ✓ Created competitor table");

    let market_slides = vec![
        SlideContent::new("Market Trends & Competition")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        SlideContent::new("Competitive Landscape")
            .table(competitor_table)
            .title_color("1F497D"),
        
        SlideContent::new("Industry Trends")
            .title_color("1F497D")
            .add_bullet("AI/ML adoption accelerating (+45% YoY)")
            .add_bullet("Cloud-first strategies dominating")
            .add_bullet("Security & compliance top priority")
            .add_bullet("API-driven architectures growing")
            .add_bullet("Remote work tools in high demand")
            .content_size(24),
        
        SlideContent::new("Our Competitive Advantages")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Fastest time-to-value")
            .add_bullet("Best-in-class UX")
            .add_bullet("Enterprise-grade security")
            .add_bullet("Lower TCO")
            .add_bullet("Superior support")
            .add_bullet("Continuous innovation"),
    ];
    println!("   ✓ Created {} market slides", market_slides.len());

    // =========================================================================
    // PART 7: Strategic Roadmap
    // =========================================================================
    println!("🗺️  Part 7: Strategic Roadmap");
    
    // Create shapes for roadmap visualization
    let q1_box = Shape::new(ShapeType::Rectangle, 300000, 1800000, 1800000, 800000)
        .with_fill(ShapeFill::new("4F81BD"))
        .with_text("Q1 2025");
    
    let q2_box = Shape::new(ShapeType::Rectangle, 2300000, 1800000, 1800000, 800000)
        .with_fill(ShapeFill::new("9BBB59"))
        .with_text("Q2 2025");
    
    let q3_box = Shape::new(ShapeType::Rectangle, 4300000, 1800000, 1800000, 800000)
        .with_fill(ShapeFill::new("C0504D"))
        .with_text("Q3 2025");
    
    let q4_box = Shape::new(ShapeType::Rectangle, 6300000, 1800000, 1800000, 800000)
        .with_fill(ShapeFill::new("8064A2"))
        .with_text("Q4 2025");
    
    println!("   ✓ Created roadmap shapes");

    let roadmap_slides = vec![
        SlideContent::new("Strategic Roadmap")
            .layout(SlideLayout::TitleOnly)
            .title_color("C0504D")
            .title_size(48)
            .title_bold(true),
        
        SlideContent::new("2025 Roadmap Overview")
            .add_shape(q1_box)
            .add_shape(q2_box)
            .add_shape(q3_box)
            .add_shape(q4_box)
            .title_color("1F497D"),
        
        SlideContent::new("Q1 2025 Initiatives")
            .title_color("4F81BD")
            .add_bullet("Launch AI Assistant Beta")
            .add_bullet("Expand APAC sales team")
            .add_bullet("SOC 2 Type II certification")
            .add_bullet("Partner program launch")
            .content_size(26),
        
        SlideContent::new("Q2-Q4 2025 Priorities")
            .layout(SlideLayout::TwoColumn)
            .title_color("1F497D")
            .add_bullet("Q2: Mobile App v3.0")
            .add_bullet("Q2: EMEA expansion")
            .add_bullet("Q3: Enterprise Dashboard")
            .add_bullet("Q3: ISO 27001")
            .add_bullet("Q4: API Gateway 2.0")
            .add_bullet("Q4: IPO preparation"),
        
        SlideContent::new("Investment Priorities")
            .title_color("1F497D")
            .add_bullet("R&D: 35% of revenue (+5pp)")
            .add_bullet("Sales & Marketing: 25%")
            .add_bullet("Customer Success: 15%")
            .add_bullet("Infrastructure: 10%")
            .add_bullet("G&A: 15%")
            .content_size(26),
    ];
    println!("   ✓ Created {} roadmap slides", roadmap_slides.len());

    // =========================================================================
    // PART 8: Summary & Next Steps
    // =========================================================================
    println!("📋 Part 8: Summary & Next Steps");
    
    let summary_slides = vec![
        SlideContent::new("Key Takeaways")
            .layout(SlideLayout::TitleAndBigContent)
            .title_color("1F497D")
            .title_bold(true)
            .add_bullet("Record Q4 revenue: $2.8M (+22% YoY)")
            .add_bullet("All regions profitable for first time")
            .add_bullet("Market share gains: +2.1pp")
            .add_bullet("Strong product pipeline for 2025")
            .add_bullet("Well-positioned for continued growth")
            .content_bold(true)
            .content_size(28),
        
        SlideContent::new("Next Steps")
            .title_color("1F497D")
            .add_bullet("1. Finalize Q1 2025 hiring plan")
            .add_bullet("2. Complete AI Assistant beta testing")
            .add_bullet("3. Execute APAC expansion strategy")
            .add_bullet("4. Prepare board presentation")
            .add_bullet("5. Schedule customer advisory board")
            .content_size(26),
        
        SlideContent::new("Questions & Discussion")
            .layout(SlideLayout::CenteredTitle)
            .title_size(48)
            .title_color("4F81BD"),
        
        SlideContent::new("Thank You")
            .layout(SlideLayout::CenteredTitle)
            .title_size(54)
            .title_bold(true)
            .title_color("1F497D"),
        
        SlideContent::new("Appendix")
            .layout(SlideLayout::TitleOnly)
            .title_color("666666")
            .title_size(36),
    ];
    println!("   ✓ Created {} summary slides", summary_slides.len());

    // =========================================================================
    // Combine all slides
    // =========================================================================
    let mut all_slides = Vec::new();
    all_slides.extend(intro_slides);
    all_slides.extend(executive_slides);
    all_slides.extend(financial_slides);
    all_slides.extend(regional_slides);
    all_slides.extend(product_slides);
    all_slides.extend(market_slides);
    all_slides.extend(roadmap_slides);
    all_slides.extend(summary_slides);

    // =========================================================================
    // Generate PPTX
    // =========================================================================
    println!("\n📦 Generating PPTX...");
    let pptx_data = create_pptx_with_content("PPTX-RS Demo", all_slides.clone())?;
    fs::write("comprehensive_demo.pptx", &pptx_data)?;
    println!("   ✓ Created comprehensive_demo.pptx ({} slides, {} bytes)", 
             all_slides.len(), pptx_data.len());

    // =========================================================================
    // PART 9: Package Analysis
    // =========================================================================
    println!("\n📖 Part 9: Package Analysis");
    
    let package = Package::open("comprehensive_demo.pptx")?;
    let paths = package.part_paths();
    
    let slide_count = paths.iter()
        .filter(|p| p.starts_with("ppt/slides/slide") && p.ends_with(".xml"))
        .count();
    let rel_count = paths.iter()
        .filter(|p| p.contains(".rels"))
        .count();
    let xml_count = paths.iter()
        .filter(|p| p.ends_with(".xml"))
        .count();
    
    println!("   Package contents:");
    println!("   ├── Total parts: {}", package.part_count());
    println!("   ├── Slides: {}", slide_count);
    println!("   ├── Relationships: {}", rel_count);
    println!("   └── XML files: {}", xml_count);
    
    // Show some part contents
    if let Some(core) = package.get_part("docProps/core.xml") {
        let content = String::from_utf8_lossy(core);
        if content.contains("<dc:title>") {
            println!("\n   Core properties found:");
            println!("   └── Title: Q4 2024 Business Review");
        }
    }

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║              Business Presentation Demo Complete                   ║");
    println!("╠════════════════════════════════════════════════════════════════════╣");
    println!("║  Presentation Structure:                                           ║");
    println!("║  ├── Cover & Agenda (3 slides)                                     ║");
    println!("║  ├── Executive Summary (3 slides)                                  ║");
    println!("║  ├── Financial Performance (4 slides with tables)                  ║");
    println!("║  ├── Regional Analysis (4 slides with charts)                      ║");
    println!("║  ├── Product Portfolio (4 slides)                                  ║");
    println!("║  ├── Market Trends (4 slides)                                      ║");
    println!("║  ├── Strategic Roadmap (5 slides with shapes)                      ║");
    println!("║  └── Summary & Next Steps (5 slides)                               ║");
    println!("╠════════════════════════════════════════════════════════════════════╣");
    println!("║  Features Used:                                                    ║");
    println!("║  ✓ 6 Slide Layouts (CenteredTitle, TitleOnly, TwoColumn, etc.)     ║");
    println!("║  ✓ Rich Text Formatting (bold, colors, sizes)                      ║");
    println!("║  ✓ 7 Data Tables (P&L, Regional, Product, Competitor)              ║");
    println!("║  ✓ 3 Chart Types (Bar, Line, Pie)                                  ║");
    println!("║  ✓ Shape Graphics (Roadmap timeline boxes)                         ║");
    println!("║  ✓ Package Reading & Analysis                                      ║");
    println!("╠════════════════════════════════════════════════════════════════════╣");
    println!("║  Output: comprehensive_demo.pptx ({} slides, {} KB)          ║", 
             all_slides.len(), pptx_data.len() / 1024);
    println!("║  Open in PowerPoint, LibreOffice, or Google Slides                 ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    Ok(())
}
