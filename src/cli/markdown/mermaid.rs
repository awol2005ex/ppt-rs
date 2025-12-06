//! Mermaid diagram parsing and rendering
//!
//! Parses Mermaid diagram code and generates actual PPTX shapes and connectors.

use crate::generator::{Shape, ShapeType, ShapeFill, ShapeLine};
use crate::generator::connectors::{Connector, ConnectorType, ConnectorLine, ArrowType, LineDash};
use std::collections::HashMap;

/// Mermaid diagram types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MermaidType {
    Flowchart,
    Sequence,
    Pie,
    Gantt,
    ClassDiagram,
    StateDiagram,
    ErDiagram,
    Mindmap,
    Timeline,
    Unknown,
}

/// Direction of flowchart layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowDirection {
    LeftToRight,  // LR
    RightToLeft,  // RL
    TopToBottom,  // TB/TD
    BottomToTop,  // BT
}

/// A parsed flowchart node
#[derive(Debug, Clone)]
pub struct FlowNode {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
}

/// Node shape types in Mermaid
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeShape {
    Rectangle,      // [text]
    RoundedRect,    // (text)
    Stadium,        // ([text])
    Diamond,        // {text}
    Circle,         // ((text))
    Hexagon,        // {{text}}
}

/// A connection between nodes
#[derive(Debug, Clone)]
pub struct FlowConnection {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub arrow_type: ArrowStyle,
}

/// Arrow styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrowStyle {
    Arrow,      // -->
    Open,       // ---
    Dotted,     // -.->
    Thick,      // ==>
}

/// A subgraph grouping
#[derive(Debug, Clone)]
pub struct Subgraph {
    pub name: String,
    pub nodes: Vec<String>, // Node IDs in this subgraph
}

/// Parsed flowchart
#[derive(Debug, Clone)]
pub struct Flowchart {
    pub direction: FlowDirection,
    pub nodes: Vec<FlowNode>,
    pub connections: Vec<FlowConnection>,
    pub subgraphs: Vec<Subgraph>,
}

/// Result containing shapes and connectors
pub struct DiagramElements {
    pub shapes: Vec<Shape>,
    pub connectors: Vec<Connector>,
}

/// Detect the type of Mermaid diagram from code
pub fn detect_type(code: &str) -> MermaidType {
    let first_line = code.lines().next().unwrap_or("").trim().to_lowercase();
    
    if first_line.starts_with("graph") || first_line.starts_with("flowchart") {
        MermaidType::Flowchart
    } else if first_line.starts_with("sequencediagram") || first_line.starts_with("sequence") {
        MermaidType::Sequence
    } else if first_line.starts_with("pie") {
        MermaidType::Pie
    } else if first_line.starts_with("gantt") {
        MermaidType::Gantt
    } else if first_line.starts_with("classdiagram") || first_line.starts_with("class") {
        MermaidType::ClassDiagram
    } else if first_line.starts_with("statediagram") || first_line.starts_with("state") {
        MermaidType::StateDiagram
    } else if first_line.starts_with("erdiagram") || first_line.starts_with("er") {
        MermaidType::ErDiagram
    } else if first_line.starts_with("mindmap") {
        MermaidType::Mindmap
    } else if first_line.starts_with("timeline") {
        MermaidType::Timeline
    } else {
        MermaidType::Unknown
    }
}

/// Parse flowchart direction from first line
fn parse_direction(first_line: &str) -> FlowDirection {
    let line = first_line.to_uppercase();
    if line.contains("LR") {
        FlowDirection::LeftToRight
    } else if line.contains("RL") {
        FlowDirection::RightToLeft
    } else if line.contains("BT") {
        FlowDirection::BottomToTop
    } else {
        FlowDirection::TopToBottom
    }
}

/// Parse a flowchart from Mermaid code
pub fn parse_flowchart(code: &str) -> Flowchart {
    let mut lines = code.lines();
    let first_line = lines.next().unwrap_or("");
    let direction = parse_direction(first_line);
    
    let mut nodes: HashMap<String, FlowNode> = HashMap::new();
    let mut connections: Vec<FlowConnection> = Vec::new();
    let mut subgraphs: Vec<Subgraph> = Vec::new();
    let mut current_subgraph: Option<Subgraph> = None;
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        
        // Handle subgraph start
        if line.starts_with("subgraph") {
            let name = line.strip_prefix("subgraph").unwrap_or("").trim().to_string();
            current_subgraph = Some(Subgraph { name, nodes: Vec::new() });
            continue;
        }
        
        // Handle subgraph end
        if line == "end" {
            if let Some(sg) = current_subgraph.take() {
                subgraphs.push(sg);
            }
            continue;
        }
        
        // Parse connections: A --> B, A --> B[Label], A[Text] --> B[Text]
        if let Some((from_part, rest)) = split_connection(line) {
            let (arrow_type, to_part) = parse_arrow_and_rest(&rest);
            
            // Parse from node
            let (from_id, from_node) = parse_node_def(&from_part);
            if let Some(node) = from_node {
                nodes.entry(from_id.clone()).or_insert(node);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&from_id) {
                        sg.nodes.push(from_id.clone());
                    }
                }
            }
            
            // Parse to node (may have label on arrow)
            let (to_part_clean, arrow_label) = extract_arrow_label(&to_part);
            let (to_id, to_node) = parse_node_def(&to_part_clean);
            if let Some(node) = to_node {
                nodes.entry(to_id.clone()).or_insert(node);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&to_id) {
                        sg.nodes.push(to_id.clone());
                    }
                }
            }
            
            connections.push(FlowConnection {
                from: from_id,
                to: to_id,
                label: arrow_label,
                arrow_type,
            });
        } else {
            // Standalone node definition
            let (id, node) = parse_node_def(line);
            if let Some(n) = node {
                nodes.entry(id.clone()).or_insert(n);
                if let Some(ref mut sg) = current_subgraph {
                    if !sg.nodes.contains(&id) {
                        sg.nodes.push(id);
                    }
                }
            }
        }
    }
    
    Flowchart {
        direction,
        nodes: nodes.into_values().collect(),
        connections,
        subgraphs,
    }
}

/// Split line at connection arrow
fn split_connection(line: &str) -> Option<(String, String)> {
    for arrow in ["==>", "-.->", "-->", "---", "->"] {
        if let Some(pos) = line.find(arrow) {
            let from = line[..pos].trim().to_string();
            let rest = line[pos..].to_string();
            return Some((from, rest));
        }
    }
    None
}

/// Parse arrow type and get the rest of the string
fn parse_arrow_and_rest(s: &str) -> (ArrowStyle, String) {
    if s.starts_with("==>") {
        (ArrowStyle::Thick, s[3..].trim().to_string())
    } else if s.starts_with("-.->") {
        (ArrowStyle::Dotted, s[4..].trim().to_string())
    } else if s.starts_with("-->") {
        (ArrowStyle::Arrow, s[3..].trim().to_string())
    } else if s.starts_with("---") {
        (ArrowStyle::Open, s[3..].trim().to_string())
    } else if s.starts_with("->") {
        (ArrowStyle::Arrow, s[2..].trim().to_string())
    } else {
        (ArrowStyle::Arrow, s.to_string())
    }
}

/// Extract arrow label like |text|
fn extract_arrow_label(s: &str) -> (String, Option<String>) {
    if let Some(start) = s.find('|') {
        if let Some(end) = s[start+1..].find('|') {
            let label = s[start+1..start+1+end].to_string();
            let rest = s[start+2+end..].trim().to_string();
            return (rest, Some(label));
        }
    }
    (s.to_string(), None)
}

/// Parse a node definition like A[Text] or B(Text) or C{Text}
fn parse_node_def(s: &str) -> (String, Option<FlowNode>) {
    let s = s.trim();
    
    // Try different bracket types
    for (open, close, shape) in [
        ("((", "))", NodeShape::Circle),
        ("([", "])", NodeShape::Stadium),
        ("{{", "}}", NodeShape::Hexagon),
        ("[", "]", NodeShape::Rectangle),
        ("(", ")", NodeShape::RoundedRect),
        ("{", "}", NodeShape::Diamond),
    ] {
        if let Some(start) = s.find(open) {
            let id = s[..start].trim().to_string();
            if let Some(end) = s[start+open.len()..].find(close) {
                let label = s[start+open.len()..start+open.len()+end].to_string();
                return (id.clone(), Some(FlowNode { id, label, shape }));
            }
        }
    }
    
    // Plain node ID without brackets
    let id = s.to_string();
    if !id.is_empty() && id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return (id.clone(), Some(FlowNode { 
            id: id.clone(), 
            label: id, 
            shape: NodeShape::Rectangle 
        }));
    }
    
    (s.to_string(), None)
}

/// Generate shapes and connectors for a flowchart
pub fn generate_flowchart_elements(flowchart: &Flowchart) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();
    let node_count = flowchart.nodes.len();
    
    if node_count == 0 {
        return DiagramElements { shapes, connectors };
    }
    
    // Layout parameters (in EMUs) - improved spacing
    let node_width = 1_400_000u32;
    let node_height = 500_000u32;
    let h_spacing = 1_800_000u32;
    let v_spacing = 900_000u32;
    
    // Calculate grid layout based on subgraphs
    let is_horizontal = matches!(flowchart.direction, FlowDirection::LeftToRight | FlowDirection::RightToLeft);
    
    // Create node positions map with better layout
    let mut node_positions: HashMap<String, (u32, u32)> = HashMap::new();
    let mut node_shape_ids: HashMap<String, u32> = HashMap::new();
    let mut shape_id = 10u32;
    
    // If we have subgraphs, layout by subgraph
    if !flowchart.subgraphs.is_empty() {
        let mut subgraph_x = 500_000u32;
        let subgraph_start_y = 1_600_000u32;
        
        for (sg_idx, subgraph) in flowchart.subgraphs.iter().enumerate() {
            // Create subgraph background
            let sg_width = node_width + 400_000;
            let sg_height = (subgraph.nodes.len() as u32) * v_spacing + 400_000;
            let sg_x = subgraph_x;
            let sg_y = subgraph_start_y;
            
            // Subgraph background shape
            let sg_shape = Shape::new(ShapeType::RoundedRectangle, sg_x, sg_y, sg_width, sg_height)
                .with_fill(ShapeFill::new(get_subgraph_color(sg_idx)))
                .with_line(ShapeLine::new("757575", 1))
                .with_text(&subgraph.name);
            shapes.push(sg_shape);
            
            // Layout nodes within subgraph
            for (node_idx, node_id) in subgraph.nodes.iter().enumerate() {
                if let Some(node) = flowchart.nodes.iter().find(|n| &n.id == node_id) {
                    let x = sg_x + 200_000;
                    let y = sg_y + 300_000 + (node_idx as u32) * v_spacing;
                    
                    node_positions.insert(node.id.clone(), (x, y));
                    node_shape_ids.insert(node.id.clone(), shape_id);
                    
                    let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
                    shapes.push(shape);
                    shape_id += 1;
                }
            }
            
            subgraph_x += sg_width + 600_000;
        }
        
        // Layout any nodes not in subgraphs
        let mut orphan_y = subgraph_start_y;
        for node in &flowchart.nodes {
            if !node_positions.contains_key(&node.id) {
                let x = subgraph_x;
                let y = orphan_y;
                
                node_positions.insert(node.id.clone(), (x, y));
                node_shape_ids.insert(node.id.clone(), shape_id);
                
                let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
                shapes.push(shape);
                shape_id += 1;
                
                orphan_y += v_spacing;
            }
        }
    } else {
        // Simple grid layout without subgraphs
        let start_x = 1_000_000u32;
        let start_y = 1_800_000u32;
        let cols = if is_horizontal { node_count.min(5) } else { 1 };
        
        for (i, node) in flowchart.nodes.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            
            let (x, y) = if is_horizontal {
                (start_x + (col as u32) * h_spacing, start_y + (row as u32) * v_spacing)
            } else {
                (start_x + (col as u32) * h_spacing, start_y + (i as u32) * v_spacing)
            };
            
            node_positions.insert(node.id.clone(), (x, y));
            node_shape_ids.insert(node.id.clone(), shape_id);
            
            let shape = create_node_shape(node, x, y, node_width, node_height, shape_id);
            shapes.push(shape);
            shape_id += 1;
        }
    }
    
    // Create connectors for connections
    for conn in &flowchart.connections {
        if let (Some(&(from_x, from_y)), Some(&(to_x, to_y))) = 
            (node_positions.get(&conn.from), node_positions.get(&conn.to)) 
        {
            // Calculate connector endpoints
            let (start_x, start_y, end_x, end_y) = if is_horizontal {
                // Horizontal: connect right side to left side
                (from_x + node_width, from_y + node_height / 2,
                 to_x, to_y + node_height / 2)
            } else {
                // Vertical: connect bottom to top
                (from_x + node_width / 2, from_y + node_height,
                 to_x + node_width / 2, to_y)
            };
            
            // Choose connector type based on layout
            let connector_type = if (start_x as i32 - end_x as i32).abs() < 100_000 
                                 || (start_y as i32 - end_y as i32).abs() < 100_000 {
                ConnectorType::Straight
            } else {
                ConnectorType::Elbow
            };
            
            // Set line style based on arrow type
            let (line_color, line_dash) = match conn.arrow_type {
                ArrowStyle::Thick => ("E65100", LineDash::Solid),
                ArrowStyle::Dotted => ("757575", LineDash::Dash),
                ArrowStyle::Open => ("1565C0", LineDash::Solid),
                ArrowStyle::Arrow => ("1565C0", LineDash::Solid),
            };
            
            let mut connector = Connector::new(connector_type, start_x, start_y, end_x, end_y)
                .with_line(ConnectorLine::new(line_color, 19050).with_dash(line_dash))
                .with_end_arrow(ArrowType::Triangle);
            
            // Add label if present
            if let Some(label) = &conn.label {
                connector = connector.with_label(label);
            }
            
            connectors.push(connector);
        }
    }
    
    DiagramElements { shapes, connectors }
}

/// Get subgraph background color
fn get_subgraph_color(index: usize) -> &'static str {
    const COLORS: [&str; 6] = ["E3F2FD", "F3E5F5", "E8F5E9", "FFF3E0", "E0F7FA", "FCE4EC"];
    COLORS[index % COLORS.len()]
}

/// Create a node shape
fn create_node_shape(node: &FlowNode, x: u32, y: u32, width: u32, height: u32, _id: u32) -> Shape {
    let shape_type = match node.shape {
        NodeShape::Rectangle => ShapeType::Rectangle,
        NodeShape::RoundedRect => ShapeType::RoundedRectangle,
        NodeShape::Stadium => ShapeType::RoundedRectangle,
        NodeShape::Diamond => ShapeType::Diamond,
        NodeShape::Circle => ShapeType::Ellipse,
        NodeShape::Hexagon => ShapeType::Hexagon,
    };
    
    let fill_color = match node.shape {
        NodeShape::Diamond => "FFF3E0",
        NodeShape::Circle => "E3F2FD",
        _ => "FFFFFF",
    };
    
    Shape::new(shape_type, x, y, width, height)
        .with_fill(ShapeFill::new(fill_color))
        .with_line(ShapeLine::new("1565C0", 2))
        .with_text(&node.label)
}

/// Generate shapes for a flowchart (backward compatibility)
pub fn generate_flowchart_shapes(flowchart: &Flowchart) -> Vec<Shape> {
    let elements = generate_flowchart_elements(flowchart);
    elements.shapes
}

/// Parse pie chart data
pub fn parse_pie_chart(code: &str) -> Vec<(String, f64)> {
    let mut slices = Vec::new();
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.contains(':') && !line.starts_with("title") {
            if let Some((label, value)) = line.split_once(':') {
                let label = label.trim().trim_matches('"').to_string();
                if let Ok(val) = value.trim().parse::<f64>() {
                    slices.push((label, val));
                }
            }
        }
    }
    
    slices
}

/// Generate shapes for a pie chart
pub fn generate_pie_shapes(slices: &[(String, f64)]) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    if slices.is_empty() {
        return shapes;
    }
    
    let colors = ["4472C4", "ED7D31", "A5A5A5", "FFC000", "5B9BD5", "70AD47", "9E480E", "997300"];
    let center_x = 2_500_000u32;
    let center_y = 3_000_000u32;
    let radius = 1_500_000u32;
    
    // Create a circle for the pie
    let pie_circle = Shape::new(ShapeType::Ellipse, center_x - radius, center_y - radius, radius * 2, radius * 2)
        .with_fill(ShapeFill::new(colors[0]))
        .with_line(ShapeLine::new("FFFFFF", 2));
    shapes.push(pie_circle);
    
    // Create legend
    let legend_x = 5_000_000u32;
    let legend_y = 2_000_000u32;
    let legend_height = 350_000u32;
    
    let total: f64 = slices.iter().map(|(_, v)| v).sum();
    
    for (i, (label, value)) in slices.iter().enumerate() {
        let color = colors[i % colors.len()];
        let percentage = if total > 0.0 { value / total * 100.0 } else { 0.0 };
        
        // Color box
        let box_shape = Shape::new(ShapeType::Rectangle, legend_x, legend_y + (i as u32) * legend_height, 200_000, 200_000)
            .with_fill(ShapeFill::new(color));
        shapes.push(box_shape);
        
        // Label
        let label_text = format!("{} ({:.1}%)", label, percentage);
        let label_shape = Shape::new(ShapeType::Rectangle, legend_x + 300_000, legend_y + (i as u32) * legend_height, 2_500_000, 200_000)
            .with_text(&label_text);
        shapes.push(label_shape);
    }
    
    shapes
}

/// Create shapes and connectors for a Mermaid diagram (main entry point)
pub fn create_diagram_elements(code: &str) -> DiagramElements {
    let diagram_type = detect_type(code);
    
    match diagram_type {
        MermaidType::Flowchart => {
            let flowchart = parse_flowchart(code);
            generate_flowchart_elements(&flowchart)
        }
        MermaidType::Pie => {
            let slices = parse_pie_chart(code);
            DiagramElements {
                shapes: generate_pie_shapes(&slices),
                connectors: Vec::new(),
            }
        }
        MermaidType::Sequence => {
            DiagramElements {
                shapes: generate_sequence_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Gantt => {
            DiagramElements {
                shapes: generate_gantt_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::ClassDiagram => {
            generate_class_diagram_elements(code)
        }
        MermaidType::StateDiagram => {
            generate_state_diagram_elements(code)
        }
        MermaidType::ErDiagram => {
            generate_er_diagram_elements(code)
        }
        MermaidType::Mindmap => {
            DiagramElements {
                shapes: generate_mindmap_shapes(code),
                connectors: Vec::new(),
            }
        }
        MermaidType::Timeline => {
            DiagramElements {
                shapes: generate_timeline_shapes(code),
                connectors: Vec::new(),
            }
        }
        _ => {
            // Fallback: create a placeholder
            DiagramElements {
                shapes: vec![
                    Shape::new(ShapeType::Rectangle, 1_000_000, 2_000_000, 7_000_000, 3_000_000)
                        .with_fill(ShapeFill::new("F5F5F5"))
                        .with_line(ShapeLine::new("757575", 1))
                        .with_text(&format!("Diagram: {}", code.lines().next().unwrap_or("Unknown")))
                ],
                connectors: Vec::new(),
            }
        }
    }
}

/// Create shapes for a Mermaid diagram (backward compatibility)
pub fn create_diagram_shapes(code: &str) -> Vec<Shape> {
    create_diagram_elements(code).shapes
}

/// Generate shapes for a sequence diagram
fn generate_sequence_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut participant_ids: Vec<String> = Vec::new();
    let mut participant_names: HashMap<String, String> = HashMap::new(); // ID -> display name
    let mut messages: Vec<(String, String, String)> = Vec::new(); // (from_id, to_id, text)
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        
        // Parse participant
        if line.starts_with("participant") {
            let rest = line.strip_prefix("participant").unwrap_or("").trim();
            // Use alias if present, otherwise use the ID
            let (id, display_name) = if let Some((id, alias)) = rest.split_once(" as ") {
                (id.trim().to_string(), alias.trim().to_string())
            } else {
                let id = rest.split_whitespace().next().unwrap_or("").to_string();
                (id.clone(), id)
            };
            if !id.is_empty() && !participant_ids.contains(&id) {
                participant_ids.push(id.clone());
                participant_names.insert(id, display_name);
            }
        }
        // Parse message
        else if line.contains("->>") || line.contains("-->>") {
            let arrow = if line.contains("-->>") { "-->>" } else { "->>" };
            if let Some((from_part, rest)) = line.split_once(arrow) {
                if let Some((to_part, msg)) = rest.split_once(':') {
                    let from = from_part.trim().to_string();
                    let to = to_part.trim().to_string();
                    let text = msg.trim().to_string();
                    
                    // Auto-add participants if not defined
                    if !participant_ids.contains(&from) {
                        participant_ids.push(from.clone());
                        participant_names.insert(from.clone(), from.clone());
                    }
                    if !participant_ids.contains(&to) {
                        participant_ids.push(to.clone());
                        participant_names.insert(to.clone(), to.clone());
                    }
                    
                    messages.push((from, to, text));
                }
            }
        }
    }
    
    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let participant_width = 1_400_000u32;
    let participant_height = 400_000u32;
    let h_spacing = 1_800_000u32;
    let lifeline_height = 3_000_000u32;
    let message_spacing = 450_000u32;
    
    // Create participant boxes and lifelines
    let mut participant_x: HashMap<String, u32> = HashMap::new();
    
    for (i, id) in participant_ids.iter().enumerate() {
        let x = start_x + (i as u32) * h_spacing;
        participant_x.insert(id.clone(), x);
        
        let display_name = participant_names.get(id).unwrap_or(id);
        
        // Participant box at top
        let box_shape = Shape::new(ShapeType::Rectangle, x, start_y, participant_width, participant_height)
            .with_fill(ShapeFill::new("E3F2FD"))
            .with_line(ShapeLine::new("1565C0", 2))
            .with_text(display_name);
        shapes.push(box_shape);
        
        // Lifeline (dashed vertical line represented as thin rectangle)
        let lifeline_x = x + participant_width / 2 - 10_000;
        let lifeline_y = start_y + participant_height;
        let lifeline = Shape::new(ShapeType::Rectangle, lifeline_x, lifeline_y, 20_000, lifeline_height)
            .with_fill(ShapeFill::new("757575"));
        shapes.push(lifeline);
        
        // Participant box at bottom
        let bottom_box = Shape::new(ShapeType::Rectangle, x, start_y + participant_height + lifeline_height, participant_width, participant_height)
            .with_fill(ShapeFill::new("E3F2FD"))
            .with_line(ShapeLine::new("1565C0", 2))
            .with_text(display_name);
        shapes.push(bottom_box);
    }
    
    // Create message arrows
    let message_y_start = start_y + participant_height + 200_000;
    
    for (i, (from, to, text)) in messages.iter().enumerate() {
        if let (Some(&from_x), Some(&to_x)) = (participant_x.get(from), participant_x.get(to)) {
            let y = message_y_start + (i as u32) * message_spacing;
            let from_center = from_x + participant_width / 2;
            let to_center = to_x + participant_width / 2;
            
            // Arrow shape
            let (arrow_x, arrow_width, is_left) = if from_center < to_center {
                (from_center, to_center - from_center, false)
            } else {
                (to_center, from_center - to_center, true)
            };
            
            let arrow_type = if is_left { ShapeType::LeftArrow } else { ShapeType::RightArrow };
            let arrow = Shape::new(arrow_type, arrow_x, y, arrow_width, 120_000)
                .with_fill(ShapeFill::new("1565C0"));
            shapes.push(arrow);
            
            // Message text above arrow
            let text_shape = Shape::new(ShapeType::Rectangle, arrow_x, y.saturating_sub(180_000), arrow_width, 160_000)
                .with_text(text);
            shapes.push(text_shape);
        }
    }
    
    shapes
}

/// Generate shapes for a Gantt chart
fn generate_gantt_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut title = String::new();
    let mut sections: Vec<(String, Vec<(String, u32)>)> = Vec::new(); // (section_name, [(task_name, duration)])
    let mut current_section = String::new();
    let mut current_tasks: Vec<(String, u32)> = Vec::new();
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        
        if line.starts_with("title") {
            title = line.strip_prefix("title").unwrap_or("").trim().to_string();
        } else if line.starts_with("section") {
            // Save previous section
            if !current_section.is_empty() {
                sections.push((current_section.clone(), current_tasks.clone()));
                current_tasks.clear();
            }
            current_section = line.strip_prefix("section").unwrap_or("").trim().to_string();
        } else if line.contains(':') && !line.starts_with("dateFormat") && !line.starts_with("axisFormat") {
            // Parse task: "Task name : status, id, duration"
            if let Some((task_name, _rest)) = line.split_once(':') {
                let duration = 3u32; // Default duration
                current_tasks.push((task_name.trim().to_string(), duration));
            }
        }
    }
    
    // Save last section
    if !current_section.is_empty() {
        sections.push((current_section, current_tasks));
    }
    
    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let section_height = 300_000u32;
    let task_height = 250_000u32;
    let task_spacing = 280_000u32;
    let bar_width_per_unit = 600_000u32;
    let label_width = 2_000_000u32;
    
    // Title
    if !title.is_empty() {
        let title_shape = Shape::new(ShapeType::Rectangle, start_x, start_y, 7_000_000, 400_000)
            .with_text(&title);
        shapes.push(title_shape);
    }
    
    let mut y = start_y + 500_000;
    let colors = ["4472C4", "ED7D31", "70AD47", "FFC000", "5B9BD5"];
    
    for (section_idx, (section_name, tasks)) in sections.iter().enumerate() {
        // Section header
        let section_shape = Shape::new(ShapeType::Rectangle, start_x, y, 7_000_000, section_height)
            .with_fill(ShapeFill::new("E0E0E0"))
            .with_text(section_name);
        shapes.push(section_shape);
        y += section_height + 50_000;
        
        // Tasks
        for (task_idx, (task_name, duration)) in tasks.iter().enumerate() {
            // Task label
            let label_shape = Shape::new(ShapeType::Rectangle, start_x, y, label_width, task_height)
                .with_text(task_name);
            shapes.push(label_shape);
            
            // Task bar
            let bar_x = start_x + label_width + 100_000 + (task_idx as u32) * 200_000;
            let bar_width = duration * bar_width_per_unit;
            let color = colors[(section_idx + task_idx) % colors.len()];
            
            let bar_shape = Shape::new(ShapeType::RoundedRectangle, bar_x, y, bar_width, task_height)
                .with_fill(ShapeFill::new(color));
            shapes.push(bar_shape);
            
            y += task_spacing;
        }
    }
    
    shapes
}

/// Generate shapes and connectors for a class diagram
fn generate_class_diagram_elements(code: &str) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();
    
    // Parse classes
    let mut classes: Vec<(String, Vec<String>, Vec<String>)> = Vec::new(); // (name, attributes, methods)
    let mut current_class = String::new();
    let mut current_attrs: Vec<String> = Vec::new();
    let mut current_methods: Vec<String> = Vec::new();
    let mut in_class = false;
    let mut relationships: Vec<(String, String, String)> = Vec::new(); // (from, to, type)
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        
        if line.starts_with("class ") && line.contains('{') {
            // Start of class definition
            current_class = line.strip_prefix("class ").unwrap_or("")
                .split('{').next().unwrap_or("").trim().to_string();
            in_class = true;
            current_attrs.clear();
            current_methods.clear();
        } else if line == "}" && in_class {
            // End of class
            classes.push((current_class.clone(), current_attrs.clone(), current_methods.clone()));
            in_class = false;
        } else if in_class {
            // Parse member
            if line.contains('(') {
                current_methods.push(line.to_string());
            } else if !line.is_empty() {
                current_attrs.push(line.to_string());
            }
        } else if line.contains("<|--") || line.contains("-->") || line.contains("--") {
            // Parse relationship
            let rel_type = if line.contains("<|--") { "extends" }
                          else if line.contains("-->") { "uses" }
                          else { "associates" };
            
            let parts: Vec<&str> = line.split(|c| c == '<' || c == '|' || c == '-' || c == '>').collect();
            let parts: Vec<&str> = parts.into_iter().filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                relationships.push((parts[0].trim().to_string(), parts[parts.len()-1].trim().to_string(), rel_type.to_string()));
            }
        }
    }
    
    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let class_width = 2_000_000u32;
    let h_spacing = 2_500_000u32;
    let header_height = 350_000u32;
    let member_height = 250_000u32;
    
    let mut class_positions: HashMap<String, (u32, u32)> = HashMap::new();
    
    for (i, (class_name, attrs, methods)) in classes.iter().enumerate() {
        let x = start_x + (i as u32 % 3) * h_spacing;
        let y = start_y + (i as u32 / 3) * 2_000_000;
        class_positions.insert(class_name.clone(), (x, y));
        
        let total_height = header_height + (attrs.len() + methods.len()) as u32 * member_height + 100_000;
        
        // Class header
        let header = Shape::new(ShapeType::Rectangle, x, y, class_width, header_height)
            .with_fill(ShapeFill::new("4472C4"))
            .with_line(ShapeLine::new("2F5496", 2))
            .with_text(class_name);
        shapes.push(header);
        
        // Attributes section
        let attrs_text = if attrs.is_empty() { String::new() } else { attrs.join("\n") };
        let attrs_height = (attrs.len().max(1) as u32) * member_height;
        let attrs_shape = Shape::new(ShapeType::Rectangle, x, y + header_height, class_width, attrs_height)
            .with_fill(ShapeFill::new("D6DCE5"))
            .with_line(ShapeLine::new("2F5496", 1))
            .with_text(&attrs_text);
        shapes.push(attrs_shape);
        
        // Methods section
        let methods_text = if methods.is_empty() { String::new() } else { methods.join("\n") };
        let methods_height = (methods.len().max(1) as u32) * member_height;
        let methods_shape = Shape::new(ShapeType::Rectangle, x, y + header_height + attrs_height, class_width, methods_height)
            .with_fill(ShapeFill::new("FFFFFF"))
            .with_line(ShapeLine::new("2F5496", 1))
            .with_text(&methods_text);
        shapes.push(methods_shape);
    }
    
    // Create connectors for relationships
    for (from, to, _rel_type) in &relationships {
        if let (Some(&(from_x, from_y)), Some(&(to_x, to_y))) = 
            (class_positions.get(from), class_positions.get(to)) 
        {
            let connector = Connector::new(
                ConnectorType::Elbow,
                from_x + class_width / 2, from_y,
                to_x + class_width / 2, to_y + 500_000
            )
            .with_line(ConnectorLine::new("2F5496", 19050))
            .with_end_arrow(ArrowType::Triangle);
            connectors.push(connector);
        }
    }
    
    DiagramElements { shapes, connectors }
}

/// Generate shapes and connectors for a state diagram
fn generate_state_diagram_elements(code: &str) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();
    
    let mut states: Vec<String> = Vec::new();
    let mut transitions: Vec<(String, String, String)> = Vec::new(); // (from, to, label)
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") || line.starts_with("direction") {
            continue;
        }
        
        // Parse transitions: State1 --> State2 : label
        if line.contains("-->") {
            let parts: Vec<&str> = line.split("-->").collect();
            if parts.len() >= 2 {
                let from = parts[0].trim().to_string();
                let (to, label) = if let Some((t, l)) = parts[1].split_once(':') {
                    (t.trim().to_string(), l.trim().to_string())
                } else {
                    (parts[1].trim().to_string(), String::new())
                };
                
                // Handle [*] for start/end states
                let from_state = if from == "[*]" { "Start".to_string() } else { from };
                let to_state = if to == "[*]" { "End".to_string() } else { to };
                
                if !states.contains(&from_state) { states.push(from_state.clone()); }
                if !states.contains(&to_state) { states.push(to_state.clone()); }
                
                transitions.push((from_state, to_state, label));
            }
        }
    }
    
    // Layout parameters
    let start_x = 1_000_000u32;
    let start_y = 1_800_000u32;
    let state_width = 1_500_000u32;
    let state_height = 500_000u32;
    let h_spacing = 2_200_000u32;
    let v_spacing = 1_200_000u32;
    
    let mut state_positions: HashMap<String, (u32, u32)> = HashMap::new();
    
    for (i, state) in states.iter().enumerate() {
        let x = start_x + (i as u32 % 3) * h_spacing;
        let y = start_y + (i as u32 / 3) * v_spacing;
        state_positions.insert(state.clone(), (x, y));
        
        let shape_type = if state == "Start" || state == "End" {
            ShapeType::Ellipse
        } else {
            ShapeType::RoundedRectangle
        };
        
        let fill_color = if state == "Start" { "000000" }
                        else if state == "End" { "000000" }
                        else { "E0F7FA" };
        
        let shape = Shape::new(shape_type, x, y, state_width, state_height)
            .with_fill(ShapeFill::new(fill_color))
            .with_line(ShapeLine::new("00838F", 2))
            .with_text(state);
        shapes.push(shape);
    }
    
    // Create connectors
    for (from, to, label) in &transitions {
        if let (Some(&(from_x, from_y)), Some(&(to_x, to_y))) = 
            (state_positions.get(from), state_positions.get(to)) 
        {
            let mut connector = Connector::new(
                ConnectorType::Elbow,
                from_x + state_width, from_y + state_height / 2,
                to_x, to_y + state_height / 2
            )
            .with_line(ConnectorLine::new("00838F", 19050))
            .with_end_arrow(ArrowType::Triangle);
            
            if !label.is_empty() {
                connector = connector.with_label(label);
            }
            connectors.push(connector);
        }
    }
    
    DiagramElements { shapes, connectors }
}

/// Generate shapes and connectors for an ER diagram
fn generate_er_diagram_elements(code: &str) -> DiagramElements {
    let mut shapes = Vec::new();
    let mut connectors = Vec::new();
    
    let mut entities: HashMap<String, Vec<String>> = HashMap::new(); // entity -> attributes
    let mut relationships: Vec<(String, String, String)> = Vec::new(); // (entity1, entity2, cardinality)
    let mut current_entity = String::new();
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        
        // Parse relationship: ENTITY1 ||--o{ ENTITY2 : relationship
        if line.contains("||") || line.contains("}|") || line.contains("|{") || line.contains("o{") {
            let parts: Vec<&str> = line.split(|c: char| c == '|' || c == '{' || c == '}' || c == 'o' || c == '-').collect();
            let parts: Vec<&str> = parts.into_iter().filter(|s| !s.is_empty() && !s.contains(':') && s.chars().any(|c| c.is_alphabetic())).collect();
            if parts.len() >= 2 {
                let e1 = parts[0].trim().to_string();
                let e2 = parts[parts.len()-1].trim().to_string();
                if !entities.contains_key(&e1) { entities.insert(e1.clone(), Vec::new()); }
                if !entities.contains_key(&e2) { entities.insert(e2.clone(), Vec::new()); }
                relationships.push((e1, e2, "relates".to_string()));
            }
        }
        // Parse entity attributes
        else if line.contains('{') {
            current_entity = line.split('{').next().unwrap_or("").trim().to_string();
            if !entities.contains_key(&current_entity) {
                entities.insert(current_entity.clone(), Vec::new());
            }
        } else if line == "}" {
            current_entity.clear();
        } else if !current_entity.is_empty() && !line.is_empty() {
            if let Some(attrs) = entities.get_mut(&current_entity) {
                attrs.push(line.to_string());
            }
        }
    }
    
    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let entity_width = 2_200_000u32;
    let header_height = 400_000u32;
    let attr_height = 280_000u32;
    let h_spacing = 2_800_000u32;
    let v_spacing = 2_500_000u32;
    
    let mut entity_positions: HashMap<String, (u32, u32)> = HashMap::new();
    
    for (i, (entity_name, attrs)) in entities.iter().enumerate() {
        let x = start_x + (i as u32 % 3) * h_spacing;
        let y = start_y + (i as u32 / 3) * v_spacing;
        entity_positions.insert(entity_name.clone(), (x, y));
        
        // Entity header
        let header = Shape::new(ShapeType::Rectangle, x, y, entity_width, header_height)
            .with_fill(ShapeFill::new("C2185B"))
            .with_line(ShapeLine::new("880E4F", 2))
            .with_text(entity_name);
        shapes.push(header);
        
        // Attributes
        let attrs_text = attrs.join("\n");
        let attrs_box_height = (attrs.len().max(1) as u32) * attr_height;
        let attrs_shape = Shape::new(ShapeType::Rectangle, x, y + header_height, entity_width, attrs_box_height)
            .with_fill(ShapeFill::new("FCE4EC"))
            .with_line(ShapeLine::new("880E4F", 1))
            .with_text(&attrs_text);
        shapes.push(attrs_shape);
    }
    
    // Create connectors
    for (e1, e2, _) in &relationships {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = 
            (entity_positions.get(e1), entity_positions.get(e2)) 
        {
            let connector = Connector::new(
                ConnectorType::Elbow,
                x1 + entity_width, y1 + header_height / 2,
                x2, y2 + header_height / 2
            )
            .with_line(ConnectorLine::new("880E4F", 19050))
            .with_end_arrow(ArrowType::Diamond);
            connectors.push(connector);
        }
    }
    
    DiagramElements { shapes, connectors }
}

/// Generate shapes for a mindmap
fn generate_mindmap_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    let mut root = String::new();
    let mut level1: Vec<String> = Vec::new();
    let mut level2: Vec<(usize, String)> = Vec::new(); // (parent_index, text)
    
    for line in code.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        
        // Count leading spaces to determine level
        let spaces = line.len() - line.trim_start().len();
        let text = trimmed.trim_start_matches(|c| c == '-' || c == '+' || c == '*')
            .trim()
            .trim_matches(|c| c == '(' || c == ')' || c == '[' || c == ']')
            .to_string();
        
        if text.is_empty() { continue; }
        
        if spaces == 0 || (root.is_empty() && spaces <= 4) {
            if root.is_empty() {
                root = text;
            }
        } else if spaces <= 8 {
            level1.push(text);
        } else {
            let parent_idx = level1.len().saturating_sub(1);
            level2.push((parent_idx, text));
        }
    }
    
    // Layout parameters
    let center_x = 4_000_000u32;
    let center_y = 3_000_000u32;
    let root_width = 2_000_000u32;
    let root_height = 600_000u32;
    let node_width = 1_500_000u32;
    let node_height = 400_000u32;
    let radius1 = 2_000_000u32;
    let radius2 = 3_200_000u32;
    
    // Root node
    let root_shape = Shape::new(ShapeType::Ellipse, center_x - root_width/2, center_y - root_height/2, root_width, root_height)
        .with_fill(ShapeFill::new("3949AB"))
        .with_line(ShapeLine::new("1A237E", 2))
        .with_text(&root);
    shapes.push(root_shape);
    
    // Level 1 nodes (arranged in circle)
    let level1_colors = ["4472C4", "ED7D31", "70AD47", "FFC000", "5B9BD5", "9E480E"];
    let angle_step = if level1.is_empty() { 0.0 } else { 2.0 * std::f64::consts::PI / level1.len() as f64 };
    
    for (i, text) in level1.iter().enumerate() {
        let angle = (i as f64) * angle_step - std::f64::consts::PI / 2.0;
        let x = center_x + (radius1 as f64 * angle.cos()) as u32 - node_width / 2;
        let y = center_y + (radius1 as f64 * angle.sin()) as u32 - node_height / 2;
        
        let color = level1_colors[i % level1_colors.len()];
        let node = Shape::new(ShapeType::RoundedRectangle, x, y, node_width, node_height)
            .with_fill(ShapeFill::new(color))
            .with_text(text);
        shapes.push(node);
    }
    
    // Level 2 nodes
    for (parent_idx, text) in &level2 {
        if *parent_idx < level1.len() {
            let parent_angle = (*parent_idx as f64) * angle_step - std::f64::consts::PI / 2.0;
            let x = center_x + (radius2 as f64 * parent_angle.cos()) as u32 - node_width / 2;
            let y = center_y + (radius2 as f64 * parent_angle.sin()) as u32 - node_height / 2;
            
            let node = Shape::new(ShapeType::RoundedRectangle, x, y, node_width, node_height)
                .with_fill(ShapeFill::new("E8EAF6"))
                .with_line(ShapeLine::new("3949AB", 1))
                .with_text(text);
            shapes.push(node);
        }
    }
    
    shapes
}

/// Generate shapes for a timeline
fn generate_timeline_shapes(code: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    
    let mut title = String::new();
    let mut events: Vec<(String, Vec<String>)> = Vec::new(); // (date, [descriptions])
    let mut current_date = String::new();
    let mut current_items: Vec<String> = Vec::new();
    
    for line in code.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }
        
        if line.starts_with("title") {
            title = line.strip_prefix("title").unwrap_or("").trim().to_string();
        } else if line.contains(':') {
            // Save previous date
            if !current_date.is_empty() {
                events.push((current_date.clone(), current_items.clone()));
                current_items.clear();
            }
            let (date, item) = line.split_once(':').unwrap();
            current_date = date.trim().to_string();
            if !item.trim().is_empty() {
                current_items.push(item.trim().to_string());
            }
        } else if !current_date.is_empty() {
            current_items.push(line.to_string());
        }
    }
    
    // Save last date
    if !current_date.is_empty() {
        events.push((current_date, current_items));
    }
    
    // Layout parameters
    let start_x = 500_000u32;
    let start_y = 1_600_000u32;
    let timeline_y = 2_500_000u32;
    let event_width = 1_400_000u32;
    let event_spacing = 1_600_000u32;
    let date_height = 300_000u32;
    let item_height = 250_000u32;
    
    // Title
    if !title.is_empty() {
        let title_shape = Shape::new(ShapeType::Rectangle, start_x, start_y, 7_500_000, 400_000)
            .with_text(&title);
        shapes.push(title_shape);
    }
    
    // Timeline line
    let line_width = (events.len() as u32) * event_spacing + 500_000;
    let timeline_line = Shape::new(ShapeType::Rectangle, start_x, timeline_y, line_width, 30_000)
        .with_fill(ShapeFill::new("5D4037"));
    shapes.push(timeline_line);
    
    // Events
    let colors = ["EFEBE9", "D7CCC8", "BCAAA4", "A1887F"];
    
    for (i, (date, items)) in events.iter().enumerate() {
        let x = start_x + (i as u32) * event_spacing;
        let color = colors[i % colors.len()];
        
        // Date marker (circle on timeline)
        let marker = Shape::new(ShapeType::Ellipse, x + event_width/2 - 75_000, timeline_y - 60_000, 150_000, 150_000)
            .with_fill(ShapeFill::new("5D4037"));
        shapes.push(marker);
        
        // Date label
        let date_shape = Shape::new(ShapeType::Rectangle, x, timeline_y - date_height - 100_000, event_width, date_height)
            .with_fill(ShapeFill::new("5D4037"))
            .with_text(date);
        shapes.push(date_shape);
        
        // Event items (below timeline)
        let items_text = items.join("\n");
        let items_height = (items.len().max(1) as u32) * item_height;
        let items_shape = Shape::new(ShapeType::RoundedRectangle, x, timeline_y + 150_000, event_width, items_height)
            .with_fill(ShapeFill::new(color))
            .with_line(ShapeLine::new("5D4037", 1))
            .with_text(&items_text);
        shapes.push(items_shape);
    }
    
    shapes
}

/// Get diagram style info (for backward compatibility)
pub fn get_diagram_style(diagram_type: MermaidType) -> (&'static str, &'static str, &'static str, &'static str) {
    match diagram_type {
        MermaidType::Flowchart => ("E3F2FD", "1565C0", "Flowchart", ""),
        MermaidType::Sequence => ("F3E5F5", "7B1FA2", "Sequence Diagram", ""),
        MermaidType::Pie => ("FFF8E1", "FF8F00", "Pie Chart", ""),
        MermaidType::Gantt => ("E8F5E9", "2E7D32", "Gantt Chart", ""),
        MermaidType::ClassDiagram => ("FFF3E0", "E65100", "Class Diagram", ""),
        MermaidType::StateDiagram => ("E0F7FA", "00838F", "State Diagram", ""),
        MermaidType::ErDiagram => ("FCE4EC", "C2185B", "ER Diagram", ""),
        MermaidType::Mindmap => ("E8EAF6", "3949AB", "Mind Map", ""),
        MermaidType::Timeline => ("EFEBE9", "5D4037", "Timeline", ""),
        MermaidType::Unknown => ("F5F5F5", "757575", "Diagram", ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_flowchart() {
        assert_eq!(detect_type("flowchart LR"), MermaidType::Flowchart);
        assert_eq!(detect_type("graph TD"), MermaidType::Flowchart);
    }

    #[test]
    fn test_detect_pie() {
        assert_eq!(detect_type("pie"), MermaidType::Pie);
    }

    #[test]
    fn test_parse_flowchart_nodes() {
        let code = "flowchart LR\n    A[Start] --> B[Process] --> C[End]";
        let flowchart = parse_flowchart(code);
        assert_eq!(flowchart.direction, FlowDirection::LeftToRight);
        assert!(!flowchart.nodes.is_empty());
        assert!(!flowchart.connections.is_empty());
    }

    #[test]
    fn test_parse_node_shapes() {
        let (id, node) = parse_node_def("A[Rectangle]");
        assert_eq!(id, "A");
        assert!(node.is_some());
        assert_eq!(node.unwrap().shape, NodeShape::Rectangle);

        let (id, node) = parse_node_def("B(Rounded)");
        assert_eq!(id, "B");
        assert_eq!(node.unwrap().shape, NodeShape::RoundedRect);

        let (id, node) = parse_node_def("C{Diamond}");
        assert_eq!(id, "C");
        assert_eq!(node.unwrap().shape, NodeShape::Diamond);
    }

    #[test]
    fn test_generate_flowchart_shapes() {
        let code = "flowchart LR\n    A[Start] --> B[End]";
        let shapes = create_diagram_shapes(code);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_parse_pie_chart() {
        let code = "pie\n    \"Dogs\" : 30\n    \"Cats\" : 45";
        let slices = parse_pie_chart(code);
        assert_eq!(slices.len(), 2);
        assert_eq!(slices[0].0, "Dogs");
        assert_eq!(slices[0].1, 30.0);
    }

    #[test]
    fn test_generate_pie_shapes() {
        let code = "pie\n    \"A\" : 50\n    \"B\" : 50";
        let shapes = create_diagram_shapes(code);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_detect_sequence() {
        assert_eq!(detect_type("sequenceDiagram"), MermaidType::Sequence);
    }

    #[test]
    fn test_unknown_diagram() {
        assert_eq!(detect_type("unknown"), MermaidType::Unknown);
    }
}
