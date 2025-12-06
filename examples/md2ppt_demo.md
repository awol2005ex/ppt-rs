# Welcome to MD2PPT

A comprehensive demonstration of Markdown to PowerPoint conversion

- Convert markdown files to professional presentations
- Support for tables, code, diagrams, and more
- Simple CLI: `pptcli md2ppt slides.md output.pptx`

> This is a speaker note! It will appear in the notes section of PowerPoint, not on the slide itself. Use blockquotes for presenter notes.

# Text Formatting

Markdown supports various text styles:

- **Bold text** for emphasis
- *Italic text* for subtle emphasis
- ***Bold and italic*** combined
- `inline code` for technical terms
- Regular text for normal content

## Subheadings Become Bold Bullets

When you use ## or ### headings within a slide, they become bold bullet points. This is useful for organizing content hierarchically.

# Bullet Points & Lists

Different bullet styles all work:

- Dash bullet point
- Another dash point
* Asterisk bullet point
* Another asterisk point
+ Plus bullet point
+ Another plus point

# Data Tables

Tables are automatically styled with headers:

| Feature | Status | Priority |
|---------|--------|----------|
| Tables | Done | High |
| Mermaid | Done | High |
| Code Blocks | Done | Medium |
| Speaker Notes | Done | Medium |
| Images | Placeholder | Low |

# Sales Report Table

| Quarter | Revenue | Growth | Target |
|---------|---------|--------|--------|
| Q1 2024 | $1.2M | +15% | $1.0M |
| Q2 2024 | $1.5M | +25% | $1.3M |
| Q3 2024 | $1.8M | +20% | $1.6M |
| Q4 2024 | $2.1M | +17% | $2.0M |

> Highlight the Q2 growth rate - it was our best quarter!

# Code Examples

Here's a Rust code example:

```rust
fn main() {
    let message = "Hello, PowerPoint!";
    println!("{}", message);
    
    for i in 1..=5 {
        println!("Slide {}", i);
    }
}
```

# Python Code

```python
def create_presentation(title, slides):
    """Create a PowerPoint presentation."""
    pptx = Presentation()
    
    for slide_content in slides:
        slide = pptx.add_slide()
        slide.title = slide_content['title']
        
    return pptx

# Usage
slides = [
    {'title': 'Introduction'},
    {'title': 'Main Content'},
    {'title': 'Conclusion'}
]
create_presentation("Demo", slides)
```

# System Architecture

```mermaid
flowchart TB
    subgraph Client
        A[Web Browser]
        B[Mobile App]
    end
    
    subgraph Backend
        C[API Gateway]
        D[Auth Service]
        E[Data Service]
    end
    
    subgraph Storage
        F[(PostgreSQL)]
        G[(Redis Cache)]
    end
    
    A --> C
    B --> C
    C --> D
    C --> E
    D --> F
    E --> F
    E --> G
```

# User Authentication Flow

```mermaid
sequenceDiagram
    participant U as User
    participant C as Client
    participant A as Auth Server
    participant D as Database
    
    U->>C: Enter credentials
    C->>A: POST /login
    A->>D: Validate user
    D-->>A: User data
    A-->>C: JWT Token
    C-->>U: Login success
    
    Note over U,D: Token valid for 24 hours
```

# Project Timeline

```mermaid
gantt
    title Product Development Schedule
    dateFormat YYYY-MM-DD
    
    section Planning
    Requirements    :done, req, 2024-01-01, 14d
    Design          :done, des, after req, 21d
    
    section Development
    Backend API     :active, api, 2024-02-05, 45d
    Frontend UI     :active, ui, 2024-02-12, 40d
    Integration     :int, after api, 14d
    
    section Testing
    QA Testing      :qa, after int, 21d
    UAT             :uat, after qa, 14d
    
    section Launch
    Deployment      :dep, after uat, 7d
```

# Data Model

```mermaid
erDiagram
    USER ||--o{ ORDER : places
    USER {
        int id PK
        string name
        string email
        date created_at
    }
    ORDER ||--|{ LINE_ITEM : contains
    ORDER {
        int id PK
        int user_id FK
        date order_date
        decimal total
    }
    PRODUCT ||--o{ LINE_ITEM : "ordered in"
    PRODUCT {
        int id PK
        string name
        decimal price
        int stock
    }
    LINE_ITEM {
        int id PK
        int order_id FK
        int product_id FK
        int quantity
    }
```

# Application States

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Loading: fetch data
    Loading --> Success: data received
    Loading --> Error: request failed
    Success --> Idle: reset
    Error --> Loading: retry
    Error --> Idle: cancel
    Success --> [*]
```

# Class Structure

```mermaid
classDiagram
    class Presentation {
        +String title
        +List~Slide~ slides
        +addSlide()
        +save()
    }
    
    class Slide {
        +String title
        +List~Element~ elements
        +addElement()
    }
    
    class Element {
        <<interface>>
        +render()
    }
    
    class TextBox {
        +String text
        +render()
    }
    
    class Table {
        +List~Row~ rows
        +render()
    }
    
    Presentation "1" --> "*" Slide
    Slide "1" --> "*" Element
    Element <|-- TextBox
    Element <|-- Table
```

# User Journey

```mermaid
journey
    title User Onboarding Experience
    section Discovery
      Visit website: 5: User
      Read features: 4: User
      Watch demo: 5: User
    section Signup
      Create account: 3: User
      Verify email: 2: User
      Complete profile: 3: User
    section First Use
      Create first presentation: 4: User
      Export to PPTX: 5: User
      Share with team: 5: User
```

# Market Analysis

```mermaid
pie showData
    title Market Share 2024
    "Our Product" : 35
    "Competitor A" : 25
    "Competitor B" : 20
    "Competitor C" : 12
    "Others" : 8
```

# Feature Priorities

```mermaid
quadrantChart
    title Feature Priority Matrix
    x-axis Low Effort --> High Effort
    y-axis Low Impact --> High Impact
    quadrant-1 Plan carefully
    quadrant-2 Do first
    quadrant-3 Delegate
    quadrant-4 Quick wins
    Tables: [0.2, 0.9]
    Mermaid: [0.6, 0.85]
    Animations: [0.8, 0.4]
    Themes: [0.3, 0.5]
    Export PDF: [0.4, 0.7]
    3D Models: [0.9, 0.3]
```

# Development History

```mermaid
gitGraph
    commit id: "Initial"
    commit id: "Core API"
    branch feature/tables
    commit id: "Add tables"
    commit id: "Style tables"
    checkout main
    merge feature/tables
    branch feature/mermaid
    commit id: "Add mermaid"
    checkout main
    merge feature/mermaid
    commit id: "Release v1.0"
```

# Project Mindmap

```mermaid
mindmap
    root((MD2PPT))
        Input
            Markdown files
            GFM tables
            Mermaid diagrams
        Processing
            Parse markdown
            Convert elements
            Generate XML
        Output
            PPTX files
            Speaker notes
            Styled content
        Features
            Tables
            Code blocks
            Diagrams
            Formatting
```

# Project Timeline View

```mermaid
timeline
    title MD2PPT Development Timeline
    2024-01 : Project started
            : Core architecture
    2024-02 : Basic markdown parsing
            : Slide generation
    2024-03 : Table support
            : Code blocks
    2024-04 : Mermaid integration
            : Speaker notes
    2024-05 : Testing & polish
            : Documentation
            : Release v1.0
```

---

# Continuation Slides

When you use `---` (horizontal rule), it creates a continuation slide. This is useful for breaking up long content.

- This slide continues the previous topic
- The title shows "(continued)"
- Great for detailed explanations

# Mixed Content Slide

This slide demonstrates multiple content types:

- First, some bullet points
- With **bold** and *italic* text

| Item | Value |
|------|-------|
| A | 100 |
| B | 200 |

> Remember to explain the table data during the presentation!

# Technical Specifications

| Component | Technology | Version |
|-----------|------------|---------|
| Backend | Rust | 1.75+ |
| CLI | Clap | 4.5 |
| Markdown | pulldown-cmark | 0.10 |
| Archive | zip | 0.6 |
| Testing | insta | 1.34 |

```rust
// Example usage
use ppt_rs::cli::parse_markdown;

let slides = parse_markdown(content)?;
let pptx = create_pptx_with_content("Title", slides)?;
```

# Summary

## What We Covered

- Text formatting (bold, italic, code)
- Bullet points and lists
- Tables with automatic styling
- Code blocks with syntax labels
- 12 types of Mermaid diagrams
- Speaker notes from blockquotes
- Slide breaks with horizontal rules

## Key Benefits

- Simple markdown syntax
- Professional output
- No PowerPoint required
- Version control friendly

> Thank the audience and open for questions!

# Thank You!

Questions?

- GitHub: github.com/yingkitw/ppt-rs
- Documentation: See README.md
- Issues: Report bugs on GitHub

**Try it yourself:**

```bash
pptcli md2ppt presentation.md output.pptx
```
