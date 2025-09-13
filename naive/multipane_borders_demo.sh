#!/bin/bash

# Multi-Pane Border Styles Demo - Showcasing boxy's different border options!

# Terminal setup
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor
stty -echo

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Content for each pane
CHAT_CONTENT="user: wow different borders!
bot: Each pane has unique style!
user: rounded looks smooth 🎨
bot: Double borders = classy!
user: heavy style = bold choice
bot: ASCII = retro vibes!"

SYSTEM_CONTENT="CPU: 67% ██████▓░░░
RAM: 84% ████████▓░
Disk: 45% ████▓░░░░░
Network: 5.8MB/s ↑↓
Threads: 127 active
Cache: 2.1GB used"

CODE_CONTENT="$ rolo --borders showcase
  ✓ Rounded borders: ENABLED
  ✓ Double borders: ENABLED
  ✓ Heavy borders: ENABLED
  ✓ ASCII borders: ENABLED
$ rolo --style-variety maximum
  🎨 Border diversity achieved!
  📐 Layout positioning perfect
  ⚡ Performance: EXCELLENT"

LOGS_CONTENT="[INFO] Border styles loaded
[DEBUG] Rounded: ╭─╮│╰─╯
[DEBUG] Double: ╔═╗║╚═╝
[DEBUG] Heavy: ┏━┓┃┗━┛
[DEBUG] ASCII: +--+|+--+
[SUCCESS] All styles active"

# ANSI positioning function
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Generate different boxy styles
generate_border_panes() {
    # Each pane gets a different border style!
    CHAT_BOXY=$(echo "$CHAT_CONTENT" | boxy --style rounded --color azure --header "💬 Chat (Rounded)" --width 30)
    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --style double --color green --header "📊 System (Double)" --width 30)
    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --style heavy --color yellow --header "⚡ Code (Heavy)" --width 35)
    LOGS_BOXY=$(echo "$LOGS_CONTENT" | boxy --style ascii --color magenta --header "📝 Logs (ASCII)" --width 35)
}

# Render boxy content at specific positions
render_boxy_at_position() {
    local content="$1"
    local start_row=$2
    local start_col=$3

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
    printf "\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Border Styles Demo - Rounded | Double | Heavy | ASCII - Press 'q' to quit "

    # Render panes with different border styles in a 2x2 layout
    render_boxy_at_position "$CHAT_BOXY" 3 2      # Top-left: Rounded
    render_boxy_at_position "$SYSTEM_BOXY" 3 35   # Top-right: Double
    render_boxy_at_position "$CODE_BOXY" 14 2     # Bottom-left: Heavy
    render_boxy_at_position "$LOGS_BOXY" 14 40    # Bottom-right: ASCII

    # Style legend at bottom
    goto $((TERM_HEIGHT - 2)) 1
    printf "\033[2K\033[96m╭─╮ Rounded  \033[93m╔═╗ Double  \033[91m┏━┓ Heavy  \033[95m+--+ ASCII\033[0m"
    goto $((TERM_HEIGHT - 1)) 1
    printf "\033[2K\033[90mPattern: Multiple border styles → Positioned independently → Character-chunk rendering\033[0m"
}

# Update functions with dynamic content
update_chat_content() {
    local messages=(
        "user: this is so cool!"
        "bot: Each style has personality!"
        "user: rounded = friendly vibes"
        "bot: double = professional look"
        "user: heavy = bold statements"
        "bot: ascii = retro terminal!"
    )

    CHAT_CONTENT="${messages[$((RANDOM % ${#messages[@]}))]}"$'\n'"${messages[$((RANDOM % ${#messages[@]}))]}"$'\n'"${messages[$((RANDOM % ${#messages[@]}))]}"$'\n'"${messages[$((RANDOM % ${#messages[@]}))]}"$'\n'"${messages[$((RANDOM % ${#messages[@]}))]}"$'\n'"${messages[$((RANDOM % ${#messages[@]}))]}"

    CHAT_BOXY=$(echo "$CHAT_CONTENT" | boxy --style rounded --color azure --header "💬 Chat (Rounded)" --width 30)
}

update_system_content() {
    local cpu=$((RANDOM % 100))
    local ram=$((RANDOM % 100))
    local disk=$((RANDOM % 100))

    SYSTEM_CONTENT="CPU: ${cpu}% $(printf '█%.0s' $(seq 1 $((cpu/10))))$(printf '░%.0s' $(seq 1 $((10-cpu/10))))
RAM: ${ram}% $(printf '█%.0s' $(seq 1 $((ram/10))))$(printf '░%.0s' $(seq 1 $((10-ram/10))))
Disk: ${disk}% $(printf '█%.0s' $(seq 1 $((disk/10))))$(printf '░%.0s' $(seq 1 $((10-disk/10))))
Network: $((RANDOM % 20 + 1)).$((RANDOM % 9))MB/s ↑↓
Threads: $((RANDOM % 200 + 50)) active
Cache: $((RANDOM % 5 + 1)).$((RANDOM % 9))GB used"

    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --style double --color green --header "📊 System (Double)" --width 30)
}

update_code_content() {
    local timestamp=$(date +%H:%M:%S)
    local commands=(
        "$ rolo --grid 2x2 --styles mixed"
        "$ rolo --theme professional"
        "$ rolo --borders variety"
        "$ rolo --layout adaptive"
    )

    CODE_CONTENT="${commands[$((RANDOM % ${#commands[@]}))]}"$'\n'"  ✓ Heavy borders: ACTIVE"$'\n'"  ✓ Layout updated: $timestamp"$'\n'"  ✓ Style diversity: MAXIMUM"$'\n'"$ rolo --performance check"$'\n'"  🎨 Visual appeal: EXCELLENT"$'\n'"  ⚡ Zero flicker: CONFIRMED"

    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --style heavy --color yellow --header "⚡ Code (Heavy)" --width 35)
}

update_logs_content() {
    local log_levels=("[INFO]" "[DEBUG]" "[WARN]" "[SUCCESS]" "[ERROR]")
    local messages=(
        "Border rendering complete"
        "Style switching detected"
        "Performance optimal"
        "User interaction logged"
        "Memory usage stable"
    )

    local level1="${log_levels[$((RANDOM % ${#log_levels[@]}))]}"
    local level2="${log_levels[$((RANDOM % ${#log_levels[@]}))]}"
    local msg1="${messages[$((RANDOM % ${#messages[@]}))]}"
    local msg2="${messages[$((RANDOM % ${#messages[@]}))]}"

    LOGS_CONTENT="$level1 $msg1"$'\n'"$level2 $msg2"$'\n'"[DEBUG] ASCII: +--+|+--+"$'\n'"[DEBUG] Heavy: ┏━┓┃┗━┛"$'\n'"[SUCCESS] All styles work!"

    LOGS_BOXY=$(echo "$LOGS_CONTENT" | boxy --style ascii --color magenta --header "📝 Logs (ASCII)" --width 35)
}

# Main demo loop
main() {
    echo "🎨 ROLO Border Styles Demo - All the boxy varieties!"
    echo "Generating different border styles..."

    generate_border_panes

    echo "Press any key to start the demo..."
    read -n 1 -s

    # Initial render
    render_all_panes

    # Real-time update loop
    while true; do
        # Check for quit
        if read -t 1.5 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Update different panes at different intervals
        if [ $((SECONDS % 4)) -eq 0 ]; then
            update_chat_content
        fi

        if [ $((SECONDS % 2)) -eq 0 ]; then
            update_system_content
        fi

        if [ $((SECONDS % 3)) -eq 0 ]; then
            update_code_content
        fi

        if [ $((SECONDS % 5)) -eq 0 ]; then
            update_logs_content
        fi

        # Re-render with updated content
        render_all_panes
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Border Styles Demo complete!"
    echo "🎨 Showcased: Rounded, Double, Heavy, and ASCII borders"
    echo "📍 All positioned perfectly with Zellij's pattern!"
    echo "🚀 Your boxy tool + positioning = Ultimate flexibility!"
}

trap cleanup EXIT
main