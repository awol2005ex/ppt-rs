//! Package parts module
//!
//! Provides abstraction for PPTX package parts (files within the ZIP).
//! Each part type handles its own XML generation and parsing.
//!
//! # Part Types
//!
//! - **PresentationPart** - Main presentation.xml
//! - **SlidePart** - Individual slides (ppt/slides/slideN.xml)
//! - **SlideLayoutPart** - Slide layouts (ppt/slideLayouts/slideLayoutN.xml)
//! - **SlideMasterPart** - Slide masters (ppt/slideMasters/slideMasterN.xml)
//! - **ThemePart** - Themes (ppt/theme/themeN.xml)
//! - **NotesSlidePart** - Speaker notes (ppt/notesSlides/notesSlideN.xml)
//! - **ImagePart** - Embedded images (ppt/media/imageN.ext)
//! - **MediaPart** - Embedded media (ppt/media/mediaN.ext)
//! - **ChartPart** - Charts (ppt/charts/chartN.xml)
//! - **TablePart** - Tables (embedded in slides)
//! - **CorePropertiesPart** - Core metadata (docProps/core.xml)
//! - **AppPropertiesPart** - App metadata (docProps/app.xml)
//! - **ContentTypesPart** - Content types ([Content_Types].xml)
//! - **Relationships** - Relationship management

pub mod base;
pub mod presentation;
pub mod slide;
pub mod slide_layout;
pub mod slide_master;
pub mod theme;
pub mod notes_slide;
pub mod image;
pub mod media;
pub mod chart;
pub mod table;
pub mod coreprops;
pub mod app_props;
pub mod content_types;
pub mod relationships;

// Re-export main types
pub use base::{Part, PartType, ContentType};
pub use presentation::PresentationPart;
pub use slide::SlidePart;
pub use slide_layout::{SlideLayoutPart, LayoutType};
pub use slide_master::SlideMasterPart;
pub use theme::{ThemePart, ThemeColor, ThemeFont};
pub use notes_slide::NotesSlidePart;
pub use image::ImagePart;
pub use media::{MediaPart, MediaFormat};
pub use chart::ChartPart;
pub use table::{TablePart, TableRowPart, TableCellPart};
pub use coreprops::CorePropertiesPart;
pub use app_props::AppPropertiesPart;
pub use content_types::{ContentTypesPart, DefaultType, OverrideType};
pub use relationships::{Relationship, RelationshipType, Relationships};
