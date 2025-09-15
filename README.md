# Rolo 🧩

**Rolo**: The Spiritual Love Child of Unix Commands

Rolo is a pragmatic, powerful text layout tool that brings the elegance of Unix pipeline processing to text formatting. Inspired by classic commands like `pr`, `paste`, and `col`, Rolo provides flexible, intelligent text layout capabilities.

## 🚀 Project Status: v0.2 (In Development)

### Sprint Progress
- ✅ Sprint 1-2 Foundation (v0.1): 6/6 tasks complete (15/15 points)
- 🔨 Sprint 3-4 Core Modes (v0.2): 1/5 tasks complete (5/21 points)

## ✨ Latest Achievement: Column Mode (TASK-007)

Rolo now supports intelligent column-based text layout with:
- ANSI-aware width handling
- Flexible column configuration
- Minimal pipeline overhead

### Quick Example
```bash
echo "a b c d" | rolo --cols 2
```

## 🛠 Features

1. **RSB Framework Integration**
   - Compliant with MODULE_SPEC
   - AGPL Licensed
   - Modular, type-safe design

2. **Intelligent Text Processing**
   - Unix pipeline friendly
   - ANSI/Unicode width support
   - Configurable layout modes

3. **Flexible CLI**
   - Multiple layout modes
   - Comprehensive help and version information
   - Robust error handling

### Feature Flags
- `width-boxy`: Boxy-powered width calculations
- `visual`: Enhanced visual processing
- `csv-support`: CSV layout optimizations
- `json-support`: JSON-aware formatting

## 🔧 Upcoming Tasks

- TASK-008: Table Mode Implementation
- TASK-009: List Mode Implementation
- TASK-010: Terminal Width Detection

## 💻 Usage Examples

### Column Mode
```bash
# Basic 2-column layout
cat data.txt | rolo --cols 2

# Custom gap between columns
cat data.txt | rolo --cols 3 --gap 4
```

### Pipeline Integration
```bash
# Complex pipeline with jynx and boxy
command | jynx | rolo --cols 3 | boxy --title "Results"
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your feature
4. Add comprehensive tests
5. Submit a pull request

## 📦 Installation

```bash
# Build and deploy locally
./bin/deploy.sh

# Or build manually
cargo build --release
cp target/release/rolo ~/.local/bin/odx/
```

## 📄 License

AGPL-3.0-or-later

## 🔬 Development Philosophy

Rolo embraces the Unix philosophy: do one thing, do it well, and play nicely with other tools. Built with ❤️ using Rust and the RSB Framework.

---

*Crafted with precision, inspired by the timeless elegance of Unix pipelines.*