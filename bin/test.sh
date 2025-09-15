#!/bin/bash
# Rolo Test Entry Point
# Unified interface for running all Rolo tests following RSB patterns

set -e

# Configuration
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TEST_DIR="$ROOT_DIR/tests"

# Try to find boxy for pretty output (optional)
BOXY=""
if command -v boxy >/dev/null 2>&1; then
    BOXY="boxy"
elif [[ -f "./target/release/boxy" ]]; then
    BOXY="./target/release/boxy"
elif [[ -f "../boxy/target/release/boxy" ]]; then
    BOXY="../boxy/target/release/boxy"
fi

# Parse optional flags (can be anywhere in arguments)
VERBOSE_MODE="false"
QUICK_MODE="true"  # Default to quick mode
COMPREHENSIVE_MODE="false"
ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --verbose|-v)
            VERBOSE_MODE="true"
            shift 1
            ;;
        --quick)
            QUICK_MODE="true"
            COMPREHENSIVE_MODE="false"
            shift 1
            ;;
        --comprehensive|--full)
            QUICK_MODE="false"
            COMPREHENSIVE_MODE="true"
            shift 1
            ;;
        *)
            ARGS+=("$1")
            shift 1
            ;;
    esac
done

# Restore non-flag arguments
set -- "${ARGS[@]}"

# Available tests for Rolo
declare -A TESTS=(
    # Core functionality tests
    ["sanity"]="sanity_main"                     # Sanity package (core + baseline)
    ["baseline"]="baseline_main"                 # Baseline tests (no features required)
    ["layout"]="features_layout"                 # Layout module tests (column/table/list)
    ["width"]="features_width"                   # Width calculation tests (boxy adapter)
    ["stream"]="features_stream"                 # Stream processing tests
    ["cli"]="features_cli"                       # CLI argument parsing tests
    ["theme"]="features_theme"                   # Theme system tests

    # Integration tests
    ["pipeline"]="integration_pipeline"          # Pipeline tests with jynx/boxy
    ["tokens"]="integration_tokens"              # TokenStream processing tests
    ["ecosystem"]="integration_ecosystem"        # Full ecosystem integration

    # UAT tests
    ["uat-columns"]="uat_main"                   # UAT: column mode demo
    ["uat-tables"]="uat_main"                    # UAT: table mode demo
    ["uat-themes"]="uat_main"                    # UAT: theme system demo
    ["uat-pipeline"]="uat_main"                  # UAT: pipeline demo
    ["visual-uat"]="visual_uat_main.rs"          # Visual UAT: executive formatting demos

    # Performance tests
    ["perf"]="performance_baseline"              # Performance baseline tests
    ["bench"]="sh/benchmark"                     # Benchmarking script

    # Feature-gated tests
    ["width-boxy"]="feature_gated_main"          # Width tests with boxy feature
    ["visual"]="feature_gated_main"              # Visual output tests
    ["csv"]="feature_gated_main"                 # CSV plugin tests
    ["json"]="feature_gated_main"                # JSON plugin tests
    ["markdown"]="feature_gated_main"            # Markdown plugin tests

    # Comprehensive suites
    ["all"]="all-tests"                          # Run all test categories
    ["smoke"]="smoke-tests"                      # Quick smoke test suite
    ["full"]="comprehensive-suite"               # Full validation test suite
)

show_help() {
    if [[ -n "$BOXY" ]]; then
        cat <<-EOF | $BOXY --theme info --title "🎯 Rolo Test Runner" --width max
Available Commands:
  test.sh [--comprehensive|--verbose] run <test>    Run specific test
  test.sh list                                      List available tests
  test.sh help                                      Show this help

Options:
  --comprehensive        Run full validation test suite
  --quick                Force quick mode (default)
  --verbose              Show detailed test output

Core Tests:
  sanity                 Core functionality unit tests (Rust)
  baseline               Baseline tests (no features required)
  layout                 Layout module tests (column/table/list)
  width                  Width calculation tests (boxy adapter)
  stream                 Stream processing tests
  cli                    CLI argument parsing tests
  theme                  Theme system tests

Integration Tests:
  pipeline               Pipeline tests with jynx/boxy
  tokens                 TokenStream processing tests
  ecosystem              Full ecosystem integration

UAT Tests:
  uat-columns            Column mode demonstration
  uat-tables             Table mode demonstration
  uat-themes             Theme system demonstration
  uat-pipeline           Pipeline demonstration
  visual-uat             Visual formatting demonstrations

Suites:
  all                    Run all test categories
  smoke                  Quick smoke test suite
  full                   Full validation test suite
EOF
    else
        echo "🎯 ROLO TEST RUNNER"
        echo "==================="
        echo
        echo "Available Commands:"
        echo "  test.sh [--comprehensive|--verbose] run <test>    Run specific test"
        echo "  test.sh list                                      List available tests"
        echo "  test.sh help                                      Show this help"
        echo
        echo "Options:"
        echo "  --comprehensive        Run full validation test suite"
        echo "  --quick                Force quick mode (default)"
        echo "  --verbose              Show detailed test output"
        echo
        echo "Core Tests:"
        echo "  sanity                 Core functionality unit tests (Rust)"
        echo "  baseline               Baseline tests (no features required)"
        echo "  layout                 Layout module tests (column/table/list)"
        echo "  width                  Width calculation tests (boxy adapter)"
        echo "  stream                 Stream processing tests"
        echo "  cli                    CLI argument parsing tests"
        echo "  theme                  Theme system tests"
        echo
        echo "Integration Tests:"
        echo "  pipeline               Pipeline tests with jynx/boxy"
        echo "  tokens                 TokenStream processing tests"
        echo "  ecosystem              Full ecosystem integration"
        echo
        echo "UAT Tests:"
        echo "  uat-columns            Column mode demonstration"
        echo "  uat-tables             Table mode demonstration"
        echo "  uat-themes             Theme system demonstration"
        echo "  uat-pipeline           Pipeline demonstration"
        echo "  visual-uat             Visual formatting demonstrations"
        echo
        echo "Suites:"
        echo "  all                    Run all test categories"
        echo "  smoke                  Quick smoke test suite"
        echo "  full                   Full validation test suite"
    fi
}

list_tests() {
    if [[ -n "$BOXY" ]]; then
        echo "📋 Mapped tests:" | $BOXY --theme info --title "🎯 Rolo Test Discovery" --width max
    else
        echo "📋 Mapped tests:"
        echo "==============="
    fi

    if [[ -d "$TEST_DIR" ]]; then
        for test_name in "${!TESTS[@]}"; do
            local test_file="${TESTS[$test_name]}"

            if [[ "$test_name" == "sanity" ]]; then
                if [[ -f "$TEST_DIR/sanity_main.rs" ]]; then
                    echo "✅ $test_name → sanity_main.rs (core + baseline)"
                else
                    echo "❌ $test_name → sanity_main.rs (missing)"
                fi
            elif [[ -f "$TEST_DIR/$test_file.sh" ]]; then
                echo "✅ $test_name → $test_file.sh"
            elif [[ -f "$TEST_DIR/$test_file" ]]; then
                echo "✅ $test_name → $test_file"
            elif [[ -f "$TEST_DIR/$test_file.rs" ]]; then
                echo "✅ $test_name → $test_file.rs"
            else
                echo "❌ $test_name → $test_file (missing)"
            fi
        done
        echo
        echo "Auto‑discovered wrappers:"
        for wrap in $(find "$TEST_DIR" -maxdepth 1 -type f -name "*.rs" -printf "%f\n" 2>/dev/null | sort); do
            base="${wrap%.rs}"
            echo "  • $base"
        done
    else
        echo "❌ Test directory not found: $TEST_DIR"
    fi
}

run_test() {
    local test_name="$1"

    if [[ -z "$test_name" ]]; then
        echo "❌ Error: Test name required"
        echo "Use: test.sh run <test>"
        echo "Available tests: ${!TESTS[*]}"
        exit 1
    fi

    if [[ ! "${TESTS[$test_name]+exists}" ]]; then
        # Fallback: run by wrapper filename or shell script name
        if [[ -f "$TEST_DIR/$test_name.rs" ]]; then
            echo "ℹ️  Running auto‑discovered wrapper: $test_name.rs"
            cargo test --test "$test_name" -- --nocapture
            exit 0
        elif [[ -f "$TEST_DIR/sh/$test_name.sh" ]]; then
            echo "ℹ️  Running shell test: tests/sh/$test_name.sh"
            exec bash "$TEST_DIR/sh/$test_name.sh"
        else
            echo "❌ Error: Unknown test '$test_name'"
            echo "Available tests: ${!TESTS[*]}"
            echo "Auto wrappers available:"
            find "$TEST_DIR" -maxdepth 1 -type f -name "*.rs" -printf "  • %f\n" 2>/dev/null | sed 's/\.rs$//'
            exit 1
        fi
    fi

    local test_file="${TESTS[$test_name]}"

    # If mapping points to a Rust wrapper (tests/<name>.rs), run as Cargo test
    if [[ "$test_file" == *.rs && -f "$TEST_DIR/$test_file" ]]; then
        local wrapper_name="${test_file%.rs}"
        if [[ -n "$BOXY" ]]; then
            echo "🦀 Running Rust wrapper: $test_file" | $BOXY --theme success --title "🎯 Rolo Test Runner" --width max
        else
            echo "🦀 Running Rust wrapper: $test_file"
        fi

        # Determine feature flags needed
        local features=""
        case "$test_name" in
            *width-boxy*|*theme*|*csv*|*json*|*markdown*)
                features="--features width-boxy,visual,csv-support"
                ;;
            *visual*|*uat*|*pipeline*)
                features="--features width-boxy,visual"
                ;;
        esac

        if [[ "$VERBOSE_MODE" == "true" ]]; then
            cargo test $features --test "$wrapper_name" -- --nocapture
        else
            cargo test $features --test "$wrapper_name"
        fi
        exit 0
    fi

    # Header
    if [[ -n "$BOXY" ]]; then
        echo "🚀 Running Rolo test: $test_name" | $BOXY --theme success --title "🎯 Rolo Test Runner" --width max
    else
        echo "🚀 Running Rolo test: $test_name"
        echo "============================="
    fi
    echo

    # Change to project root
    cd "$ROOT_DIR"

    # Export test configuration
    export ROLO_TEST_MODE="true"
    export ROLO_VERBOSE="${VERBOSE_MODE}"
    export ROLO_QUICK_MODE="${QUICK_MODE}"
    export ROLO_COMPREHENSIVE="${COMPREHENSIVE_MODE}"

    # Handle different test types
    case "$test_name" in
        "all")
            # Run a broad set of tests across categories
            "$0" run baseline
            "$0" run sanity
            "$0" run layout
            "$0" run width
            "$0" run stream
            "$0" run cli
            "$0" run pipeline
            "$0" run uat-columns
            "$0" run uat-tables
            "$0" run visual-uat
            ;;
        "smoke")
            # Quick smoke tests
            "$0" run baseline
            "$0" run sanity
            "$0" run layout
            ;;
        "full")
            # Comprehensive test suite
            "$0" run all
            "$0" run tokens
            "$0" run ecosystem
            "$0" run perf
            ;;
        *)
            # Individual test
            if [[ -f "$TEST_DIR/sh/$test_file.sh" ]]; then
                # Shell script test
                exec bash "$TEST_DIR/sh/$test_file.sh"
            else
                echo "❌ Test implementation not found: $test_file"
                exit 1
            fi
            ;;
    esac
}

# Main command dispatch
case "${1:-help}" in
    "run")
        run_test "$2"
        ;;
    "list")
        list_tests
        ;;
    "help"|"--help"|"-h"|"")
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo "Use: test.sh help"
        exit 1
        ;;
esac