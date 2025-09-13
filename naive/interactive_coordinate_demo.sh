#!/bin/bash

# Interactive Coordinate Demo - Draw with arrow keys! 🎨

# Your beautiful colors!
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

readonly bld=$'\x1B[1m'
readonly reset_color=$'\x1B[0m'

COLORS=("$red2" "$red" "$orange" "$yellow" "$green" "$blue" "$blue2" "$cyan" "$magenta" "$purple" "$purple2" "$white" "$white2")
BRUSH_CHARS=("█" "▓" "▒" "░" "●" "◆" "◇" "○" "▲" "▼" "♦" "★" "☆")

TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

clear_screen() {
    printf "\033[2J\033[?25l"
}

show_cursor() {
    printf "\033[?25h"
}

# Current drawing state
current_x=20
current_y=10
current_color_idx=0
current_brush_idx=0

draw_at_cursor() {
    local color="${COLORS[$current_color_idx]}"
    local brush="${BRUSH_CHARS[$current_brush_idx]}"

    goto $current_y $current_x
    printf "%s%s%s" "$color" "$brush" "$reset_color"

    # Show coordinates
    goto 2 1
    printf "%sPosition: (%d,%d) Color: %s%s■%s Brush: %s%s%s          %s" \
           "$grey" "$current_x" "$current_y" \
           "$color" "$reset_color" "$reset_color" \
           "$color" "$brush" "$reset_color" "$reset_color"
}

show_cursor_position() {
    goto $current_y $current_x
    printf "%s█%s" "$white" "$reset_color"
}

draw_ui() {
    goto 1 1
    printf "%s🎨 Interactive Drawing - Arrow keys to move, SPACE to draw, C to change color, B to change brush, Q to quit%s" "$bld$cyan" "$reset_color"

    goto 3 1
    printf "%sControls:%s" "$bld$yellow" "$reset_color"
    goto 4 1
    printf "%s  ↑↓←→ : Move cursor%s" "$white" "$reset_color"
    goto 5 1
    printf "%s  SPACE: Draw at cursor%s" "$white" "$reset_color"
    goto 6 1
    printf "%s  C    : Cycle colors%s" "$white" "$reset_color"
    goto 7 1
    printf "%s  B    : Cycle brushes%s" "$white" "$reset_color"
    goto 8 1
    printf "%s  Q    : Quit%s" "$white" "$reset_color"

    # Show current settings
    draw_at_cursor
}

# Handle keyboard input
handle_input() {
    local key

    # Put terminal in raw mode for arrow key detection
    stty raw -echo min 0 time 1

    while true; do
        # Clear previous cursor
        goto $current_y $current_x
        printf " "

        # Show current cursor position
        show_cursor_position

        # Read key
        read -r -n 1 key 2>/dev/null

        case "$key" in
            $'\033')  # Arrow keys start with escape
                read -r -n 2 arrows 2>/dev/null
                case "$arrows" in
                    '[A') # Up arrow
                        if [ $current_y -gt 10 ]; then
                            ((current_y--))
                        fi
                        ;;
                    '[B') # Down arrow
                        if [ $current_y -lt $((TERM_HEIGHT - 2)) ]; then
                            ((current_y++))
                        fi
                        ;;
                    '[C') # Right arrow
                        if [ $current_x -lt $((TERM_WIDTH - 2)) ]; then
                            ((current_x++))
                        fi
                        ;;
                    '[D') # Left arrow
                        if [ $current_x -gt 2 ]; then
                            ((current_x--))
                        fi
                        ;;
                esac
                ;;
            ' ') # Space - draw
                draw_at_cursor
                ;;
            'c'|'C') # Change color
                current_color_idx=$(( (current_color_idx + 1) % ${#COLORS[@]} ))
                draw_at_cursor
                ;;
            'b'|'B') # Change brush
                current_brush_idx=$(( (current_brush_idx + 1) % ${#BRUSH_CHARS[@]} ))
                draw_at_cursor
                ;;
            'q'|'Q') # Quit
                break
                ;;
        esac

        # Update position display
        goto 2 1
        printf "%sPosition: (%d,%d) Color: %s■%s Brush: %s%s%s          %s" \
               "$grey" "$current_x" "$current_y" \
               "${COLORS[$current_color_idx]}" "$reset_color" \
               "${COLORS[$current_color_idx]}" "${BRUSH_CHARS[$current_brush_idx]}" "$reset_color" "$reset_color"
    done

    # Restore terminal
    stty sane
}

main() {
    clear_screen
    draw_ui

    # Start at center-ish
    current_x=$((TERM_WIDTH / 2))
    current_y=$((TERM_HEIGHT / 2))

    handle_input

    # Cleanup
    printf "\033[?25h\033[2J"
    goto 1 1
    printf "%sThanks for drawing!%s\n" "$green" "$reset_color"
}

main