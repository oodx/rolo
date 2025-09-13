#!/bin/bash

# Rolo Scrolling Regions Demo - Using proper ANSI DECSTBM technique
# Shows how to create static header/footer with scrolling content area

# ANSI escape codes
CLEAR_SCREEN="\033[2J"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
RESET_SCROLL="\033[r"

# Colors
HEADER_COLOR="\033[1;44;37m"  # Bold white on blue
FOOTER_COLOR="\033[1;42;30m"  # Bold black on green
CONTENT_COLOR="\033[0m"       # Normal
STATUS_COLOR="\033[1;33m"     # Bold yellow
RESET="\033[0m"

# Layout dimensions
TERMINAL_HEIGHT=$(tput lines)
TERMINAL_WIDTH=$(tput cols)

# Define regions
HEADER_LINES=2
FOOTER_LINES=3
CONTENT_START=$((HEADER_LINES + 1))
CONTENT_END=$((TERMINAL_HEIGHT - FOOTER_LINES))

# Content state
declare -a content_lines=()
content_count=0

# Position cursor at specific row/col
goto() {
    echo -ne "\033[${1};${2}H"
}

# Set scrolling region (DECSTBM)
set_scroll_region() {
    local top=$1
    local bottom=$2
    echo -ne "\033[${top};${bottom}r"
}

# Update static header (outside scroll region)
update_header() {
    local title="$1"
    local info="$2"

    # Save current cursor position
    echo -ne "$SAVE_CURSOR"

    # Update header line 1
    goto 1 1
    printf "${HEADER_COLOR}%-*s${RESET}" $TERMINAL_WIDTH " $title"

    # Update header line 2
    goto 2 1
    printf "${HEADER_COLOR}%-*s${RESET}" $TERMINAL_WIDTH " $info"

    # Restore cursor position
    echo -ne "$RESTORE_CURSOR"
}

# Update static footer (outside scroll region)
update_footer() {
    local status="$1"
    local commands="$2"
    local prompt="$3"

    # Save current cursor position
    echo -ne "$SAVE_CURSOR"

    # Footer line 1 - status
    goto $((TERMINAL_HEIGHT - 2)) 1
    printf "${FOOTER_COLOR}%-*s${RESET}" $TERMINAL_WIDTH " Status: $status"

    # Footer line 2 - commands
    goto $((TERMINAL_HEIGHT - 1)) 1
    printf "${FOOTER_COLOR}%-*s${RESET}" $TERMINAL_WIDTH " $commands"

    # Footer line 3 - input prompt
    goto $TERMINAL_HEIGHT 1
    printf "${FOOTER_COLOR} $prompt: ${RESET}"

    # Restore cursor position
    echo -ne "$RESTORE_CURSOR"
}

# Add content to scrolling region
add_content() {
    local line="$1"

    # Move to scrolling region
    goto $CONTENT_END 1

    # Add the content line (this will cause scrolling in the region)
    echo "$line"

    ((content_count++))
}

# Initialize the layout
init_layout() {
    # Clear screen
    echo -e "$CLEAR_SCREEN$HIDE_CURSOR"

    # Set up scrolling region (content area only)
    set_scroll_region $CONTENT_START $CONTENT_END

    # Draw static header
    update_header "🚀 Rolo Scrolling Regions Demo" "Lines $CONTENT_START-$CONTENT_END scroll, header/footer stay fixed"

    # Draw static footer
    update_footer "Ready" "Commands: add, spam, status, quit" "Enter command"

    # Position cursor in scrolling region
    goto $CONTENT_START 1

    # Show cursor
    echo -e "$SHOW_CURSOR"
}

# Demo different content types
demo_content() {
    add_content "${CONTENT_COLOR}Welcome to scrolling regions demo!"
    sleep 0.5
    add_content "${STATUS_COLOR}This content area scrolls independently${RESET}"
    sleep 0.5
    add_content "${CONTENT_COLOR}Header and footer stay completely static"
    sleep 0.5
    add_content "${STATUS_COLOR}Watch how new lines push older content up${RESET}"
    sleep 0.5
    add_content "${CONTENT_COLOR}But the header/footer never move!"

    # Update status in footer without affecting scroll
    update_footer "Demo running" "Commands: add, spam, status, quit" "Enter command"
}

# Generate spam content
spam_content() {
    update_footer "Generating spam..." "Scroll test in progress" "Please wait"

    for i in {1..20}; do
        add_content "${CONTENT_COLOR}Spam line #$i - demonstrating smooth scrolling behavior"
        sleep 0.1

        # Update status every few lines
        if ((i % 5 == 0)); then
            update_footer "Spam progress: $i/20" "Scrolling test active" "Please wait"
        fi
    done

    update_footer "Spam complete!" "Commands: add, spam, status, quit" "Enter command"
}

# Main interactive loop
main_loop() {
    init_layout
    demo_content

    while true; do
        # Position cursor at input prompt
        goto $TERMINAL_HEIGHT 18

        read -e user_input

        case "$user_input" in
            "add")
                add_content "${CONTENT_COLOR}User added line at $(date +%H:%M:%S)"
                update_footer "Line added" "Commands: add, spam, status, quit" "Enter command"
                ;;
            "spam")
                spam_content
                ;;
            "status")
                update_footer "Lines: $content_count | Region: $CONTENT_START-$CONTENT_END" "Commands: add, spam, status, quit" "Enter command"
                ;;
            "test")
                add_content "${STATUS_COLOR}Testing header update...${RESET}"
                update_header "🧪 UPDATED HEADER!" "Header updated without affecting scroll position"
                sleep 2
                update_header "🚀 Rolo Scrolling Regions Demo" "Lines $CONTENT_START-$CONTENT_END scroll, header/footer stay fixed"
                update_footer "Header test complete" "Commands: add, spam, status, quit" "Enter command"
                ;;
            "clear")
                # Clear content area only
                goto $CONTENT_START 1
                for ((i=CONTENT_START; i<=CONTENT_END; i++)); do
                    goto $i 1
                    printf "%*s" $TERMINAL_WIDTH ""
                done
                goto $CONTENT_START 1
                content_count=0
                add_content "${STATUS_COLOR}Content cleared!${RESET}"
                update_footer "Content cleared" "Commands: add, spam, status, quit" "Enter command"
                ;;
            "quit"|"exit"|"q")
                break
                ;;
            "")
                # Empty input, just reposition
                ;;
            *)
                add_content "${CONTENT_COLOR}You said: '$user_input'"
                update_footer "Message logged" "Commands: add, spam, status, quit" "Enter command"
                ;;
        esac
    done
}

# Cleanup and exit
cleanup() {
    # Reset scrolling region to full screen
    echo -ne "$RESET_SCROLL"

    # Clear screen and show cursor
    echo -e "$CLEAR_SCREEN$SHOW_CURSOR"

    # Position cursor at top
    goto 1 1

    echo "Thanks for trying the scrolling regions demo!"
    echo
    echo "Key insights:"
    echo "• Header/footer stayed completely static"
    echo "• Content scrolled independently in its region"
    echo "• Updates to static areas didn't affect scroll position"
    echo "• This is how nano, vim, tmux manage their layouts!"

    exit 0
}

trap cleanup INT TERM

# Main entry point
main() {
    echo "🚀 Rolo Scrolling Regions Demo"
    echo "============================="
    echo
    echo "This demo shows proper ANSI scrolling region usage:"
    echo "• Header (lines 1-2) - static, never scrolls"
    echo "• Content (lines 3-$CONTENT_END) - scrolling region"
    echo "• Footer (lines $((CONTENT_END+1))-$TERMINAL_HEIGHT) - static, never scrolls"
    echo
    echo "Commands to try:"
    echo "• 'add' - Add a line to content"
    echo "• 'spam' - Generate lots of content"
    echo "• 'test' - Test header updates"
    echo "• 'clear' - Clear content area"
    echo "• 'status' - Show region info"
    echo "• 'quit' - Exit"
    echo
    read -p "Press Enter to start the demo..."

    main_loop
    cleanup
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi