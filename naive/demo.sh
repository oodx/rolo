#!/bin/bash

# Rolo Terminal Layout Engine - Bash Capabilities Demo
# Demonstrates flicker-free terminal techniques that could inspire rolo's design

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"

# Colors
RED="\033[31m"
GREEN="\033[32m"
BLUE="\033[34m"
YELLOW="\033[33m"
RESET="\033[0m"

# Position cursor at row, col
goto() {
    echo -ne "\033[${1};${2}H"
}

# Draw a box at position with width/height
draw_box() {
    local row=$1 col=$2 width=$3 height=$4 title=$5
    
    # Top border
    goto $row $col
    echo -n "┌"
    for ((i=1; i<width-1; i++)); do echo -n "─"; done
    echo -n "┐"
    
    # Title if provided
    if [[ -n "$title" ]]; then
        goto $row $((col + 2))
        echo -n "[$title]"
    fi
    
    # Side borders
    for ((i=1; i<height-1; i++)); do
        goto $((row + i)) $col
        echo -n "│"
        goto $((row + i)) $((col + width - 1))
        echo -n "│"
    done
    
    # Bottom border
    goto $((row + height - 1)) $col
    echo -n "└"
    for ((i=1; i<width-1; i++)); do echo -n "─"; done
    echo -n "┘"
}

# Update text at specific position without redraw
update_text() {
    local row=$1 col=$2 text=$3
    echo -ne "$SAVE_CURSOR"
    goto $row $col
    echo -ne "$CLEAR_LINE"
    echo -n "$text"
    echo -ne "$RESTORE_CURSOR"
}

# Demo 1: Static Layout with No Flicker
demo_static_layout() {
    echo -e "$CLEAR_SCREEN$HIDE_CURSOR"
    
    # Draw static layout
    draw_box 2 5 40 8 "Status Panel"
    draw_box 2 50 30 8 "Menu"
    draw_box 12 5 75 10 "Main Content"
    
    # Add content to boxes
    goto 4 7
    echo -n "System: ${GREEN}Online${RESET}"
    goto 5 7
    echo -n "CPU: 45%"
    goto 6 7
    echo -n "Memory: 2.1GB"
    
    # Menu items
    goto 4 52
    echo -n "1. New File"
    goto 5 52
    echo -n "2. Open File"
    goto 6 52
    echo -n "3. Settings"
    goto 7 52
    echo -n "4. Exit"
    
    # Main content
    goto 14 7
    echo -n "Welcome to Rolo Layout Demo"
    goto 15 7
    echo -n "This layout was drawn once and uses selective updates"
    
    # Position cursor for input
    goto 25 1
    echo -e "$SHOW_CURSOR"
}

# Demo 2: Live Updates Without Redraw
demo_live_updates() {
    echo "Starting live update demo (press Ctrl+C to stop)..."
    sleep 2
    
    demo_static_layout
    
    local counter=0
    while true; do
        # Update CPU percentage
        cpu=$((RANDOM % 100))
        update_text 6 7 "CPU: ${cpu}%"
        
        # Update counter in main area
        update_text 16 7 "Updates: $counter (no flicker!)"
        
        # Color-coded status
        if ((cpu > 80)); then
            update_text 4 7 "System: ${RED}High Load${RESET}"
        elif ((cpu > 50)); then
            update_text 4 7 "System: ${YELLOW}Medium Load${RESET}"
        else
            update_text 4 7 "System: ${GREEN}Normal${RESET}"
        fi
        
        ((counter++))
        sleep 0.5
    done
}

# Demo 3: Interactive Menu (bash select-style)
demo_interactive_menu() {
    echo -e "$CLEAR_SCREEN"
    
    draw_box 5 20 40 12 "Interactive Menu"
    
    local options=("Show Static Layout" "Live Updates Demo" "Scrolling Region Demo" "Exit")
    local selected=0
    local key
    
    while true; do
        # Draw menu options
        for i in "${!options[@]}"; do
            goto $((7 + i)) 22
            if [[ $i -eq $selected ]]; then
                echo -ne "${GREEN}> ${options[i]}${RESET}"
            else
                echo -ne "  ${options[i]}"
            fi
            echo -ne "$CLEAR_LINE"
        done
        
        # Get key input
        read -rsn1 key
        case "$key" in
            A) # Up arrow (after escape sequence)
                ((selected > 0)) && ((selected--))
                ;;
            B) # Down arrow  
                ((selected < ${#options[@]} - 1)) && ((selected++))
                ;;
            "") # Enter key
                case $selected in
                    0) demo_static_layout; read -p "Press Enter to continue..."; break;;
                    1) demo_live_updates;;
                    2) demo_scrolling_region; break;;
                    3) echo -e "$CLEAR_SCREEN"; exit 0;;
                esac
                ;;
        esac
    done
}

# Demo 4: Scrolling Region (like tmux panes)
demo_scrolling_region() {
    echo -e "$CLEAR_SCREEN"
    
    draw_box 2 5 70 20 "Scrolling Region Demo"
    
    # Set scrolling region inside the box
    echo -ne "\033[4;19r"  # Set scroll region from line 4 to 19
    
    goto 4 7
    echo "This demonstrates a scrolling region within a static layout:"
    echo
    
    for i in {1..30}; do
        echo "  Line $i - This content scrolls while borders stay static"
        sleep 0.2
    done
    
    # Reset scrolling region
    echo -ne "\033[r"
    
    goto 25 1
    read -p "Press Enter to continue..."
}

# Main menu
main() {
    echo "Rolo Terminal Layout Engine - Bash Techniques Demo"
    echo "=================================================="
    echo
    echo "This demo shows terminal techniques that could inspire rolo's design:"
    echo "• Static positioning without full redraws"
    echo "• Selective updates to prevent flicker"
    echo "• Interactive menus with cursor movement"
    echo "• Scrolling regions within static layouts"
    echo
    echo "Use arrow keys to navigate, Enter to select"
    echo
    
    demo_interactive_menu
}

# Cleanup on exit
trap 'echo -e "\033[r\033[?25h\033[2J\033[H"; exit' INT TERM

# Check if running directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi