use super::FontInfo;
use std::path::PathBuf;
use std::ffi::CString;
use fontconfig::{Fontconfig, Pattern, ObjectSet};

pub fn get_fontconfig_fonts() -> Option<Vec<FontInfo>> {
    // Initialize fontconfig
    let fc = Fontconfig::new().or(None)?;
    let mut fonts = Vec::new();
    
    // Create a pattern to match all fonts
    let pattern = Pattern::new(&fc);
    
    // Create an object set for the properties we want to extract
    let mut object_set = ObjectSet::new(&fc);
    
    // Add the family and file properties to retrieve
    // Convert the constants to CString objects that can be passed to add()
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).ok()?;
    let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).ok()?;
    
    object_set.add(&family_cstr);
    object_set.add(&file_cstr);
    
    // Use the global list_fonts function from fontconfig
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    // Iterate through patterns in the font set
    for pattern in font_set.iter() {
        // Try to get the family name
        if let Some(family) = pattern.get_string(&family_cstr) {
            // Try to get the file path
            if let Some(path) = pattern.get_string(&file_cstr) {
                fonts.push(FontInfo {
                    name: family.to_string(),
                    path: path.to_string(),
                });
            }
        }
    }
    
    // If we didn't find any fonts, return some common ones that are likely to exist
    if fonts.is_empty() {
        for path in &[
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/TTF/Arial.ttf",
        ] {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                fonts.push(FontInfo {
                    name: path_buf.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                    path: path.to_string(),
                });
            }
        }
    }
    
    Some(fonts)
} 