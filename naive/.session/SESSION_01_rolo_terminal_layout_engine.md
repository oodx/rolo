# Rolo Terminal Layout Engine - Session 01
**Date**: 2025-09-12
**Status**: Active development - Terminal layout engine prototyping with bash demos

## Project Overview

**Rolo** is a terminal layout engine for Rust, designed to be similar to Yoga/Ink but focused on powerful terminal applications. The goal is flicker-free, efficient layout management for terminal UIs.

## Session Accomplishments

### 1. Core Terminal Layout Demos Created
- **`demo.sh`** - Comprehensive bash demo showing static layouts, live updates, interactive menus, and scrolling regions
- **`chat_demo.sh`** - Claude-style chat interface with flicker-free updates, dual orientation modes, slash commands, mouse isolation
- **`chat_boxy_demo.sh`** - Integration attempt with user's boxy tool (had rendering issues)

### 2. Key Technical Breakthroughs
- **Flicker-free updates** using cursor save/restore and selective clearing
- **Protected boundary system** preventing content overflow between layout regions
- **Dual orientation support** (top-down traditional vs bottom-up terminal feed style)
- **Mouse scroll isolation** using alternate screen buffer
- **Interactive slash command system** with real-time processing

### 3. Layout Engine Insights Discovered
- **Differential rendering** beats full-screen redraws (like Ink's approach)
- **Surgical cursor positioning** enables smooth updates
- **Protected zones** prevent layout contamination
- **ANSI control sequences** provide powerful terminal control when used correctly

## User's Custom Tools Identified

### boxy (v0.9.0)
- Professional box drawing utility with Unicode width handling
- 90+ color palette with semantic themes (info, success, error, warning)
- Header/title/footer system with status alignment
- Theme management and YAML import/export
- **Location**: Available as CLI command `boxy --help`

### jynx
- Intelligent syntax highlighter with auto-detection
- Theme management system
- Filter system for different highlighting styles
- Width formatting and alignment options
- **Location**: Available as CLI command `jynx --help`

## Current Challenges

### 1. Boxy Integration Issue
- **Problem**: Boxy creates complete, self-contained boxes but rolo needs dynamic real-time updates
- **Attempted**: Manual recreation of boxy border chars (failed - caused garbled output)
- **Need**: Find way to use boxy for components while maintaining real-time chat updates

### 2. Menu and Static Placement
- **Core Issue**: How to combine boxy's complete box approach with rolo's need for dynamic content regions
- **Challenge**: Static positioning vs complete box rendering paradigms don't naturally align

## Key Files Created

### Working Demos
1. **`/home/xnull/repos/code/rust/oodx/rolo/naive/demo.sh`**
   - Static layout demonstration
   - Live updates without flicker
   - Interactive menus with arrow key navigation
   - Scrolling regions within static borders

2. **`/home/xnull/repos/code/rust/oodx/rolo/naive/chat_demo.sh`**
   - Complete chat interface with flicker-free updates
   - Dual orientation modes (CHAT_ORIENTATION=bottom for feed style)
   - Slash commands: /help, /quit, /orientation, /spam, /clear, /status
   - Protected boundary system preventing layout corruption
   - Mouse scroll isolation using alternate screen buffer

3. **`/home/xnull/repos/code/rust/oodx/rolo/naive/chat_boxy_demo.sh`**
   - Attempted boxy integration (has rendering issues)
   - Shows approach of using boxy themes with manual positioning

### Project Structure
```
/home/xnull/repos/code/rust/oodx/rolo/
├── naive/
│   ├── demo.sh                    # Core layout techniques demo
│   ├── chat_demo.sh               # Working chat interface
│   ├── chat_boxy_demo.sh          # Boxy integration attempt
│   └── .session/
│       └── SESSION_01_*.md        # This file
├── README.md                      # "rows and columns and menus oh my"
├── NEXT.md                        # Previous session notes (Claude CLI hooks)
└── .git/                          # Git repository
```

## Technical Architecture Discovered

### Flicker-Free Rendering System
```bash
# Core techniques that work:
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
goto() { echo -ne "\033[${1};${2}H"; }  # Absolute positioning
# Protected boundaries prevent content overflow
# Selective clearing instead of full screen redraws
```

### Protected Zone System
```bash
CHAT_CONTENT_START=3
CHAT_CONTENT_END=$((CHAT_HEIGHT - 2))
INPUT_AREA_START=$((CHAT_HEIGHT + 3))
# Hard boundaries that content cannot cross
```

### Dual Orientation Logic
- **Top-down**: Traditional scrolling (like most chat apps)
- **Bottom-up**: Terminal feed style (newest messages stick to bottom)
- Environment variable: `CHAT_ORIENTATION=bottom`

## Next Session Instructions

### Immediate Tasks
1. **Resolve boxy integration** - Find way to use boxy for styling while maintaining dynamic updates
2. **Test jynx integration** - Explore syntax highlighting for code in chat messages
3. **Solve static placement vs dynamic content paradigm** - Core architectural challenge

### Files to Read First
1. **`chat_demo.sh`** - The working baseline demo with all core features
2. **User tool help**: Run `boxy --help` and `jynx --help` to understand capabilities
3. **`demo.sh`** - Reference for layout techniques that work

### Tools to Access
- **boxy**: CLI tool for professional box drawing and theming
- **jynx**: CLI tool for syntax highlighting
- Both tools available on user's system

### Key Concepts to Remember
- **Differential rendering** > full screen redraws
- **Protected boundaries** prevent layout corruption
- **Cursor positioning** enables surgical updates
- **User has custom tools** that should enhance, not replace, core layout engine

### Architecture Questions to Resolve
1. How to use boxy for component styling while maintaining real-time updates?
2. Can jynx enhance chat messages without breaking layout?
3. What's the right balance between complete box rendering vs dynamic positioning?
4. How should rolo (the Rust engine) implement these terminal layout patterns?

### Success Metrics
- Chat interface that uses boxy themes without garbled rendering
- Syntax highlighting integration that doesn't break layout
- Clear pattern for combining static positioning with complete box rendering
- Demonstrations that inform the Rust rolo architecture

## Session Context
This session was highly productive for understanding terminal layout fundamentals and creating working demos. The user was impressed with the flicker-free approach and dual orientation system. The main challenge moving forward is architectural - how to combine the user's professional styling tools (boxy/jynx) with the real-time update requirements of a dynamic layout engine.

**Next session should start by running the working chat demo to establish baseline, then carefully experiment with boxy integration strategies.**