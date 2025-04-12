# Open-Get-Fonts API Usage Documentation

This document provides a comprehensive guide on how to use the open-get-fonts library and its underlying platform-specific APIs.

## Table of Contents
- [Overview](#overview)
- [JavaScript API](#javascript-api)
- [Rust API](#rust-api)
- [Platform-Specific Font Detection](#platform-specific-font-detection)
  - [Linux (fontconfig)](#linux-fontconfig)
  - [macOS (Core Text)](#macos-core-text)
  - [Windows (DirectWrite)](#windows-directwrite)

## Overview

The open-get-fonts library provides a cross-platform API for detecting system fonts. It works on:
- Linux (via fontconfig)
- macOS (via Core Text)
- Windows (via DirectWrite)

The library is implemented in Rust and exposed to JavaScript through Neon bindings, making it usable in Node.js applications.

## JavaScript API

The library exports a single function called `getFonts()` which returns a Promise that resolves to an array of font information objects:

```javascript
const openGetFonts = require('open-get-fonts');

// Get all system fonts
openGetFonts.getFonts()
  .then(fonts => {
    fonts.forEach(font => {
      console.log(`Font: ${font.name}, Path: ${font.path}`);
    });
  })
  .catch(err => {
    console.error('Error retrieving fonts:', err);
  });
```

Each font object has the following structure:
```javascript
{
  name: string, // The font family name
  path: string  // The path to the font file on disk
}
```

## Rust API

If you're using this library as a Rust dependency, you can use its API directly:

```rust
use open_get_fonts::{FontInfo, get_system_fonts};

fn main() {
    // Get all system fonts
    let fonts: Vec<FontInfo> = get_system_fonts();
    
    for font in fonts {
        println!("Font: {}, Path: {}", font.name, font.path);
    }
}
```

The `FontInfo` struct is defined as:
```rust
pub struct FontInfo {
    pub name: String,
    pub path: String,
}
```

## Platform-Specific Font Detection

### Linux (fontconfig)

On Linux, the library uses the fontconfig library to detect system fonts. The main implementation is in the `linux.rs` file.

#### Using fontconfig directly

If you want to use the fontconfig API directly for more advanced use cases, here's how to do it:

```rust
use fontconfig::{Fontconfig, Pattern, ObjectSet};
use std::ffi::CString;

fn get_fonts() {
    // Initialize fontconfig
    let fc = Fontconfig::new().expect("Failed to initialize fontconfig");
    
    // Create a pattern to match all fonts
    let pattern = Pattern::new(&fc);
    
    // Create an object set for the properties you want to extract
    let mut object_set = ObjectSet::new(&fc);
    
    // Convert fontconfig constants to CString for API calls
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
    let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).unwrap();
    
    // Add properties to the object set
    object_set.add(&family_cstr);
    object_set.add(&file_cstr);
    
    // List fonts matching the pattern
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    // Iterate through the font set
    for pattern in font_set.iter() {
        // Extract properties
        if let Some(family) = pattern.get_string(&family_cstr) {
            if let Some(path) = pattern.get_string(&file_cstr) {
                println!("Font: {}, Path: {}", family, path);
            }
        }
    }
}
```

#### Additional fontconfig features

Fontconfig provides many more features that you can use:

1. **Font matching with specific properties**:
```rust
// Create a pattern with specific properties
let mut pattern = Pattern::new(&fc);
let weight_cstr = CString::new(fontconfig::FC_WEIGHT.to_bytes()).unwrap();
let weight_value = fontconfig::FC_WEIGHT_BOLD; // Use a bold font

// Set the weight property in the pattern
pattern.add_integer(&weight_cstr, weight_value as i32);

// Now match fonts with this pattern
let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
```

2. **Font sorting**:
```rust
// Sort fonts by preference
let sorted_set = fontconfig::sort_fonts(&pattern, true);
```

3. **Font config manipulation**:
```rust
// Get config paths
let config_home = fontconfig::get_config_home();
let config_dirs = fontconfig::get_config_dirs();
```

4. **Advanced pattern matching**:
```rust
// Match a font with specific criteria
let result = fontconfig::match_pattern(&pattern, true);
if let Some(matched_pattern) = result {
    // Use the matched pattern
}
```

### macOS (Core Text)

On macOS, the library uses Core Text to detect system fonts.

#### Core Text API Overview

The macOS module uses the Core Text framework through FFI bindings. Here's a simplified overview of how to use it:

```rust
// This is a conceptual example, actual implementation details may vary
fn get_macos_fonts() {
    // Get all font descriptors
    let collection = CTFontCollectionCreateFromAvailableFonts(None);
    let descriptors = CTFontCollectionCreateMatchingFontDescriptors(collection);
    
    // Iterate through font descriptors
    for descriptor in descriptors {
        // Get font name
        let name = CTFontDescriptorCopyAttribute(descriptor, kCTFontNameAttribute);
        
        // Get font file URL
        let url = CTFontDescriptorCopyAttribute(descriptor, kCTFontURLAttribute);
        let path = convert_url_to_path(url);
        
        println!("Font: {}, Path: {}", name, path);
    }
}
```

### Windows (DirectWrite)

On Windows, the library uses DirectWrite API to detect system fonts.

#### DirectWrite API Overview

The Windows module uses the DirectWrite API through the windows-rs bindings. Here's a simplified overview of how to use it:

```rust
// This is a conceptual example, actual implementation details may vary
fn get_windows_fonts() {
    // Create DirectWrite factory
    let factory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED);
    
    // Create font collection
    let font_collection: IDWriteFontCollection = factory.GetSystemFontCollection();
    
    // Iterate through font families
    for i in 0..font_collection.GetFontFamilyCount() {
        let font_family = font_collection.GetFontFamily(i);
        
        // Get font family name
        let name = get_font_family_name(font_family);
        
        // Get font files
        let font = font_family.GetFont(0);
        let face = font.CreateFontFace();
        let files = face.GetFiles();
        
        for file in files {
            let path = get_file_path(file);
            println!("Font: {}, Path: {}", name, path);
        }
    }
}
```

## Advanced Usage and Customization

### Custom Font Detection Implementation

If you want to implement custom font detection logic, you can create your own implementation and integrate it with the existing code. Here's how:

```rust
// Custom font detection function
fn my_custom_font_detection() -> Vec<FontInfo> {
    let mut fonts = Vec::new();
    
    // Your custom logic here
    
    fonts
}

// Integrate with the main get_system_fonts function
let additional_fonts = my_custom_font_detection();
fonts.extend(additional_fonts);
```

### Error Handling and Fallbacks

The library implements a fallback mechanism to handle cases where the primary font detection methods fail:

```rust
// Example error handling pattern
if let Ok(font_families) = source.all_families() {
    // Primary method succeeded, process fonts
} else {
    // Try alternative methods
    #[cfg(target_os = "linux")]
    {
        if let Some(linux_fonts) = linux::get_fontconfig_fonts() {
            fonts.extend(linux_fonts);
        }
    }
    
    // ... platform-specific fallbacks for other OSes
}
```

This approach ensures that even if the primary font-kit based detection fails, the library can still return useful font information using platform-specific APIs.

## Building and Testing

To build the project:

```bash
npm run build
```

To test the font detection:

```javascript
const openGetFonts = require('open-get-fonts');

openGetFonts.getFonts()
  .then(fonts => {
    console.log(`Found ${fonts.length} fonts`);
    console.log(fonts.slice(0, 5)); // Print first 5 fonts
  });
``` 