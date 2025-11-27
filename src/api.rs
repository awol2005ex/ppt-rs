//! Public API module
//!
//! High-level API for working with PowerPoint presentations.

use crate::exc::{Result, PptxError};
use crate::opc::Package;
use crate::generator::{SlideContent, create_pptx_with_content};
use std::io::{Read, Seek};
use std::path::Path;

/// Represents a PowerPoint presentation
#[derive(Debug, Clone, Default)]
pub struct Presentation {
    title: String,
    slides: Vec<SlideContent>,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Self {
        Presentation {
            title: String::new(),
            slides: Vec::new(),
        }
    }

    /// Create a presentation with a title
    pub fn with_title(title: &str) -> Self {
        Presentation {
            title: title.to_string(),
            slides: Vec::new(),
        }
    }

    /// Set the presentation title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Add a slide to the presentation
    pub fn add_slide(mut self, slide: SlideContent) -> Self {
        self.slides.push(slide);
        self
    }

    /// Get the number of slides
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get the presentation title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Build the presentation as PPTX bytes
    pub fn build(&self) -> Result<Vec<u8>> {
        if self.slides.is_empty() {
            return Err(PptxError::InvalidState("Presentation has no slides".into()));
        }
        create_pptx_with_content(&self.title, self.slides.clone())
            .map_err(|e| PptxError::Generic(e.to_string()))
    }

    /// Save the presentation to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = self.build()?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

/// Open a presentation from a file path
pub fn open<P: AsRef<Path>>(path: P) -> Result<Package> {
    Package::open(path)
}

/// Open a presentation from a reader
pub fn open_reader<R: Read + Seek>(reader: R) -> Result<Package> {
    Package::open_reader(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_builder() {
        let pres = Presentation::with_title("Test")
            .add_slide(SlideContent::new("Slide 1").add_bullet("Point 1"));
        
        assert_eq!(pres.get_title(), "Test");
        assert_eq!(pres.slide_count(), 1);
    }

    #[test]
    fn test_presentation_build() {
        let pres = Presentation::with_title("Test")
            .add_slide(SlideContent::new("Slide 1"));
        
        let result = pres.build();
        assert!(result.is_ok());
    }
}
