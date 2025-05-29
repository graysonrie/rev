# Revit CLI Tool

A command-line interface tool for managing Revit add-in projects, making it easier to build, export, and manage Revit add-ins.

## Features

- Build Revit add-in projects using MSBuild or dotnet
- Export add-ins to the correct Revit directory
- Manage Revit version settings
- Locate project DLLs
- Handles both standard add-ins and web-based UIs (Next.js)

## Installation

### Option 1: Install from source

1. Make sure you have Rust installed ([Install Rust](https://rustup.rs/))
2. Clone this repository
3. Run:
```bash
cargo install --path .
```

### Option 2: Install from binary
Download the latest release from the releases page and add it to your system PATH.

## Prerequisites

- Windows OS
- .NET SDK or Visual Studio Build Tools
- Revit installation (2019-2025)

## Usage

```bash
# Build the current project
rev build

# Export the add-in to Revit
rev export

# Check current Revit version
rev revit-version

# Change target Revit version
rev change-revit-version

# Locate project DLL
rev locate
```

## Development Requirements

- Rust 1.75 or later
- Windows 10/11
- Visual Studio 2022 or MSBuild Tools 2022
- .NET SDK 6.0 or later

## Building from Source

1. Clone the repository
2. Run `cargo build --release`
3. The binary will be available in `target/release/rev.exe`

## License

MIT License
