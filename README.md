# Rolo

Text layout tool for Unix pipelines - the spiritual love child of `pr`, `paste`, and `col`.

## Overview

Rolo is a Rust CLI utility for column and table layout of text streams. It provides ANSI-aware width handling, streaming performance, and integrates seamlessly with jynx (syntax highlighting) and boxy (decoration) via Unix pipes.

## Features

- **Column Mode**: Format text in N columns with proper alignment
- **Table Mode**: Auto-detect column widths and format as tables
- **List Mode**: Single column formatting with optional line numbers
- **ANSI-aware**: Correctly handles colored text and Unicode
- **Streaming**: Processes large datasets efficiently
- **Pipeline-friendly**: Works seamlessly with jynx and boxy

## Installation

```bash
# Build and deploy locally
./bin/deploy.sh

# Or build manually
cargo build --release
cp target/release/rolo ~/.local/bin/odx/
```

## Usage

```bash
# Column mode
echo "apple banana cherry date" | rolo --cols 2

# Table mode
cat data.tsv | rolo --table --headers

# Pipeline integration
command | jynx | rolo --cols 3 | boxy --title "Results"
```

## Development Status

This is v0.1.0 - foundational structure is complete, core functionality coming in Sprint 1-2.

## License

AGPL-3.0-or-later - see LICENSE file.