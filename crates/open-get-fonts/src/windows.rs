use super::FontInfo;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteFontCollection, IDWriteFontFamily,
    IDWriteLocalizedStrings, DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
};
use widestring::U16CString;

pub fn get_directwrite_fonts() -> Option<Vec<FontInfo>> {
    unsafe {
        let mut factory: *mut IDWriteFactory = ptr::null_mut();
        let hr = DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &IDWriteFactory::uuidof(),
            &mut factory as *mut _ as *mut _,
        );
        
        if !SUCCEEDED(hr) {
            return None;
        }
        
        let mut font_collection: *mut IDWriteFontCollection = ptr::null_mut();
        let hr = (*factory).GetSystemFontCollection(&mut font_collection, FALSE);
        
        if !SUCCEEDED(hr) {
            return None;
        }
        
        let mut fonts = Vec::new();
        let family_count = (*font_collection).GetFontFamilyCount();
        
        for i in 0..family_count {
            let mut family: *mut IDWriteFontFamily = ptr::null_mut();
            let hr = (*font_collection).GetFontFamily(i, &mut family);
            
            if !SUCCEEDED(hr) {
                continue;
            }
            
            let mut names: *mut IDWriteLocalizedStrings = ptr::null_mut();
            let hr = (*family).GetFamilyNames(&mut names);
            
            if !SUCCEEDED(hr) {
                continue;
            }
            
            // Get family name
            let mut name_length: u32 = 0;
            let hr = (*names).GetStringLength(0, &mut name_length);
            
            if !SUCCEEDED(hr) {
                continue;
            }
            
            // +1 for null terminator
            let mut name_buffer: Vec<u16> = vec![0; (name_length + 1) as usize];
            let hr = (*names).GetString(0, name_buffer.as_mut_ptr(), name_length + 1);
            
            if !SUCCEEDED(hr) {
                continue;
            }
            
            // Convert to Rust string
            let name = U16CString::from_vec_with_nul(name_buffer)
                .ok()
                .and_then(|s| s.to_string().ok())
                .unwrap_or_default();
            
            // Get font path (in Windows, this is more complex and generally requires 
            // looking up in the registry or enumerating font files directly)
            // For simplicity, we'll just use a placeholder path pattern
            let path = format!("C:\\Windows\\Fonts\\{}.ttf", name);
            
            fonts.push(FontInfo {
                name,
                path,
            });
        }
        
        Some(fonts)
    }
} 