#!/bin/bash

# Multi-Pane Positioning Demo - Using Zellij's Character-Chunk Pattern
# Breakthrough: No multiple scroll regions - just absolute positioning!

# Terminal setup
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor
stty -echo

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Layout configuration (like Zellij's layout engine)
PANE_A_X=1
PANE_A_Y=2
PANE_A_WIDTH=30
PANE_A_HEIGHT=10

PANE_B_X=32
PANE_B_Y=2
PANE_B_WIDTH=30
PANE_B_HEIGHT=10

PANE_C_X=1
PANE_C_Y=14
PANE_C_WIDTH=61
PANE_C_HEIGHT=8

STATUS_X=1
STATUS_Y=1
STATUS_WIDTH=$TERM_WIDTH

# Content buffers (independent like Zellij's Grid instances)
declare -a PANE_A_BUFFER=(
    "┌─ Chat Messages ─────────┐"
    "│ user: hey there!        │"
    "│ bot: Hello! How are you?│"
    "│ user: great, testing    │"
    "│ bot: Awesome! This is   │"
    "│      working perfectly! │"
    "│                         │"
    "│                         │"
    "└─────────────────────────┘"
)

declare -a PANE_B_BUFFER=(
    "┌─ System Monitor ────────┐"
    "│ CPU: 45% ████░░░░░░     │"
    "│ RAM: 62% ██████░░░░     │"
    "│ Network: 1.2MB/s ↑↓    │"
    "│ Connections: 42         │"
    "│ Uptime: 2h 15m          │"
    "│ Load: 0.8, 0.9, 1.1     │"
    "│                         │"
    "└─────────────────────────┘"
)

declare -a PANE_C_BUFFER=(
    "┌─ Code Output ─────────────────────────────────────────────┐"
    "│ $ cargo build --release                                   │"
    "│    Compiling rolo v0.1.0 (/home/user/rolo)              │"
    "│     Finished release [optimized] target(s) in 2.45s      │"
    "│ $ ./target/release/rolo --multi-pane                     │"
    "│   ✓ Layout engine initialized                            │"
    "│   ✓ Character-chunk compositor ready                     │"
    "└───────────────────────────────────────────────────────────┘"
)

# ANSI positioning function (like Zellij's vte_goto_instruction)
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Character-chunk renderer (Zellij's core pattern)
render_character_chunks() {
    local pane_name=$1
    local x=$2
    local y=$3
    local -n buffer_ref=$4

    for i in "${!buffer_ref[@]}"; do
        local row=$((y + i))
        goto "$row" "$x"
        printf "%s" "${buffer_ref[i]}"
    done
}

# Layout compositor (renders all panes using absolute positioning)
render_all_panes() {
    # Status bar
    goto $STATUS_Y $STATUS_X
    printf "\033[7m%*s\033[0m" $STATUS_WIDTH " ROLO Multi-Pane Demo - Zellij's Positioning Pattern - Press 'q' to quit "

    # Render each pane using character-chunk positioning
    render_character_chunks "PANE_A" $PANE_A_Y $PANE_A_X PANE_A_BUFFER
    render_character_chunks "PANE_B" $PANE_B_Y $PANE_B_X PANE_B_BUFFER
    render_character_chunks "PANE_C" $PANE_C_Y $PANE_C_X PANE_C_BUFFER

    # Debug info at bottom
    goto $((TERM_HEIGHT - 1)) 1
    printf "\033[2K\033[90mPattern: Independent buffers → Layout calc → ANSI positioning (\\\u{1b}[row;colH)\033[0m"
}

# Content update functions (simulate live data)
update_chat_pane() {
    local new_messages=(
        "│ user: this is amazing!  │"
        "│ bot: Right? No flicker! │"
        "│ user: positioning rocks │"
        "│ bot: Zellij's secret!   │"
    )

    # Shift existing messages up and add new ones (like Zellij's scroll)
    for i in {1..4}; do
        if [ $((i + 4)) -lt ${#PANE_A_BUFFER[@]} ]; then
            PANE_A_BUFFER[i]="${new_messages[$((i-1))]}"
        fi
    done
}

update_system_pane() {
    local cpu=$((RANDOM % 100))
    local ram=$((RANDOM % 100))
    local net_speed=$((RANDOM % 50 + 10))

    PANE_B_BUFFER[1]="│ CPU: ${cpu}% $(printf '█%.0s' $(seq 1 $((cpu/10))))$(printf '░%.0s' $(seq 1 $((10-cpu/10))))     │"
    PANE_B_BUFFER[2]="│ RAM: ${ram}% $(printf '█%.0s' $(seq 1 $((ram/10))))$(printf '░%.0s' $(seq 1 $((10-ram/10))))     │"
    PANE_B_BUFFER[3]="│ Network: ${net_speed}.${RANDOM:0:1}MB/s ↑↓    │"
    PANE_B_BUFFER[4]="│ Connections: $((RANDOM % 100 + 20))         │"
}

update_code_pane() {
    local commands=(
        "│ $ rolo --layout split-vertical                        │"
        "│ $ rolo --layout grid --panes 4                       │"
        "│ $ rolo --theme matrix --positioning absolute         │"
        "│ $ rolo --demo character-chunks                        │"
    )

    PANE_C_BUFFER[4]="${commands[$((RANDOM % ${#commands[@]}))]}"
    PANE_C_BUFFER[5]="│   ✓ Positioning compositor updated: $(date +%H:%M:%S)     │"
}

# Main demo loop
main() {
    echo "🚀 ROLO Multi-Pane Demo - Zellij's Character-Chunk Pattern!"
    echo "Press any key to start the demo..."
    read -n 1 -s

    # Initial render
    render_all_panes

    # Real-time update loop (like Zellij's output thread)
    while true; do
        # Check for quit
        if read -t 0.5 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Update content buffers (simulate live data)
        update_system_pane

        # Every 3 seconds, update chat
        if [ $((SECONDS % 3)) -eq 0 ]; then
            update_chat_pane
        fi

        # Every 2 seconds, update code output
        if [ $((SECONDS % 2)) -eq 0 ]; then
            update_code_pane
        fi

        # Re-render using positioning (NO FLICKER!)
        render_all_panes
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Demo complete! The secret: Character-by-character absolute positioning!"
    echo "🎯 Key insight: Each pane = independent buffer + layout positioning"
    echo "📍 No scroll regions needed - just \\u{1b}[row;colH magic!"
}

trap cleanup EXIT
main