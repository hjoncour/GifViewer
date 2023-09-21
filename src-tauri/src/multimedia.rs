use std::collections::HashMap;
use std::fmt;

pub struct Multimedia {
    pub title: String,               // The title or name of the picture or animation.
    pub description: String,         // A brief description or caption.
    pub author: String,              // The name of the author or creator.
    pub created_at: String,          // Date or timestamp when it was created.
    pub format: String,              // File format (e.g., JPEG, PNG, GIF, MP4).
    pub dimensions: (u32, u32),      // Width and height in pixels.
    pub duration: Option<u32>,       // Duration in seconds (for animations, if applicable).
    pub size_bytes: u64,             // File size in bytes.
    pub metadata: HashMap<String, String>, // Additional metadata as key-value pairs.
    pub content: Vec<u8>,            // The binary content of the picture or animation.
}

impl fmt::Display for Multimedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Multimedia Details:")?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Created At: {}", self.created_at)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "Dimensions: {:?}", self.dimensions)?;
        if let Some(duration) = self.duration {
            writeln!(f, "Duration: {} seconds", duration)?;
        } else {
            writeln!(f, "Duration: N/A")?;
        }
        writeln!(f, "Size (Bytes): {}", self.size_bytes)?;
        writeln!(f, "Metadata: {:?}", self.metadata)?;
        writeln!(f, "Content Length: {} bytes", self.content.len())?;
        Ok(())
    }
}
