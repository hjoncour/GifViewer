use std::collections::HashMap;
use std::fmt;

pub struct Multimedia {
    pub name: String,                         // The name of the picture or animation.
    pub description: String,                  // A brief description or caption.
    pub author: String,                       // The name of the author or creator.
    pub format: String,                       // File format (e.g., JPEG, PNG, GIF, MP4).
    pub dimensions: (u32, u32),             // Width and height in pixels.
    pub size_bytes: u64,                    // File size in bytes.
    pub metadata: HashMap<String, String>,  // Additional metadata as key-value pairs.
    pub content: Vec<u8>,                   // The binary content of the picture or animation.
}

impl fmt::Display for Multimedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Multimedia Details:")?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "Dimensions: {:?}", self.dimensions)?;
        writeln!(f, "Size (Bytes): {}", self.size_bytes)?;
        writeln!(f, "Metadata: {:?}", self.metadata)?;
        writeln!(f, "Content Length: {} bytes", self.content.len())?;
        Ok(())
    }
}
