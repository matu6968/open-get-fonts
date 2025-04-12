use super::FontInfo;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURL, CFURLRef};
use core_text::font::CTFont;
use core_text::font_collection::CTFontCollection;
use core_text::font_descriptor::{CTFontDescriptor, CTFontDescriptorRef};
use std::path::PathBuf;

pub fn get_core_text_fonts() -> Option<Vec<FontInfo>> {
    let collection = CTFontCollection::create_for_all_families();
    let descriptors = collection.get_descriptors()?;
    let mut fonts = Vec::new();
    
    for i in 0..descriptors.len() {
        let descriptor = descriptors.get(i);
        let font = CTFont::new_from_descriptor(&descriptor, 0.0);
        
        // Get font name
        let name = font.family_name();
        
        // Get font path
        let url_key = unsafe { CFString::wrap_under_get_rule(CTFontDescriptor::get_url_attribute_key()) };
        
        if let Some(url) = descriptor.get_url(&url_key) {
            if let Some(path) = url.to_path() {
                fonts.push(FontInfo {
                    name: name.to_string(),
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }
    
    Some(fonts)
} 