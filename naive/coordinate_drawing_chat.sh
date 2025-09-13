#!/bin/bash

# Coordinate Drawing Chat - Zellij-style positioning with interactive drawing!

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
DISABLE_SCROLLBACK="\033[?47h"
ENABLE_SCROLLBACK="\033[?47l"

# Your beautiful color palette! 🎨
readonly red2=$'\x1B[38;5;197m'
readonly red=$'\x1B[31m'
readonly orange=$'\x1B[38;5;214m'
readonly yellow=$'\x1B[33m'
readonly green=$'\x1B[32m'
readonly blue=$'\x1B[36m'
readonly blue2=$'\x1B[38;5;39m'
readonly cyan=$'\x1B[38;5;14m'
readonly magenta=$'\x1B[35m'
readonly purple=$'\x1B[38;5;213m'
readonly purple2=$'\x1B[38;5;141m'
readonly white=$'\x1B[38;5;248m'
readonly white2=$'\x1B[38;5;15m'
readonly grey=$'\x1B[38;5;244m'
readonly grey2=$'\x1B[38;5;240m'

readonly revc=$'\x1B[7m'   # Reverse video
readonly bld=$'\x1B[1m'    # Bold
readonly x=$'\x1B[0m'      # Reset all attributes
readonly xx=$'\x1B[0m'     # Alias for reset
readonly eol=$'\x1B[K'     # Erase to end of line

# Colors for regular chat
USER_COLOR="$cyan"         # Your beautiful cyan
ASSISTANT_COLOR="$green"   # Your green
DIM="$grey"               # Your grey for dim text
BOLD="$bld"               # Your bold
RESET="$x"                # Your reset

# Random colors for drawing - using your gorgeous palette!
COLORS=(
    "$red2" "$red" "$orange" "$yellow" "$green" "$blue" "$blue2"
    "$cyan" "$magenta" "$purple" "$purple2" "$white" "$white2"
)

# Drawing characters
DRAW_CHARS=("█" "▓" "▒" "░" "●" "◆" "◇" "○" "▲" "▼" "♦" "♠" "♣" "♥" "★" "☆")

# Layout dimensions
TERMINAL_HEIGHT=$(tput lines)
TERMINAL_WIDTH=$(tput cols)
CHAT_HEIGHT=$((TERMINAL_HEIGHT - 8))
INPUT_HEIGHT=4

# Protected zones
CHAT_CONTENT_START=3
CHAT_CONTENT_END=$((CHAT_HEIGHT - 2))
INPUT_PROMPT_ROW=$((CHAT_HEIGHT + 3))
BOTTOM_PADDING=1

# Chat state
declare -a chat_messages=()
declare -a chat_types=()  # "user", "assistant", "system"
chat_scroll_offset=0
chat_inner_height=$((CHAT_CONTENT_END - CHAT_CONTENT_START + 1))

# Drawing canvas state
declare -A canvas_points=()  # Store what's drawn at each coordinate

# Position cursor
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Draw a character at specific coordinates with color
draw_at_coordinate() {
    local x=$1
    local y=$2
    local char="$3"
    local color="$4"

    # Bounds checking
    if [ $x -lt 1 ] || [ $x -gt $TERMINAL_WIDTH ] || [ $y -lt 1 ] || [ $y -gt $TERMINAL_HEIGHT ]; then
        return 1
    fi

    # Don't draw in protected chat area
    if [ $y -ge $CHAT_CONTENT_START ] && [ $y -le $CHAT_CONTENT_END ]; then
        return 1
    fi

    # Store what we drew for persistence
    canvas_points["$x,$y"]="$color$char$RESET"

    # Draw it
    goto $y $x
    printf "%s%s%s" "$color" "$char" "$RESET"

    return 0
}

# Redraw all canvas points (for screen refreshes)
redraw_canvas() {
    for coord in "${!canvas_points[@]}"; do
        local x="${coord%,*}"
        local y="${coord#*,}"
        local content="${canvas_points[$coord]}"

        goto $y $x
        printf "%s" "$content"
    done
}

# Create boxy system message
create_system_boxy() {
    local message="$1"
    local theme="${2:-warning}"
    local header="${3:-⚙️ System}"

    printf "%s" "$message" | boxy --color "$theme" --header "$header" --width max --layout "hl"
}

# Draw basic layout
draw_layout() {
    printf "%s%s%s" "$DISABLE_SCROLLBACK" "$CLEAR_SCREEN" "$HIDE_CURSOR"

    # Title
    goto 1 1
    printf "%s🎨 Coordinate Drawing Chat - Enter 'x,y' to draw!%s" "$BOLD" "$RESET"

    # Basic separator
    goto $((CHAT_HEIGHT + 1)) 1
    for ((i=1; i<=TERMINAL_WIDTH; i++)); do printf "─"; done

    # Input prompt placeholder
    goto $INPUT_PROMPT_ROW 5
    printf "%sType a message or 'x,y' coordinates to draw (e.g. '10,5')...%s" "$DIM" "$RESET"
}

# Add regular chat message
add_chat_message() {
    local sender="$1"
    local message="$2"
    local type="${3:-user}"

    chat_messages+=("${sender}: ${message}")
    chat_types+=("$type")

    # Auto-scroll
    if ((${#chat_messages[@]} > chat_inner_height)); then
        chat_scroll_offset=$(( ${#chat_messages[@]} - chat_inner_height ))
    fi

    refresh_chat_area
}

# Add system message with boxy styling
add_system_message() {
    local message="$1"
    local theme="${2:-amber}"
    local header="${3:-⚙️ System}"

    # Generate the boxy message
    local boxy_output=$(create_system_boxy "$message" "$theme" "$header")

    # Find available space in chat area
    local current_row=$((CHAT_CONTENT_END - 3))

    # Position and display the boxy message
    goto $current_row 1
    printf "%s" "$boxy_output"

    # Add spacer after boxy message
    goto $((current_row + 4)) 1
    printf "\n"
}

# Refresh regular chat area
refresh_chat_area() {
    # Clear regular chat content
    for ((i=CHAT_CONTENT_START; i<=$((CHAT_CONTENT_END - 5)); i++)); do
        goto $i 2
        printf "%s" "$eol"  # Use your eol instead of CLEAR_LINE
    done

    refresh_chat_top_down
}

# Top-down chat rendering
refresh_chat_top_down() {
    local start_msg=$chat_scroll_offset
    local end_msg=$((start_msg + chat_inner_height))

    local row=$CHAT_CONTENT_START
    for ((i=start_msg; i<end_msg && i<${#chat_messages[@]} && row <= $((CHAT_CONTENT_END - 5)); i++)); do
        if [[ $i -ge 0 ]]; then
            local msg="${chat_messages[i]}"
            local type="${chat_types[i]}"

            # Skip system messages
            if [[ "$type" != "system" ]]; then
                goto $row 3

                if [[ "$msg" =~ ^You: ]]; then
                    printf "%s%s%s" "$USER_COLOR" "$msg" "$RESET"
                else
                    printf "%s%s%s" "$ASSISTANT_COLOR" "$msg" "$RESET"
                fi
            fi
        fi
        ((row++))
    done

    # Redraw any canvas points that might have been overwritten
    redraw_canvas
}

# Clear input area
clear_input() {
    goto $INPUT_PROMPT_ROW 5
    printf "%s" "$eol"  # Use your eol
    goto $INPUT_PROMPT_ROW 5
    printf "%sYou:%s " "$BOLD" "$RESET"
}

# Parse coordinate input (x,y format)
parse_coordinates() {
    local input="$1"

    # Check if input matches x,y pattern
    if [[ "$input" =~ ^[0-9]+,[0-9]+$ ]]; then
        local x="${input%,*}"
        local y="${input#*,}"
        echo "$x $y"
        return 0
    fi

    return 1
}

# Handle drawing command
handle_drawing() {
    local x=$1
    local y=$2

    # Pick random color and character
    local color="${COLORS[$((RANDOM % ${#COLORS[@]}))]}"
    local char="${DRAW_CHARS[$((RANDOM % ${#DRAW_CHARS[@]}))]}"

    if draw_at_coordinate "$x" "$y" "$char" "$color"; then
        add_system_message "Drew ${char} at coordinates ($x, $y)" "green" "🎨 Draw"
        return 0
    else
        add_system_message "Can't draw at ($x, $y) - out of bounds or protected area" "crimson" "❌ Error"
        return 1
    fi
}

# Enhanced slash commands
handle_slash_command() {
    local cmd="$1"
    case "$cmd" in
        "/quit"|"/exit"|"/q")
            add_system_message "Chat session ending..." "crimson" "👋 Goodbye"
            sleep 1
            cleanup
            ;;
        "/clear"|"/cls")
            chat_messages=()
            chat_types=()
            chat_scroll_offset=0
            canvas_points=()  # Clear canvas too
            draw_layout
            refresh_chat_area
            add_system_message "Chat history and canvas cleared" "green" "✅ Success"
            ;;
        "/help"|"/h")
            add_system_message "Commands: /clear, /canvas, /quit | Coordinates: 'x,y' (e.g. '25,10')" "azure" "📖 Help"
            ;;
        "/canvas")
            local count=${#canvas_points[@]}
            add_system_message "Canvas has $count drawn points. Try coordinates like '15,8' or '50,20'!" "azure" "🎨 Canvas"
            ;;
        *)
            add_system_message "Unknown command: $cmd (try /help)" "crimson" "❌ Error"
            ;;
    esac
}

# Main chat loop
chat_loop() {
    draw_layout

    # Welcome messages
    add_system_message "Welcome to Coordinate Drawing Chat!" "green" "🚀 Welcome"
    sleep 0.5
    add_system_message "Type coordinates like '25,10' to draw random colored shapes!" "azure" "💡 Info"
    sleep 0.5

    # Add some regular chat
    add_chat_message "Assistant" "Try entering coordinates like '30,15' to draw!" "assistant"
    add_chat_message "Assistant" "Each coordinate gets a random color and character!" "assistant"

    local input_row=$INPUT_PROMPT_ROW
    printf "%s" "$SHOW_CURSOR"
    goto $input_row 10

    while true; do
        goto $input_row 5
        printf "%sYou:%s " "$BOLD" "$RESET"
        read -e user_input

        if [[ -z "$user_input" ]]; then
            clear_input
            continue
        fi

        if [[ "$user_input" =~ ^/ ]]; then
            handle_slash_command "$user_input"
            clear_input
            continue
        fi

        # Check if it's coordinates
        if coords=$(parse_coordinates "$user_input"); then
            read -r x y <<< "$coords"
            handle_drawing "$x" "$y"
            clear_input
            continue
        fi

        # Regular user message
        add_chat_message "You" "$user_input" "user"
        clear_input

        # Simulate assistant response
        sleep 0.3
        local responses=(
            "Nice! Try some coordinates like '20,12' to draw!"
            "The coordinate system is fun! Try '40,8'!"
            "Each point gets random colors and shapes!"
            "Drawing at different coordinates is addictive!"
            "The canvas persists between chat messages!"
            "Try coordinates outside the chat area!"
        )

        local response="${responses[$((RANDOM % ${#responses[@]}))]}"
        add_chat_message "Assistant" "$response" "assistant"

        goto $input_row 10
    done
}

# Cleanup
cleanup() {
    printf "%s%s%s" "$ENABLE_SCROLLBACK" "$SHOW_CURSOR" "$CLEAR_SCREEN"
    goto 1 1
    printf "Thanks for trying Coordinate Drawing Chat!\n"
    exit 0
}

trap cleanup INT TERM

# Main entry
main() {
    echo "🎨 Coordinate Drawing Chat Demo"
    echo "==============================="
    echo
    echo "Features:"
    echo "• Chat with coordinate-based drawing"
    echo "• Enter 'x,y' coordinates to draw colored shapes"
    echo "• Random colors and characters at each point"
    echo "• Canvas persists during chat"
    echo
    echo "Try coordinates like: 25,10 or 50,15"
    echo
    read -p "Press Enter to start..."

    chat_loop
    cleanup
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi