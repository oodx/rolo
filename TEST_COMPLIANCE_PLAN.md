# Rolo Test Compliance Plan
## Following RSB Testing Standards

Updated: 2025-09-15

## Overview
This document outlines the testing strategy for rolo following RSB's HOWTO_TEST.md patterns and MODULE_SPEC requirements.

## Test Structure (RSB Compliant)

### Directory Layout
```
tests/
├── sanity/
│   └── baseline.rs              # Visible demos (user-friendly outputs)
├── features/
│   ├── layout/
│   │   ├── sanity.rs           # Layout module core tests
│   │   ├── column.rs           # Column mode tests
│   │   ├── table.rs            # Table mode tests
│   │   └── list.rs             # List mode tests
│   ├── width/
│   │   ├── sanity.rs           # Width calculation tests
│   │   ├── boxy_adapter.rs     # Boxy adapter tests (feature-gated)
│   │   └── unicode.rs          # Unicode width tests
│   ├── stream/
│   │   ├── sanity.rs           # Stream processing tests
│   │   └── pipeline.rs         # Pipeline integration tests
│   ├── cli/
│   │   ├── sanity.rs           # CLI parsing tests
│   │   └── args.rs             # Argument handling tests
│   └── theme/
│       ├── sanity.rs           # Theme system tests
│       └── visual.rs           # Visual theme tests
├── uat/
│   ├── columns.rs              # Column mode UAT
│   ├── tables.rs               # Table mode UAT
│   ├── themes.rs               # Theme system UAT
│   ├── pipeline.rs             # Pipeline integration UAT
│   └── visual.rs               # Visual output UAT
├── integration/
│   ├── ecosystem.rs            # jynx + rolo + boxy tests
│   ├── tokens.rs               # RSB TokenStream tests
│   └── performance.rs          # Performance baseline tests
├── sh/
│   ├── benchmark.sh            # Performance benchmarking
│   ├── pipeline_e2e.sh         # End-to-end pipeline tests
│   └── deployment.sh           # Deployment validation
└── old/                        # Legacy tests (if any)
```

### Test Wrappers (Cargo Integration)
```
tests/
├── sanity_main.rs              # includes sanity/baseline.rs
├── features_layout.rs          # includes features/layout/*.rs
├── features_width.rs           # includes features/width/*.rs (with feature gates)
├── features_stream.rs          # includes features/stream/*.rs
├── features_cli.rs             # includes features/cli/*.rs
├── features_theme.rs           # includes features/theme/*.rs (with feature gates)
├── uat_main.rs                 # includes uat/*.rs
├── integration_ecosystem.rs    # includes integration/ecosystem.rs
├── integration_tokens.rs       # includes integration/tokens.rs
└── performance_baseline.rs     # includes integration/performance.rs
```

## Test Suites by Category

### 1. Sanity Tests
**Purpose**: Quick but meaningful functional checks with visible output
**Requirements**:
- ✅ Each module MUST have a sanity test
- ✅ Tests basic functionality without features
- ✅ Visible output for debugging
- ✅ Fast execution (< 1 second per module)

**Coverage**:
- Layout: Basic column/table/list formatting
- Width: ANSI stripping and Unicode width
- Stream: stdin/stdout processing
- CLI: Argument parsing and dispatch
- Theme: Basic theme loading

### 2. Feature Tests
**Purpose**: Comprehensive functional coverage for each module
**Requirements**:
- ✅ Cover all public APIs
- ✅ Feature-gated tests for optional dependencies
- ✅ Error handling validation
- ✅ Edge case coverage

**Feature Gates**:
```rust
#[cfg(feature = "width-boxy")]
mod boxy_adapter_tests { ... }

#[cfg(feature = "visual")]
mod theme_visual_tests { ... }

#[cfg(feature = "csv")]
mod csv_plugin_tests { ... }
```

### 3. UAT Tests (User Acceptance)
**Purpose**: Demonstrate user experience with visible output
**Requirements**:
- ✅ Show actual command usage
- ✅ Display sample outputs
- ✅ Validate user workflows
- ✅ Feature-gated for visual components

**Examples**:
```rust
#[test]
#[cfg(feature = "visual")]
fn uat_column_mode_demo() {
    println!("=== Column Mode Demo ===");
    let input = "apple banana cherry date elderberry fig";
    let output = rolo::layout::column_format(input, 3);
    println!("Input: {}", input);
    println!("Output (3 cols):\n{}", output);
}
```

### 4. Integration Tests
**Purpose**: End-to-end and multi-module flows
**Requirements**:
- ✅ Pipeline integration (jynx → rolo → boxy)
- ✅ TokenStream processing
- ✅ Performance benchmarks
- ✅ Real-world scenarios

## Test Runner Compliance

### Commands Supported
```bash
# Core test suites
./bin/test.sh run sanity         # All sanity tests
./bin/test.sh run layout         # Layout module tests
./bin/test.sh run width          # Width calculation tests
./bin/test.sh run stream         # Stream processing tests
./bin/test.sh run cli            # CLI tests

# Integration tests
./bin/test.sh run pipeline       # Pipeline integration
./bin/test.sh run tokens         # TokenStream tests
./bin/test.sh run ecosystem      # Full ecosystem tests

# UAT tests (visual output)
./bin/test.sh run uat-columns    # Column mode demo
./bin/test.sh run uat-tables     # Table mode demo
./bin/test.sh run uat-themes     # Theme system demo

# Suites
./bin/test.sh run smoke          # Fast core tests
./bin/test.sh run all            # All functional tests
./bin/test.sh run full           # Comprehensive suite

# Utilities
./bin/test.sh list               # List all available tests
./bin/test.sh help               # Show help
```

### Feature Flag Handling
The test runner automatically applies appropriate feature flags:
- Default tests: No features (minimal build)
- Width tests: `--features width-boxy`
- Visual tests: `--features visual`
- UAT tests: `--features width-boxy,visual`
- Full tests: `--features width-boxy,visual,csv,json,markdown`

## Implementation Checklist

### Phase 1: Foundation (v0.1)
- [ ] Create test directory structure
- [ ] Implement sanity_main.rs wrapper
- [ ] Create baseline sanity tests for each module
- [ ] Setup bin/test.sh runner
- [ ] Verify test discovery works

### Phase 2: Core Tests (v0.2)
- [ ] Implement features_layout.rs with comprehensive tests
- [ ] Implement features_width.rs with boxy adapter tests
- [ ] Implement features_stream.rs with pipeline tests
- [ ] Implement features_cli.rs with argument tests
- [ ] Add feature-gated test coverage

### Phase 3: Integration (v0.3)
- [ ] Create integration_ecosystem.rs for pipeline tests
- [ ] Create integration_tokens.rs for TokenStream tests
- [ ] Add performance_baseline.rs for benchmarking
- [ ] Implement shell scripts for E2E testing

### Phase 4: UAT (v0.4)
- [ ] Create uat_main.rs wrapper
- [ ] Implement visual UAT tests for each mode
- [ ] Add theme system UAT tests
- [ ] Create pipeline demonstration UATs

## Quality Gates

### Minimum Requirements (MVP)
- ✅ All modules have sanity tests
- ✅ Sanity tests pass without features
- ✅ Test runner works with basic commands
- ✅ No panics in normal operation

### Production Requirements (v1.0)
- ✅ 80%+ code coverage
- ✅ All feature combinations tested
- ✅ Performance benchmarks established
- ✅ UAT tests demonstrate all features
- ✅ Integration tests with ecosystem tools

## Error Handling Strategy

### Test-Specific Error Handling
- Use `utils::stderrx()` for non-visual logging in tests
- Avoid visual macros in core module tests
- Feature-gate visual error output
- Provide clear test failure messages

### Example:
```rust
#[test]
fn test_width_calculation() {
    let result = width::utils::get_display_width("test");
    assert_eq!(result, 4, "Basic ASCII width should be 4");

    // Use RSB logging pattern
    if result != 4 {
        utils::stderrx("error", "Width calculation failed for ASCII text");
    }
}
```

## Automation Integration

### CI/CD Pipeline
```yaml
# Basic CI (no features)
- name: Test Core
  run: cargo test

# Feature testing
- name: Test with Features
  run: cargo test --features width-boxy,visual

# Test runner validation
- name: Test Runner Smoke
  run: ./bin/test.sh run smoke

# Full validation
- name: Full Test Suite
  run: ./bin/test.sh run full --comprehensive
```

### Pre-commit Hooks
- Run smoke tests before commits
- Validate test runner functionality
- Ensure no feature leakage in default tests

## Documentation Requirements

### Test Documentation
- Each test file has header comment explaining purpose
- Complex tests include usage examples
- Feature-gated tests document requirements
- UAT tests include expected output samples

### Integration with RSB Patterns
- Follow MODULE_SPEC error handling
- Use RSB macros appropriately
- Maintain prelude policy compliance
- Document adapter patterns in tests

This plan ensures rolo follows RSB testing standards while providing comprehensive coverage for text layout functionality.