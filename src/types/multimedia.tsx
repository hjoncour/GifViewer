class Multimedia {
    name:           string;                     // The name of the file
    description:    string;                     // A brief description or caption.
    author:         string;                     // The name of the author or creator.
    format:         string;                     // File format (e.g., JPEG, PNG, GIF, MP4).
    fileType:       string;                     // Type of file: animated picture, static picture, other
    content:        string;                     // The base64 content of the media
    sizeBytes:      number;                     // File size in bytes.
    dimensions:     [number, number];           // Width and height in pixels.
    metadata:       Record<string, string>;     // Additional metadata as key-value pairs.
  
    constructor(
      name:                 string,
      description:          string,
      author:               string,
      format:               string,
      fileType:             string,
      dimensions:           [number, number],
      sizeBytes:            number,
      metadata:             Record<string, string>,
      content:              string
    ) {
      this.name        =    name;
      this.description =    description;
      this.author      =    author;
      this.format      =    format;
      this.fileType    =    fileType;
      this.dimensions  =    dimensions;
      this.sizeBytes   =    sizeBytes;
      this.metadata    =    metadata;
      this.content     =    content;
    }
  
    toString(): string {
      return `Multimedia Details:
  Name: ${this.name}
  Description: ${this.description}
  Author: ${this.author}
  Format: ${this.format}
  Dimensions: [${this.dimensions[0]}, ${this.dimensions[1]}]
  Size (Bytes): ${this.sizeBytes}
  Metadata: ${JSON.stringify(this.metadata)}
  Content Length: ${this.content.length} bytes`;
    }
  }
  