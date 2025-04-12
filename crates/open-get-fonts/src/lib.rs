use neon::prelude::*;
use neon::types::JsPromise;
use std::sync::{Arc, Mutex};
use font_kit::source::SystemSource;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::handle::Handle;
use log::debug;

// Platform-specific font detection implemented in separate modules
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

// This function will return a promise that resolves to an array of font information
fn get_fonts(mut cx: FunctionContext) -> JsResult<JsPromise> {
    // Create a promise with associated deferred object to resolve later
    let (deferred, promise) = cx.promise();
    
    // We need to spawn a new thread to do the font detection work
    // so we don't block the Node.js event loop
    let channel = cx.channel();
    
    // Safety: We're capturing the deferred object to resolve the promise later
    let deferred = Arc::new(Mutex::new(Some(deferred)));
    
    // Spawn a new thread to do the font detection work
    std::thread::spawn(move || {
        // Use font-kit to get system fonts
        let fonts = get_system_fonts();
        
        // Send the fonts back to the main thread to resolve the promise
        let deferred = Arc::clone(&deferred);
        channel.send(move |mut cx| {
            let mut deferred_guard = deferred.lock().unwrap();
            let deferred = deferred_guard.take().unwrap();
            
            // Create a JavaScript array to hold the font information
            let js_array = cx.empty_array();
            
            for (i, font) in fonts.iter().enumerate() {
                let js_obj = cx.empty_object();
                let js_name = cx.string(&font.name);
                let js_path = cx.string(&font.path);
                
                js_obj.set(&mut cx, "name", js_name)?;
                js_obj.set(&mut cx, "path", js_path)?;
                
                js_array.set(&mut cx, i as u32, js_obj)?;
            }
            
            // Resolve the promise with the array
            deferred.resolve(&mut cx, js_array);
            
            Ok(())
        });
    });
    
    // Return the promise
    Ok(promise)
}

// Font information structure
#[derive(Debug, Clone)]
pub struct FontInfo {
    pub name: String,
    pub path: String,
}

// Use font-kit to get system fonts
fn get_system_fonts() -> Vec<FontInfo> {
    let mut fonts = Vec::new();
    let source = SystemSource::new();
    
    // Function to process a font handle
    let mut process_handle = |handle: Handle, family_name: &str| {
        match handle {
            Handle::Path { path, .. } => {
                let path_str = path.to_string_lossy().to_string();
                fonts.push(FontInfo {
                    name: family_name.to_string(),
                    path: path_str,
                });
            },
            Handle::Memory { .. } => {
                // Memory fonts don't have a path, but we still report them with an empty path
                fonts.push(FontInfo {
                    name: family_name.to_string(),
                    path: String::new(),
                });
            },
        }
    };
    
    // Get all font families available on the system
    if let Ok(font_families) = source.all_families() {
        for family_name in font_families {
            // Try to get a specific font from this family with default properties
            let default_properties = Properties::new();
            
            if let Ok(handle) = source.select_best_match(&[FamilyName::Title(family_name.clone())], &default_properties) {
                process_handle(handle, &family_name);
            }
        }
    } else {
        debug!("Failed to get font families from system");
        // Try alternate methods on failure...
        
        // For Linux with fontconfig
        #[cfg(target_os = "linux")]
        {
            if let Some(linux_fonts) = linux::get_fontconfig_fonts() {
                fonts.extend(linux_fonts);
            }
        }
        
        // For macOS with Core Text
        #[cfg(target_os = "macos")]
        {
            if let Some(macos_fonts) = macos::get_core_text_fonts() {
                fonts.extend(macos_fonts);
            }
        }
        
        // For Windows with DirectWrite
        #[cfg(target_os = "windows")]
        {
            if let Some(windows_fonts) = windows::get_directwrite_fonts() {
                fonts.extend(windows_fonts);
            }
        }
    }
    
    fonts
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getFonts", get_fonts)?;
    Ok(())
}

// Additional platform-specific implementations
// Removed duplicate platform-specific modules as they are defined at the top of the file
