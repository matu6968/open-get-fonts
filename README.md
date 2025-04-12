# open-get-fonts

**open-get-fonts:** Open source reimplementation of the get-fonts native library seen in the Obsidian app originally written in C++

This library is part of a project to make Obsidian interoperable on various architectures, including ARM64 and ARMv7, by reimplementing its native components in Rust.

## What does this library do?

The `get-fonts` library in Obsidian is responsible for retrieving information about fonts installed on the system. This Rust implementation provides the same functionality using the `font-kit` crate for cross-platform font detection, with platform-specific optimizations for:

- **Linux**: Using fontconfig
- **macOS**: Using Core Text and Core Foundation
- **Windows**: Using DirectWrite

## Function Interfaces

The library exposes a single function:

```javascript
getFonts(): Promise<Array<{name: string, path: string}>>
```

This function returns a Promise that resolves to an array of font information objects, each containing:
- `name`: The font family name
- `path`: The path to the font file on disk

## Using open-get-fonts

After building the library, you can use it in your Node.js project:

```javascript
const { getFonts } = require('open-get-fonts');

// Get all system fonts
getFonts().then(fonts => {
  console.log('Found fonts:', fonts);
  // Example output: 
  // [
  //   { name: 'Arial', path: '/path/to/arial.ttf' },
  //   { name: 'Helvetica', path: '/path/to/helvetica.ttf' },
  //   ...
  // ]
}).catch(err => {
  console.error('Error getting fonts:', err);
});
```

## Building open-get-fonts

Building open-get-fonts requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

To run the build, run:

```sh
$ npm run build
```

This command uses the [@neon-rs/cli](https://www.npmjs.com/package/@neon-rs/cli) utility to assemble the binary Node addon from the output of `cargo`.

## Exploring open-get-fonts

After building open-get-fonts, you can explore its exports at the Node console:

```sh
$ npm i
$ npm run build
$ node
> require('.').greeting()
{ message: 'hello node' }
```

## Available Scripts

In the project directory, you can run:

#### `npm run build`

Builds the Node addon (`index.node`) from source, generating a release build with `cargo --release`.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm run build` and similar commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm run debug`

Similar to `npm run build` but generates a debug build with `cargo`.

#### `npm run cross`

Similar to `npm run build` but uses [cross-rs](https://github.com/cross-rs/cross) to cross-compile for another platform. Use the [`CARGO_BUILD_TARGET`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) environment variable to select the build target.

#### `npm run release`

Initiate a full build and publication of a new patch release of this library via GitHub Actions.

#### `npm run dryrun`

Initiate a dry run of a patch release of this library via GitHub Actions. This performs a full build but does not publish the final result.

#### `npm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).

## Project Layout

The directory structure of this project is:

```
open-get-fonts/
├── Cargo.toml
├── README.md
├── lib/
├── src/
|   ├── index.mts
|   └── index.cts
├── crates/
|   └── open-get-fonts/
|       └── src/
|           └── lib.rs
├── platforms/
├── package.json
└── target/
```

| Entry          | Purpose                                                                                                                                  |
|----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html), which informs the `cargo` command.                   |
| `README.md`    | This file.                                                                                                                               |
| `lib/`         | The directory containing the generated output from [tsc](https://typescriptlang.org).                                                    |
| `src/`         | The directory containing the TypeScript source files.                                                                                    |
| `index.mts`    | Entry point for when this library is loaded via [ESM `import`](https://nodejs.org/api/esm.html#modules-ecmascript-modules) syntax.       |
| `index.cts`    | Entry point for when this library is loaded via [CJS `require`](https://nodejs.org/api/modules.html#requireid).                          |
| `crates/`      | The directory tree containing the Rust source code for the project.                                                                      |
| `lib.rs`       | Entry point for the Rust source code.                                                                                                          |
| `platforms/`   | The directory containing distributions of the binary addon backend for each platform supported by this library.                          |
| `package.json` | The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.                    |
| `target/`      | Binary artifacts generated by the Rust build.                                                                                            |

## Learn More

Learn more about:

- [Neon](https://neon-bindings.com).
- [Rust](https://www.rust-lang.org).
- [Node](https://nodejs.org).
