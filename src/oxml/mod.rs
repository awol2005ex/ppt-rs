//! OXML (Office XML) element handling
//!
//! Provides XML parsing and content extraction for Office Open XML documents.

pub mod action;
pub mod chart;
pub mod coreprops;
pub mod dml;
pub mod editor;
pub mod ns;
pub mod presentation;
pub mod shapes;
pub mod simpletypes;
pub mod slide;
pub mod table;
pub mod text;
pub mod theme;
pub mod xmlchemy;

// Core XML parsing
pub use xmlchemy::{XmlElement, XmlParser, BaseOxmlElement};

// Slide parsing
pub use slide::{SlideParser, ParsedSlide, ParsedShape, ParsedTable, ParsedTableCell, Paragraph, TextRun};

// Presentation reading
pub use presentation::{PresentationReader, PresentationInfo};

// Presentation editing
pub use editor::PresentationEditor;

// Namespace utilities
pub use ns::Namespace;
