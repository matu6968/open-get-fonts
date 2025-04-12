# Fontconfig API Usage in open-get-fonts

This document provides a detailed guide on how to use the fontconfig library for font detection in Linux environments, as implemented in the open-get-fonts library.

## Introduction to Fontconfig

Fontconfig is a library designed to provide system-wide font configuration, customization, and access for Linux and Unix-like operating systems. It helps applications find, enumerate, and select available fonts on the system.

## Basic Fontconfig Concepts

### Key Components

1. **Fontconfig** - The main entry point for the library
2. **Pattern** - Used to match font properties
3. **FontSet** - A collection of fonts matching a pattern
4. **ObjectSet** - A set of font properties to extract

## Getting Started with Fontconfig

### Initialization

The first step is to initialize the fontconfig library:

```rust
use fontconfig::Fontconfig;

// Initialize fontconfig
let fc = Fontconfig::new().or(None)?;
```

### Creating Patterns

Patterns are used to match fonts with specific properties:

```rust
use fontconfig::{Fontconfig, Pattern};

let fc = Fontconfig::new().unwrap();
let pattern = Pattern::new(&fc);
```

### Specifying Properties to Retrieve

To specify which properties you want to extract from fonts, create an ObjectSet:

```rust
use fontconfig::{Fontconfig, ObjectSet};
use std::ffi::CString;

let fc = Fontconfig::new().unwrap();
let mut object_set = ObjectSet::new(&fc);

// Convert fontconfig constants to CString objects
let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).unwrap();

// Add properties to the object set
object_set.add(&family_cstr);
object_set.add(&file_cstr);
```

## Listing and Filtering Fonts

### Listing All Fonts

To list all available fonts on the system:

```rust
use fontconfig::{Fontconfig, Pattern, ObjectSet};
use std::ffi::CString;

fn list_all_fonts() {
    let fc = Fontconfig::new().unwrap();
    let pattern = Pattern::new(&fc);
    let mut object_set = ObjectSet::new(&fc);
    
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
    let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).unwrap();
    
    object_set.add(&family_cstr);
    object_set.add(&file_cstr);
    
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    for pattern in font_set.iter() {
        if let Some(family) = pattern.get_string(&family_cstr) {
            if let Some(path) = pattern.get_string(&file_cstr) {
                println!("Font: {}, Path: {}", family, path);
            }
        }
    }
}
```

### Filtering Fonts by Properties

To filter fonts based on specific properties:

```rust
use fontconfig::{Fontconfig, Pattern, ObjectSet};
use std::ffi::CString;

fn list_bold_fonts() {
    let fc = Fontconfig::new().unwrap();
    let mut pattern = Pattern::new(&fc);
    
    // Set up CStrings for property names
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
    let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).unwrap();
    let weight_cstr = CString::new(fontconfig::FC_WEIGHT.to_bytes()).unwrap();
    
    // Add weight property to filter for bold fonts
    // FC_WEIGHT_BOLD is typically defined as 200
    pattern.add_integer(&weight_cstr, 200);
    
    // Create object set for the properties we want to extract
    let mut object_set = ObjectSet::new(&fc);
    object_set.add(&family_cstr);
    object_set.add(&file_cstr);
    
    // List fonts matching the pattern
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    // Process results
    for pattern in font_set.iter() {
        if let Some(family) = pattern.get_string(&family_cstr) {
            if let Some(path) = pattern.get_string(&file_cstr) {
                println!("Bold Font: {}, Path: {}", family, path);
            }
        }
    }
}
```

## Advanced Fontconfig Usage

### Font Matching

Font matching is a core feature of fontconfig:

```rust
use fontconfig::{Fontconfig, Pattern};
use std::ffi::CString;

fn match_font() {
    let fc = Fontconfig::new().unwrap();
    let mut pattern = Pattern::new(&fc);
    
    // Set properties for the desired font
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
    let family_value = CString::new("Arial").unwrap();
    pattern.add_string(&family_cstr, &family_value);
    
    // Match the best font
    if let Some(matched_pattern) = fontconfig::match_pattern(&pattern, true) {
        // Extract information from the matched pattern
        if let Some(family) = matched_pattern.get_string(&family_cstr) {
            println!("Matched Font: {}", family);
        }
    }
}
```

### Font Sorting

Fontconfig can sort fonts based on how well they match a pattern:

```rust
fn sort_fonts() {
    let fc = Fontconfig::new().unwrap();
    let mut pattern = Pattern::new(&fc);
    
    // Set up pattern with desired properties
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).unwrap();
    let family_value = CString::new("Sans").unwrap();
    pattern.add_string(&family_cstr, &family_value);
    
    // Sort fonts based on the pattern
    let sorted_set = fontconfig::sort_fonts(&pattern, true);
    
    // Process sorted results
    // The best matches come first
}
```

### Font Configuration Files

Fontconfig uses configuration files to control font behavior:

```rust
fn print_config_info() {
    // Get config home directory
    if let Some(config_home) = fontconfig::get_config_home() {
        println!("Config Home: {}", config_home.display());
    }
    
    // Get all config directories
    for dir in fontconfig::get_config_dirs() {
        println!("Config Dir: {}", dir.display());
    }
}
```

## Common Fontconfig Properties

Fontconfig defines many standard property names that can be used with patterns:

- `FC_FAMILY` - Font family name
- `FC_FILE` - Font file path
- `FC_WEIGHT` - Font weight (light, normal, bold, etc.)
- `FC_SLANT` - Font slant (roman, italic, oblique)
- `FC_WIDTH` - Font width (normal, condensed, expanded)
- `FC_SIZE` - Font size
- `FC_PIXEL_SIZE` - Font size in pixels
- `FC_STYLE` - Font style
- `FC_LANG` - Language coverage

## Error Handling

Proper error handling is important when working with fontconfig:

```rust
fn safe_font_listing() -> Result<(), String> {
    // Initialize fontconfig
    let fc = Fontconfig::new().map_err(|_| "Failed to initialize fontconfig")?;
    
    // Create pattern and object set
    let pattern = Pattern::new(&fc);
    let mut object_set = ObjectSet::new(&fc);
    
    // Add properties to extract
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes())
        .map_err(|_| "Failed to create CString")?;
    object_set.add(&family_cstr);
    
    // List fonts
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    // Process results
    for pattern in font_set.iter() {
        if let Some(family) = pattern.get_string(&family_cstr) {
            println!("Font: {}", family);
        }
    }
    
    Ok(())
}
```

## Implementation in open-get-fonts

The open-get-fonts library uses fontconfig as a fallback mechanism when the primary font detection method fails. Here's how it's implemented:

```rust
pub fn get_fontconfig_fonts() -> Option<Vec<FontInfo>> {
    // Initialize fontconfig
    let fc = Fontconfig::new().or(None)?;
    let mut fonts = Vec::new();
    
    // Create a pattern to match all fonts
    let pattern = Pattern::new(&fc);
    
    // Create an object set for the properties we want to extract
    let mut object_set = ObjectSet::new(&fc);
    
    // Add the family and file properties to retrieve
    let family_cstr = CString::new(fontconfig::FC_FAMILY.to_bytes()).ok()?;
    let file_cstr = CString::new(fontconfig::FC_FILE.to_bytes()).ok()?;
    
    object_set.add(&family_cstr);
    object_set.add(&file_cstr);
    
    // Use the global list_fonts function from fontconfig
    let font_set = fontconfig::list_fonts(&pattern, Some(&object_set));
    
    // Iterate through patterns in the font set
    for pattern in font_set.iter() {
        // Extract font family and path
        if let Some(family) = pattern.get_string(&family_cstr) {
            if let Some(path) = pattern.get_string(&file_cstr) {
                fonts.push(FontInfo {
                    name: family.to_string(),
                    path: path.to_string(),
                });
            }
        }
    }
    
    // Fallback to common system fonts if nothing was found
    if fonts.is_empty() {
        // Check common system font locations
        // ...
    }
    
    Some(fonts)
}
```

## Best Practices

When working with fontconfig in Rust:

1. **Always check for errors** - Fontconfig operations can fail for various reasons.
2. **Handle memory management properly** - Ensure that resources are properly released.
3. **Use patterns effectively** - Patterns are powerful but can be complex.
4. **Provide fallbacks** - Not all systems have the same fonts.
5. **Cache results when appropriate** - Font enumeration can be expensive.

## Resources

- [Fontconfig Documentation](https://www.freedesktop.org/software/fontconfig/fontconfig-user.html)
- [Fontconfig Rust Crate](https://crates.io/crates/fontconfig)

## Troubleshooting

Common issues when working with fontconfig:

1. **No fonts found** - Check if fontconfig is properly installed and configured.
2. **Incorrect character encodings** - Ensure proper string encoding when working with font names.
3. **Performance issues** - Font enumeration can be slow; consider caching results.
4. **API version mismatches** - The Rust fontconfig crate might not support all features of the latest fontconfig library.

## Conclusion

Fontconfig provides a powerful API for font management on Linux systems. By understanding its core concepts and API patterns, you can effectively integrate font detection and selection into your Rust applications. 