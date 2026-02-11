// SPDX-License-Identifier: MIT

//! Image processing utilities for multi-platform posting

use anyhow::{Result, anyhow};
use image::{ImageFormat, GenericImageView};
use std::io::Cursor;

/// Strip EXIF data from image bytes
pub fn strip_exif(image_bytes: &[u8]) -> Result<Vec<u8>> {
    // Load the image
    let img = image::load_from_memory(image_bytes)?;
    
    // Re-encode without metadata
    let mut output = Vec::new();
    let format = guess_format(image_bytes)?;
    
    let mut cursor = Cursor::new(&mut output);
    img.write_to(&mut cursor, format)?;
    
    Ok(output)
}

/// Compress/resize image to meet size requirements
/// Returns compressed image bytes
pub fn compress_image(image_bytes: &[u8], max_size_bytes: usize, max_dimension: Option<u32>) -> Result<Vec<u8>> {
    let img = image::load_from_memory(image_bytes)?;
    let format = guess_format(image_bytes)?;
    
    // Resize if needed
    let img = if let Some(max_dim) = max_dimension {
        let (width, height) = img.dimensions();
        if width > max_dim || height > max_dim {
            img.resize(max_dim, max_dim, image::imageops::FilterType::Lanczos3)
        } else {
            img
        }
    } else {
        img
    };
    
    // Try different quality levels to meet size requirement
    let mut quality = 95;
    let mut output = Vec::new();
    
    loop {
        output.clear();
        let mut cursor = Cursor::new(&mut output);
        
        match format {
            ImageFormat::Jpeg => {
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
                encoder.encode_image(&img)?;
            }
            ImageFormat::Png => {
                // PNG doesn't have quality setting, so we just encode
                img.write_to(&mut cursor, format)?;
                break; // PNG is lossless, can't compress further with quality
            }
            ImageFormat::WebP => {
                img.write_to(&mut cursor, format)?;
                break;
            }
            _ => {
                img.write_to(&mut cursor, format)?;
                break;
            }
        }
        
        if output.len() <= max_size_bytes || quality <= 50 {
            break;
        }
        
        quality -= 5;
    }
    
    // If still too large, try downscaling
    if output.len() > max_size_bytes {
        let scale_factor = ((max_size_bytes as f64) / (output.len() as f64)).sqrt();
        let new_width = ((img.width() as f64) * scale_factor) as u32;
        let new_height = ((img.height() as f64) * scale_factor) as u32;
        
        let resized = img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
        
        output.clear();
        let mut cursor = Cursor::new(&mut output);
        
        if format == ImageFormat::Jpeg {
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, 85);
            encoder.encode_image(&resized)?;
        } else {
            resized.write_to(&mut cursor, format)?;
        }
    }
    
    Ok(output)
}

/// Guess image format from bytes
fn guess_format(bytes: &[u8]) -> Result<ImageFormat> {
    image::guess_format(bytes).map_err(|e| anyhow!("Failed to guess image format: {}", e))
}

/// Get MIME type from image bytes
pub fn get_mime_type(bytes: &[u8]) -> Result<String> {
    let format = guess_format(bytes)?;
    
    Ok(match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        _ => "application/octet-stream",
    }.to_string())
}

/// Process image for platform requirements
pub struct ImageProcessor {
    max_size: Option<usize>,
    max_dimension: Option<u32>,
    strip_exif: bool,
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            max_size: None,
            max_dimension: None,
            strip_exif: false,
        }
    }
    
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }
    
    pub fn with_max_dimension(mut self, dim: u32) -> Self {
        self.max_dimension = Some(dim);
        self
    }
    
    pub fn with_exif_stripping(mut self) -> Self {
        self.strip_exif = true;
        self
    }
    
    pub fn process(&self, image_bytes: &[u8]) -> Result<Vec<u8>> {
        let mut bytes = image_bytes.to_vec();
        
        // Strip EXIF first if requested
        if self.strip_exif {
            bytes = strip_exif(&bytes)?;
        }
        
        // Compress if size limit specified
        if let Some(max_size) = self.max_size {
            if bytes.len() > max_size {
                bytes = compress_image(&bytes, max_size, self.max_dimension)?;
            }
        }
        
        Ok(bytes)
    }
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new()
    }
}
