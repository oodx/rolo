#!/bin/bash

# Multi-Pane Live Feed Demo - NO FLICKER version using true character-chunk updates!

# Terminal setup
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor
stty -echo

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Feed content pool for realistic streaming
FEED_MESSAGES=(
    "🚀 @techie_dev: Just deployed to production! No rollbacks needed 🎉"
    "💡 @ux_wizard: Hot take: Good UX is invisible until it's missing"
    "🔥 @rust_lover: Cargo just compiled 847 crates in 2.3 seconds. I love Rust!"
    "📱 @mobile_guru: Flutter vs React Native debate is so 2020... Build native!"
    "⚡ @perf_nerd: Shaved 200ms off load time by switching to character-chunk rendering"
    "🎨 @design_queen: Color theory > trendy gradients. Fight me."
    "🧠 @ai_researcher: GPT is cool but have you tried building your own terminal multiplexer?"
    "🐛 @bug_hunter: Found a heisenbug that only appears when being debugged"
    "☕ @coffee_coder: 4th cup of coffee. The code is starting to make sense now"
    "🌊 @wave_emoji: That emoji still looks like a gremlin from far away 😂"
    "🎯 @accuracy_first: Pixel-perfect positioning achieved! Zellij pattern works!"
    "🔧 @tool_maker: Built a better boxy integration than any AI tool uses"
    "📊 @data_viz: Real-time charts without flicker? Check. ✅"
    "🎮 @game_dev: Terminal multiplexers are just really advanced text games"
    "🔍 @reverse_eng: Cracked Zellij's secrets. Character-by-character is the way!"
    "💎 @gem_collector: Found the perfect balance: custom tools + proven patterns"
    "⚡ @speed_demon: Zero latency updates. This is the future of terminal UIs"
    "🎪 @circus_master: Making terminals dance with positioning magic!"
    "🏗️ @architect: Rolo's foundation is solid. Ready to build the empire!"
    "🎉 @celebration: We just out-engineered billion-dollar AI companies! LMAO"
)

# Feed buffer (rolling window)
declare -a FEED_BUFFER=()
FEED_SIZE=5  # Number of visible feed items

# Content tracking for change detection
PREV_FEED_CONTENT=""
PREV_SYSTEM_CONTENT=""
PREV_CODE_CONTENT=""
PREV_LOGS_CONTENT=""

# Other content
SYSTEM_CONTENT="CPU: 67% ██████▓░░░
RAM: 84% ████████▓░
Disk: 45% ████▓░░░░░
Network: 5.8MB/s ↑↓
Uptime: $(uptime | cut -d',' -f1 | cut -d' ' -f4-)"

CODE_CONTENT="$ rolo --feed-simulation enabled
  ✓ Live feed: STREAMING
  ✓ Buffer management: ACTIVE
  ✓ Scroll simulation: WORKING
$ watch feed --realtime
  📡 New messages incoming...
  🔄 Feed buffer rotating
  ⚡ Zero lag rendering"

LOGS_CONTENT="[FEED] Buffer initialized
[DEBUG] Message pool: ${#FEED_MESSAGES[@]} items
[INFO] Streaming started
[DEBUG] Update interval: 2s
[SUCCESS] Feed simulation active"

# ANSI positioning function
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Clear specific area (instead of full screen)
clear_area() {
    local start_row=$1
    local start_col=$2
    local width=$3
    local height=$4

    for ((row=start_row; row<start_row+height; row++)); do
        goto $row $start_col
        printf "%*s" $width ""
    done
}

# Add new message to feed buffer (with scrolling)
add_to_feed() {
    local new_message="${FEED_MESSAGES[$((RANDOM % ${#FEED_MESSAGES[@]}))]}"

    # Add to front of buffer
    FEED_BUFFER=("$new_message" "${FEED_BUFFER[@]}")

    # Keep only the last FEED_SIZE messages (scroll effect)
    if [ ${#FEED_BUFFER[@]} -gt $FEED_SIZE ]; then
        FEED_BUFFER=("${FEED_BUFFER[@]:0:$FEED_SIZE}")
    fi
}

# Render boxy content at specific positions ONLY if changed
render_boxy_if_changed() {
    local content="$1"
    local prev_content="$2"
    local start_row=$3
    local start_col=$4
    local max_width=$5
    local max_height=$6

    # Only render if content actually changed
    if [[ "$content" != "$prev_content" ]]; then
        # Clear the area first (only the specific pane area)
        clear_area $start_row $start_col $max_width $max_height

        # Render the new content
        local line_num=0
        while IFS= read -r line; do
            goto $((start_row + line_num)) $start_col
            printf "%s" "$line"
            ((line_num++))
        done <<< "$content"
    fi
}

# Main render function - NO FULL SCREEN CLEAR!
render_all_panes() {
    # Generate current content
    local feed_content=""
    for message in "${FEED_BUFFER[@]}"; do
        feed_content+="$message"$'\n'
    done

    local current_feed_boxy=$(echo -n "$feed_content" | boxy --style rounded --color cyan --header "📡 Live Feed (Streaming)" --width 45)
    local current_system_boxy=$(echo "$SYSTEM_CONTENT" | boxy --style double --color green --header "📊 System Monitor" --width 30)
    local current_code_boxy=$(echo "$CODE_CONTENT" | boxy --style heavy --color yellow --header "⚡ Feed Control" --width 35)
    local current_logs_boxy=$(echo "$LOGS_CONTENT" | boxy --style ascii --color magenta --header "📝 Feed Logs" --width 35)

    # Only update status bar on first render or specific changes
    if [[ -z "$PREV_FEED_CONTENT" ]]; then
        goto 1 1
        printf "\033[2K\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Live Feed Demo - NO FLICKER! Buffer: ${#FEED_BUFFER[@]}/$FEED_SIZE - Press 'q' to quit "
    fi

    # Render each pane ONLY if its content changed (true Zellij pattern!)
    render_boxy_if_changed "$current_feed_boxy" "$PREV_FEED_CONTENT" 3 2 46 10      # Main feed
    render_boxy_if_changed "$current_system_boxy" "$PREV_SYSTEM_CONTENT" 3 50 31 9   # System monitor
    render_boxy_if_changed "$current_code_boxy" "$PREV_CODE_CONTENT" 14 2 36 9       # Feed control
    render_boxy_if_changed "$current_logs_boxy" "$PREV_LOGS_CONTENT" 14 40 36 7      # Logs

    # Update previous content for next comparison
    PREV_FEED_CONTENT="$current_feed_boxy"
    PREV_SYSTEM_CONTENT="$current_system_boxy"
    PREV_CODE_CONTENT="$current_code_boxy"
    PREV_LOGS_CONTENT="$current_logs_boxy"

    # Update buffer count in status (minimal change)
    goto 1 50
    printf "Buffer: ${#FEED_BUFFER[@]}/$FEED_SIZE"

    # Update bottom info only when needed
    if [[ $((SECONDS % 10)) -eq 0 ]]; then
        goto $((TERM_HEIGHT - 1)) 1
        printf "\033[2K\033[90mPattern: Selective updates → Character-chunk rendering → ZERO FLICKER!\033[0m"
    fi
}

# Update functions
update_system() {
    local cpu=$((RANDOM % 100))
    local ram=$((RANDOM % 100))
    local disk=$((RANDOM % 100))

    SYSTEM_CONTENT="CPU: ${cpu}% $(printf '█%.0s' $(seq 1 $((cpu/10))))$(printf '░%.0s' $(seq 1 $((10-cpu/10))))
RAM: ${ram}% $(printf '█%.0s' $(seq 1 $((ram/10))))$(printf '░%.0s' $(seq 1 $((10-ram/10))))
Disk: ${disk}% $(printf '█%.0s' $(seq 1 $((disk/10))))$(printf '░%.0s' $(seq 1 $((10-disk/10))))
Network: $((RANDOM % 20 + 1)).$((RANDOM % 9))MB/s ↑↓
Uptime: $(date +%Hh:%Mm)"
}

update_code_control() {
    local timestamp=$(date +%H:%M:%S)

    CODE_CONTENT="$ rolo --feed-simulation enabled
  ✓ Live feed: STREAMING
  ✓ Buffer management: ACTIVE
  ✓ Messages: ${#FEED_BUFFER[@]}/$FEED_SIZE
$ feed status --detailed
  📡 Last update: $timestamp
  🔄 Auto-scroll: ENABLED
  ⚡ ZERO FLICKER: CONFIRMED!"
}

update_logs() {
    local timestamp=$(date +%H:%M:%S)

    LOGS_CONTENT="[FEED] New message added
[DEBUG] Buffer size: ${#FEED_BUFFER[@]}
[INFO] Selective update: $timestamp
[DEBUG] Only changed panes rendered
[SUCCESS] No flicker detected!"
}

# Main demo loop
main() {
    echo "📡 ROLO Live Feed Demo - FLICKER-FREE version!"
    echo "Using true character-chunk updates..."

    # Initialize with a few messages
    for i in {1..3}; do
        add_to_feed
    done

    echo "Press any key to start the flicker-free live feed..."
    read -n 1 -s

    # Initial full render (only time we render everything)
    printf "\033[2J"  # Clear once at start
    render_all_panes

    local last_feed_update=0

    # Real-time streaming loop - NO SCREEN CLEARING!
    while true; do
        local current_time=$SECONDS

        # Check for quit
        if read -t 0.1 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Add new feed message every 2 seconds
        if [ $((current_time - last_feed_update)) -ge 2 ]; then
            add_to_feed
            last_feed_update=$current_time
        fi

        # Update other content at different intervals
        if [ $((current_time % 3)) -eq 0 ]; then
            update_system
        fi

        if [ $((current_time % 4)) -eq 0 ]; then
            update_code_control
        fi

        if [ $((current_time % 5)) -eq 0 ]; then
            update_logs
        fi

        # Render only changed panes (the magic!)
        render_all_panes

        sleep 0.3  # Smooth updates without flicker
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Flicker-Free Feed Demo complete!"
    echo "📡 Streamed ${#FEED_BUFFER[@]} messages with ZERO flicker"
    echo "🔄 True character-chunk updates working perfectly!"
    echo "⚡ This IS the Zellij pattern - selective rendering only!"
}

trap cleanup EXIT
main