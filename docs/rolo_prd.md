# Rolo — PRD for an ANSI-Aware Column & Table Tool

## Overview
Rolo is a Rust CLI utility for **column and table layout** of text streams, designed as the spiritual love child of Unix commands `pr`, `paste`, and `col`. It combines the simplicity of `paste` with the readability of `pr` and `column`, but with **ANSI/emoji-aware width handling**, **streaming performance**, and **boxy/jynx integration**. The goal is to provide clean, aligned columns or tables in pipelines without breaking color codes, while serving as a structured data layout tool that fits seamlessly into the Unix pipe ecosystem alongside boxy (decoration) and jynx (syntax highlighting).

## Goals
- Provide a **drop-in replacement** for `paste`/`column` in colored pipelines.
- Handle **ANSI escapes** and **Unicode graphemes** correctly for width calculations.
- Offer both **column mode** (fixed N items per row) and **table mode** (auto-fit, equal-width columns).
- Fit into the **jynx/boxy ecosystem**, using consistent themes and pipeline conventions.
- Be blazing fast, suitable for 10k+ lines/sec log streams.
- **Adopt RSB Framework** patterns for string-biased, pragmatic Rust development.
- Support Unix heritage features from `pr` (pagination, headers), `paste` (column merging), and `col` (line wrapping).

## User Stories
- *As a developer*, I want to print a list of env var names in 4 neat colored columns without alignment breaking.
- *As a user*, I want to quickly reformat command output into a table for readability.
- *As a theming author*, I want table borders and column guides that match boxy/jynx palettes.

## Functional Requirements
1. **Column Mode**
   - `--cols N` → group N items per row.
   - `--sep` → token separator (default: whitespace).
   - `--gap` → spacing between cells.
   - Streaming safe: no need to pre-buffer entire input.

2. **Table Mode**
   - `--table` → switch to table layout.
   - `--delim` → column delimiter (default: tab, optional CSV parsing).
   - `--headers` → treat first row as header, apply different style.
   - `--align` → per-column alignment (L/C/R).
   - `--pad` → padding inside cells.
   - `--border` → none | ascii | light (Unicode).

3. **Width Handling**
   - `--fit` (default): fit to terminal width.
   - `--width N`: fixed width.
   - `--truncate` ellipsis or cut.
   - `--nowrap`: allow overflow.

4. **Theme & Integration**
   - `--theme plain|dim|guide`: basic visual presets.
   - Colors optional; should integrate with jynx/boxy palettes.
   - Boxy can wrap rolo output without breaking spacing.

5. **Compatibility**
   - Works as a pipe citizen (stdin → stdout).
   - No TUI, no interactivity.
   - Respects `$COLUMNS` when fitting.

## Non-Functional Requirements
- **Performance**: Must handle >10k lines/sec.
- **Accuracy**: Width calculations must be correct for ANSI, Unicode EAW, and graphemes (emoji).
- **Extensibility**: Modular design for future CSV/JSON input, advanced theming.

## CLI Examples
```bash
# Column mode, 4 cols
printf '%s\n' $LIST | rolo --cols 4

# Table mode, headers, auto-fit
cat data.tsv | rolo --table --headers --fit

# Env diff in 3 cols
ediff | jynx --theme themes/envdiff.yml | rolo --cols 3 | boxy --title "ENV Δ"

# Pipeline integration with jynx and boxy
echo "text text text" | jynx <jynx_options> | rolo --cols 2 | boxy <boxy_options>

# Standalone structured data display
ls -la | rolo --table --headers
```

## RSB Framework Integration

### Architecture Alignment
- **Module Structure**: Follow RSB's MODULE_SPEC for consistent module organization
- **REBEL Philosophy**: Embrace string-biased, pragmatic patterns over pure Rust idioms
- **Error Handling**: Use RSB's `validate!()` macros for fail-fast, clear error messages
- **CLI Integration**: Leverage RSB's built-in `Args` struct and `dispatch!()` system
- **String Operations**: Utilize RSB's powerful string macros and parameter expansion

### MODULE_SPEC Compliant Architecture

Rolo will strictly follow RSB's MODULE_SPEC for consistent, maintainable module structure:

```
rolo/
├── src/
│   ├── lib.rs            # Library entry, prelude exports
│   ├── main.rs           # CLI binary entry
│   ├── prelude.rs        # Curated public API surface
│   │
│   ├── layout/           # Core layout module (MODULE_SPEC compliant)
│   │   ├── mod.rs        # Orchestrator, re-exports
│   │   ├── utils.rs      # Curated low-level helpers
│   │   ├── helpers.rs    # Internal implementations
│   │   ├── macros.rs     # Module-owned macros
│   │   ├── error.rs      # Typed error enums
│   │   ├── column.rs     # Column mode logic
│   │   └── table.rs      # Table mode logic
│   │
│   ├── stream/           # Stream processing module
│   │   ├── mod.rs        # Orchestrator
│   │   ├── utils.rs      # Stream utilities
│   │   ├── helpers.rs    # Internal stream helpers
│   │   └── error.rs      # Stream errors
│   │
│   ├── width/            # Width calculation module
│   │   ├── mod.rs        # Orchestrator
│   │   ├── utils.rs      # Width utilities (boxy integration)
│   │   ├── width_boxy_adapter.rs  # Adapter for boxy's width_plugin
│   │   └── error.rs      # Width calculation errors
│   │
│   ├── theme/            # Theme system module
│   │   ├── mod.rs        # Orchestrator
│   │   ├── utils.rs      # Theme utilities
│   │   ├── macros.rs     # Theme macros
│   │   ├── plain.rs      # Plain theme
│   │   └── guide.rs      # Guide theme
│   │
│   ├── cli/              # CLI module (RSB cli patterns)
│   │   ├── mod.rs        # Orchestrator
│   │   ├── args.rs       # Argument parsing
│   │   ├── dispatch.rs   # Command dispatch
│   │   └── error.rs      # CLI errors
│   │
│   └── plugins/          # Plugin system (Phase 2)
│       ├── mod.rs        # Plugin orchestrator
│       ├── trait.rs      # RoloPlugin trait
│       └── registry.rs   # Plugin registry
```

#### MODULE_SPEC Key Principles for Rolo:

1. **Module Layout**:
   - Each module has `mod.rs` as orchestrator (no business logic)
   - `utils.rs` contains curated low-level helpers
   - `helpers.rs` for internal implementations
   - `macros.rs` for module-owned macros
   - `error.rs` for typed error enums

2. **Cross-Module Integration**:
   - Use adapters pattern (e.g., `width_boxy_adapter.rs`)
   - Isolate foreign module dependencies
   - Provide graceful fallbacks with feature flags

3. **Prelude Policy**:
   - Export only stable, commonly-used items
   - No optional/visual features in prelude
   - Module-owned macros via `prelude::macros`

4. **Error Handling**:
   - Each module owns its error types
   - Consistent error messaging
   - No dependency on visual macros in core

5. **Testing Structure**:
   - Baseline tests without optional features
   - Feature-gated integration tests
   - Per-module test coverage

### Width Calculation Strategy
- **Reuse Boxy's width_plugin**: Leverage the battle-tested `get_display_width()` and `get_terminal_width()` functions from boxy
- **No reinvention**: Avoid creating a new width implementation when boxy's solution handles ANSI stripping and Unicode width correctly
- **Dependency approach**: Either import boxy as a library dependency or extract the width_plugin as a shared crate

## Plugin Architecture

### Core Plugin System
Rolo should support a flexible plugin pattern for extensibility:

1. **Plugin Interface**
   - Trait-based plugin system for custom formatters, filters, and transformers
   - Plugins can hook into: input processing, layout calculation, output formatting
   - Runtime plugin loading via configuration

2. **Built-in Plugins**
   - **Width Calculator**: Boxy's width_plugin (default)
   - **CSV Parser**: Auto-detect and parse CSV input
   - **JSON Formatter**: Structure JSON data into tables
   - **Markdown Table**: Generate markdown-compatible tables
   - **Color Themes**: Pluggable theme system

3. **Plugin Configuration**
   ```toml
   # ~/.config/rolo/config.toml
   [plugins]
   width = "boxy"  # or custom implementation

   [plugins.csv]
   enabled = true
   delimiter = ","
   quote = '"'

   [plugins.markdown]
   enabled = false
   style = "github"
   ```

### Cargo Feature Flags (MODULE_SPEC Compliant)

Following RSB's feature flag patterns:

```toml
[features]
# Core defaults - lean and focused
default = []

# Width calculation strategies
width-boxy = []                  # Use boxy's width implementation (adapter pattern)
width-unicode = []                # Pure unicode-width implementation

# Visual and formatting features
visual = []                       # Base visual features
themes = ["visual"]               # Theme system
borders = ["visual"]              # Border drawing
colors = ["visual"]               # Color support

# Input/Output formats
csv = ["csv-crate"]              # CSV parsing support
json = ["serde_json"]            # JSON support
markdown = []                    # Markdown table generation
tokens = []                      # RSB TokenStream support

# Umbrella features
full = ["width-boxy", "themes", "borders", "colors", "csv", "json", "markdown", "tokens"]
minimal = ["width-unicode"]      # Minimal standalone build

# Development features
dev = []                         # Development helpers
bench = []                       # Benchmarking support
```

#### Feature Guidelines:
- Core functionality has no feature dependencies
- Visual features are opt-in via feature flags
- Adapters use feature gates with fallbacks
- Tests cover both default and feature-enabled profiles

### Plugin Development Guidelines
- Plugins must implement the `RoloPlugin` trait
- Plugins should be stateless or manage their own state
- Performance-critical plugins should provide benchmarks
- Documentation required for public plugin APIs

## Implementation Plan
- **Phase 1 (MVP)**
  - Column mode: `--cols`, `--sep`, ANSI width safe.
  - Table mode: `--table`, `--delim`, auto column widths.
  - Width: `--fit`, `--width N`.
  - Borders: `--border none|ascii`.

- **Phase 2**
  - Headers styling.
  - Advanced borders (Unicode light/double).
  - Per-column alignment.
  - Truncate/wrap policies.

- **Phase 3**
  - CSV/JSON autodetect.
  - Column width constraints (`--col-width 10,*,20`).
  - Theming integration with boxy/jynx.
  - Global presets (like `--theme debug`).

## Open Questions & Clarifications

### Technical Decisions
- **Auto-detection**: Should Rolo auto-detect when to use column vs table mode based on input patterns?
- **ANSI Handling**:
  - Should it support inline ANSI stripping (`--strip-ansi`)?
  - What levels of stripping (partial vs full)?
  - Performance impact of comprehensive width calculations?
- **Theme Integration**:
  - How much overlap with boxy themes vs its own theme system?
  - Should themes be shareable between jynx/boxy/rolo?
  - Color preservation guarantees across the pipeline?

### Unix Heritage Features
- **From `pr`**: Page formatting, headers, footers, line numbering?
- **From `paste`**: Multiple file merging, serial vs parallel modes?
- **From `col`**: Reverse line feeds, tab expansion, line wrapping policies?

### Performance Considerations
- Width calculation caching strategies for repeated ANSI sequences
- Memory vs speed trade-offs for large datasets
- Streaming buffer sizes for optimal throughput

## Next Steps
1. **RSB Integration**: Set up project with RSB framework dependencies and module structure
2. **Import Boxy Width Plugin**: Configure boxy as dependency or extract width_plugin as shared crate
3. **Plugin System Design**: Implement trait-based plugin architecture with configuration support
4. Implement streaming `--cols` mode with RSB's stream processing patterns
5. Build minimal `--table` with auto width using boxy's width calculations
6. Integrate `$COLUMNS` detection via RSB's host module
7. Create integration tests with jynx and boxy pipelines
8. Package as Rust crate + CLI (`rolo`) following RSB conventions with feature flags

