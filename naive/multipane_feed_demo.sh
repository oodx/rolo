#!/bin/bash

# Multi-Pane Live Feed Demo - One pane becomes a streaming feed!

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

# Generate boxy panes
generate_panes() {
    # Create feed content from buffer
    local feed_content=""
    for message in "${FEED_BUFFER[@]}"; do
        feed_content+="$message"$'\n'
    done

    # Generate panes with different styles
    FEED_BOXY=$(echo -n "$feed_content" | boxy --style rounded --color cyan --header "📡 Live Feed (Streaming)" --width 45)
    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --style double --color green --header "📊 System Monitor" --width 30)
    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --style heavy --color yellow --header "⚡ Feed Control" --width 35)
    LOGS_BOXY=$(echo "$LOGS_CONTENT" | boxy --style ascii --color magenta --header "📝 Feed Logs" --width 35)
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
    printf "\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Live Feed Demo - Streaming Messages! Buffer: ${#FEED_BUFFER[@]}/$FEED_SIZE - Press 'q' to quit "

    # Render panes
    render_boxy_at_position "$FEED_BOXY" 3 2      # Main feed (left side)
    render_boxy_at_position "$SYSTEM_BOXY" 3 50   # System monitor (top right)
    render_boxy_at_position "$CODE_BOXY" 14 2     # Feed control (bottom left)
    render_boxy_at_position "$LOGS_BOXY" 14 40    # Logs (bottom right)

    # Feed info at bottom
    goto $((TERM_HEIGHT - 2)) 1
    printf "\033[2K\033[96m📡 Live Feed: ${#FEED_BUFFER[@]} messages  🔄 Auto-scroll: ON  ⚡ Updates: Every 2s\033[0m"
    goto $((TERM_HEIGHT - 1)) 1
    printf "\033[2K\033[90mPattern: Streaming content → Rolling buffer → Real-time positioning\033[0m"
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

    SYSTEM_BOXY=$(echo "$SYSTEM_CONTENT" | boxy --style double --color green --header "📊 System Monitor" --width 30)
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
  ⚡ Performance: OPTIMAL"

    CODE_BOXY=$(echo "$CODE_CONTENT" | boxy --style heavy --color yellow --header "⚡ Feed Control" --width 35)
}

update_logs() {
    local timestamp=$(date +%H:%M:%S)

    LOGS_CONTENT="[FEED] New message added
[DEBUG] Buffer size: ${#FEED_BUFFER[@]}
[INFO] Scroll update: $timestamp
[DEBUG] Render cycle complete
[SUCCESS] Feed flowing smoothly!"

    LOGS_BOXY=$(echo "$LOGS_CONTENT" | boxy --style ascii --color magenta --header "📝 Feed Logs" --width 35)
}

# Main demo loop
main() {
    echo "📡 ROLO Live Feed Demo - Real streaming simulation!"
    echo "Initializing feed buffer..."

    # Initialize with a few messages
    for i in {1..3}; do
        add_to_feed
    done

    generate_panes

    echo "Press any key to start the live feed..."
    read -n 1 -s

    # Initial render
    render_all_panes

    local last_feed_update=0

    # Real-time streaming loop
    while true; do
        local current_time=$SECONDS

        # Check for quit
        if read -t 0.1 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Add new feed message every 2 seconds (like a real feed!)
        if [ $((current_time - last_feed_update)) -ge 2 ]; then
            add_to_feed
            last_feed_update=$current_time
            # Regenerate feed pane with new content
            local feed_content=""
            for message in "${FEED_BUFFER[@]}"; do
                feed_content+="$message"$'\n'
            done
            FEED_BOXY=$(echo -n "$feed_content" | boxy --style rounded --color cyan --header "📡 Live Feed (Streaming)" --width 45)
        fi

        # Update other panes at different intervals
        if [ $((current_time % 3)) -eq 0 ]; then
            update_system
        fi

        if [ $((current_time % 4)) -eq 0 ]; then
            update_code_control
        fi

        if [ $((current_time % 5)) -eq 0 ]; then
            update_logs
        fi

        # Re-render everything
        render_all_panes

        sleep 0.5  # Smooth updates
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Live Feed Demo complete!"
    echo "📡 Streamed ${#FEED_BUFFER[@]} messages in the feed buffer"
    echo "🔄 Auto-scrolling feed simulation working perfectly!"
    echo "⚡ Real-time updates with character-chunk positioning!"
}

trap cleanup EXIT
main