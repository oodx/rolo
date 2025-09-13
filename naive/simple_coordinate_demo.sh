#!/bin/bash

# Simple Coordinate Demo - Random colored boxes with coordinates displayed!

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

# Array of your gorgeous colors
COLORS=(
    "$red2" "$red" "$orange" "$yellow" "$green" "$blue" "$blue2"
    "$cyan" "$magenta" "$purple" "$purple2" "$white" "$white2"
)

# Box characters
BOX_CHARS=("█" "▓" "▒" "░" "●" "◆" "○" "▲" "♦" "★")

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Position cursor
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Clear screen and setup
clear_screen() {
    printf "\033[2J\033[?25l"  # Clear screen and hide cursor
}

# Draw a colored box with coordinates
draw_box_with_coords() {
    local coord_x=$1
    local coord_y=$2

    # Pick random color and character
    local color="${COLORS[$((RANDOM % ${#COLORS[@]}))]}"
    local char="${BOX_CHARS[$((RANDOM % ${#BOX_CHARS[@]}))]}"

    # Draw the colored box
    goto $coord_y $coord_x
    printf "%s%s%s" "$color" "$char" "$x"

    # Draw coordinates next to it
    goto $coord_y $((coord_x + 2))
    printf "%s(%d,%d)%s" "$grey" "$coord_x" "$coord_y" "$x"
}

# Draw random boxes across the screen
draw_random_boxes() {
    local count=$1

    for ((i=0; i<count; i++)); do
        # Generate random coordinates (avoid edges)
        local rand_x=$((RANDOM % (TERM_WIDTH - 10) + 3))
        local rand_y=$((RANDOM % (TERM_HEIGHT - 5) + 3))

        draw_box_with_coords "$rand_x" "$rand_y"

        # Small delay to see them appear
        sleep 0.1
    done
}

# Main demo
main() {
    clear_screen

    # Title
    goto 1 1
    printf "%s🎨 Simple Coordinate Demo - Your Color Palette! 🎨%s" "$bld$cyan" "$x"

    # Instructions
    goto 2 1
    printf "%sRandom colored boxes with coordinates shown. Press any key to add more, 'q' to quit.%s" "$grey" "$x"

    # Draw initial set of boxes
    draw_random_boxes 10

    # Interactive loop
    while true; do
        # Position cursor at bottom for input indication
        goto $TERM_HEIGHT 1
        printf "%sPress any key for more boxes, 'q' to quit:%s " "$yellow" "$x"

        # Read single character
        read -n 1 -s key

        if [[ "$key" == "q" ]]; then
            break
        fi

        # Clear the input line
        goto $TERM_HEIGHT 1
        printf "%s" "$eol"

        # Add 5 more random boxes
        draw_random_boxes 5
    done

    # Cleanup
    printf "\033[?25h\033[2J"  # Show cursor and clear screen
    goto 1 1
    printf "%sThanks for trying the coordinate demo!%s\n" "$green" "$x"
}

# Run the demo
main