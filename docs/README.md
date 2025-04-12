# open-get-fonts Documentation

This directory contains documentation for the open-get-fonts library.

## Available Documentation

- [Main Usage Documentation](../USAGE.md) - Comprehensive guide on using the open-get-fonts library
- [Fontconfig API](FONTCONFIG.md) - Detailed guide on using the fontconfig library for Linux font detection

## Platform-Specific Implementations

The open-get-fonts library provides cross-platform font detection through the following implementations:

- **Linux**: Uses the fontconfig library 
- **macOS**: Uses the Core Text framework
- **Windows**: Uses the DirectWrite API

Each platform-specific implementation is contained in its own module:
- `linux.rs` - Linux implementation using fontconfig
- `macos.rs` - macOS implementation using Core Text
- `windows.rs` - Windows implementation using DirectWrite

## Building and Testing

See the [main usage documentation](../USAGE.md) for instructions on building and testing the library.

## Contributing

When contributing to platform-specific implementations, please consult the appropriate documentation:

- For Linux contributions, refer to the [Fontconfig API](FONTCONFIG.md) documentation
- For macOS contributions, consult the [Core Text documentation](https://developer.apple.com/documentation/coretext)
- For Windows contributions, consult the [DirectWrite documentation](https://docs.microsoft.com/en-us/windows/win32/directwrite/direct-write-portal)

## API Overview

The open-get-fonts library exposes a simple API for retrieving font information:

JavaScript:
```javascript
const openGetFonts = require('open-get-fonts');
openGetFonts.getFonts().then(fonts => console.log(fonts));
```

Rust:
```rust
use open_get_fonts::{FontInfo, get_system_fonts};
let fonts: Vec<FontInfo> = get_system_fonts();
``` 