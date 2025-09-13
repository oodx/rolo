#!/bin/bash

# Multi-Pane Boxy Demo - Zellij's positioning + your beautiful boxy styling!

# Terminal setup
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor
stty -echo

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Content for each pane (without manual box drawing)
CHAT_CONTENT="user: hey there!
bot: Hello! How are you?
user: great, testing boxy!
bot: This looks AMAZING!
user: positioning + styling = 🔥
bot: Zellij's pattern rocks!"

SYSTEM_CONTENT="CPU: 45% ████░░░░░░
RAM: 62% ██████░░░░
Network: 1.2MB/s ↑↓
Connections: 42
Uptime: 2h 15m
Load: 0.8, 0.9, 1.1"

CODE_CONTENT="$ cargo build --release
   Compiling rolo v0.1.0 (/home/user/rolo)
    Finished release [optimized] target(s) in 2.45s
$ ./target/release/rolo --multi-pane
  ✓ Layout engine initialized
  ✓ Boxy styling enabled
  ✓ Character-chunk compositor ready"

# ANSI positioning function
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Generate boxy content and store in variables (pre-render)
generate_boxy_panes() {
    # Generate each pane with boxy
    CHAT_BOXY=$(echo "$CHAT_CONTENT" | boxy --color azure --header "Chat Messages" --width 28)
    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --color green --header "System Monitor" --width 28)
    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --color yellow --header "Code Output" --width 50)
}

# Render boxy content at specific positions
render_boxy_at_position() {
    local content="$1"
    local start_row=$2
    local start_col=$3

    # Split boxy output into lines and position each one
    local line_num=0
    while IFS= read -r line; do
        goto $((start_row + line_num)) $start_col
        printf "%s" "$line"
        ((line_num++))
    done <<< "$content"
}

# Main render function
render_all_panes() {
    # Clear screen
    printf "\033[2J"

    # Status bar
    goto 1 1
    printf "\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Boxy Demo - Zellij Positioning + Your Beautiful Styling! Press 'q' to quit "

    # Render each boxy pane at calculated positions
    render_boxy_at_position "$CHAT_BOXY" 3 2
    render_boxy_at_position "$SYSTEM_BOXY" 3 32
    render_boxy_at_position "$CODE_BOXY" 14 2

    # Debug info
    goto $((TERM_HEIGHT - 1)) 1
    printf "\033[2K\033[90mPattern: Boxy styling → Independent positioning → Character-chunk rendering\033[0m"
}

# Update functions with new boxy content
update_chat_content() {
    local new_chat="user: this is amazing!
bot: Boxy + positioning = 🚀
user: no more manual boxes!
bot: Your tools are perfect!
user: rolo architecture ready
bot: Zellij's secret revealed!"

    CHAT_CONTENT="$new_chat"
    CHAT_BOXY=$(echo "$CHAT_CONTENT" | boxy --color azure --header "Chat Messages" --width 28)
}

update_system_content() {
    local cpu=$((RANDOM % 100))
    local ram=$((RANDOM % 100))
    local net_speed=$((RANDOM % 50 + 10))

    SYSTEM_CONTENT="CPU: ${cpu}% $(printf '█%.0s' $(seq 1 $((cpu/10))))$(printf '░%.0s' $(seq 1 $((10-cpu/10))))
RAM: ${ram}% $(printf '█%.0s' $(seq 1 $((ram/10))))$(printf '░%.0s' $(seq 1 $((10-ram/10))))
Network: ${net_speed}.${RANDOM:0:1}MB/s ↑↓
Connections: $((RANDOM % 100 + 20))
Uptime: $(date +%Hh:%Mm)
Load: 0.$((RANDOM%9)), 0.$((RANDOM%9)), 1.$((RANDOM%9))"

    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --color green --header "System Monitor" --width 28)
}

update_code_content() {
    local timestamp=$(date +%H:%M:%S)

    CODE_CONTENT="$ rolo --layout split-vertical
$ rolo --layout grid --panes 4
$ rolo --theme matrix --positioning absolute
$ rolo --demo boxy-integration
  ✓ Boxy styling: ENABLED
  ✓ Positioning updated: $timestamp
  ✓ No flicker detected: SUCCESS"

    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --color yellow --header "Code Output" --width 50)
}

# Main demo loop
main() {
    echo "🚀 ROLO Boxy Demo - Your Beautiful Styling + Zellij's Positioning!"
    echo "Generating boxy content..."

    generate_boxy_panes

    echo "Press any key to start the demo..."
    read -n 1 -s

    # Initial render
    render_all_panes

    # Real-time update loop
    while true; do
        # Check for quit
        if read -t 1 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Update content and regenerate boxy (every few seconds to avoid too much boxy overhead)
        if [ $((SECONDS % 3)) -eq 0 ]; then
            update_system_content
        fi

        if [ $((SECONDS % 4)) -eq 0 ]; then
            update_chat_content
        fi

        if [ $((SECONDS % 2)) -eq 0 ]; then
            update_code_content
        fi

        # Re-render with updated boxy content
        render_all_panes
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Boxy + Positioning Demo complete!"
    echo "🎨 Your styling tools + Zellij's pattern = Perfect combo!"
    echo "📍 Professional layouts with zero flicker!"
}

trap cleanup EXIT
main