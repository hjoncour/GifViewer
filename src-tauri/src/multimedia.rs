use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Multimedia {
    pub name: String,                                           // The name of the file
    pub local_index: usize,                                     // Local index
    pub description: String,                                    // A brief description or caption.
    pub author: String,                                         // The name of the author or creator.
    pub format: String,                                         // File format (e.g., JPEG, PNG, GIF, MP4).
    pub file_type: String,                                      // Type of file: animated picture, static picture, other
    pub dimensions: (u32, u32),                                 // Width and height in pixels.
    pub size_bytes: u64,                                        // File size in bytes.
    pub metadata: HashMap<String, String>,                      // Additional metadata as key-value pairs.
    pub content: String,                                        // The base64 content of the media
}

impl fmt::Display for Multimedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Multimedia Details:")?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Local index: {}", self.local_index)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "Dimensions: {:?}", self.dimensions)?;
        writeln!(f, "Size (Bytes): {}", self.size_bytes)?;
        writeln!(f, "Metadata: {:?}", self.metadata)?;
        writeln!(f, "Content: {}", self.content)?;
        Ok(())
    }
}
