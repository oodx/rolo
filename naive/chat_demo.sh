#!/bin/bash

# Rolo Chat Terminal Demo
# Shows a Claude-style chat interface with flicker-free updates

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
CLEAR_TO_END="\033[J"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
DISABLE_SCROLLBACK="\033[?47h"    # Use alternate screen buffer (no scrollback)
ENABLE_SCROLLBACK="\033[?47l"     # Return to main screen buffer

# Colors
USER_COLOR="\033[36m"    # Cyan for user
ASSISTANT_COLOR="\033[32m" # Green for assistant
STATUS_COLOR="\033[33m"   # Yellow for status
DIM="\033[2m"
BOLD="\033[1m"
RESET="\033[0m"

# Layout dimensions
TERMINAL_HEIGHT=$(tput lines)
TERMINAL_WIDTH=$(tput cols)
CHAT_HEIGHT=$((TERMINAL_HEIGHT - 6))  # Leave room for input area
INPUT_HEIGHT=4
STATUS_HEIGHT=2

# Protected zones - hard boundaries that content cannot cross
CHAT_CONTENT_START=3                    # First row where chat content can appear
CHAT_CONTENT_END=$((CHAT_HEIGHT - 2))   # Last row where chat content can appear (leave padding)
INPUT_AREA_START=$((CHAT_HEIGHT + 3))   # First row of input area (with padding)
INPUT_PROMPT_ROW=$((CHAT_HEIGHT + 3))   # Row where actual prompt appears (with top padding)
PROTECTED_SEPARATOR=$((CHAT_HEIGHT + 1)) # Separator row - never overwrite
BOTTOM_PADDING=1                        # Rows to leave empty above input in bottom mode

# Chat orientation (default: top-down, set CHAT_ORIENTATION=bottom for bottom-up)
CHAT_ORIENTATION=${CHAT_ORIENTATION:-top}

# Chat state
declare -a chat_messages=()
declare -a chat_colors=()
chat_scroll_offset=0
chat_inner_height=$((CHAT_CONTENT_END - CHAT_CONTENT_START + 1))  # Available rows for content

# Position cursor at row, col
goto() {
    echo -ne "\033[${1};${2}H"
}

# Draw the static layout
draw_layout() {
    echo -e "$DISABLE_SCROLLBACK$CLEAR_SCREEN$HIDE_CURSOR"
    
    # Chat area border
    goto 1 1
    echo -n "┌"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "─"; done
    echo -n "┐"
    
    # Chat area title
    goto 1 3
    echo -n "[ ${BOLD}Rolo Chat Terminal${RESET} ]"
    
    # Side borders for chat area
    for ((i=2; i<=CHAT_HEIGHT; i++)); do
        goto $i 1
        echo -n "│"
        goto $i $TERMINAL_WIDTH
        echo -n "│"
    done
    
    # Separator between chat and input
    local sep_row=$((CHAT_HEIGHT + 1))
    goto $sep_row 1
    echo -n "├"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "─"; done
    echo -n "┤"
    
    # Status area
    goto $sep_row 3
    echo -n "[ Status ]"
    
    # Input area borders with padding
    for ((i=$((sep_row + 1)); i<=$((sep_row + INPUT_HEIGHT - 1)); i++)); do
        goto $i 1
        echo -n "│"
        goto $i $TERMINAL_WIDTH
        echo -n "│"
    done
    
    # Bottom border
    local bottom_row=$((sep_row + INPUT_HEIGHT - 1))
    goto $bottom_row 1
    echo -n "└"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "─"; done
    echo -n "┘"
    
    # Input area with padding - empty row first
    goto $((sep_row + 2)) 3
    echo -n "${DIM}${RESET}"  # Empty padded row
    
    # Actual input prompt with left padding
    goto $((sep_row + 3)) 5
    echo -n "${DIM}Type a message or /help for commands...${RESET}"
}

# Update status bar
update_status() {
    local status_text="$1"
    local status_row=$((CHAT_HEIGHT + 1))
    
    echo -ne "$SAVE_CURSOR"
    goto $status_row $((TERMINAL_WIDTH - ${#status_text} - 2))
    echo -ne "$CLEAR_LINE"
    echo -ne "${STATUS_COLOR}${status_text}${RESET}"
    echo -ne "$RESTORE_CURSOR"
}

# Add message to chat
add_message() {
    local sender="$1"
    local message="$2"
    local color="$3"
    
    chat_messages+=("${sender}: ${message}")
    chat_colors+=("$color")
    
    # Auto-scroll behavior: ALWAYS show latest messages in both orientations
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        # Bottom-up: reset to show latest (offset 0 = newest at bottom)
        chat_scroll_offset=0
    else
        # Top-down: traditional auto-scroll to show latest
        if ((${#chat_messages[@]} > chat_inner_height)); then
            chat_scroll_offset=$(( ${#chat_messages[@]} - chat_inner_height ))
        fi
    fi
    
    refresh_chat_area
}

# Refresh the entire chat area (still more efficient than full redraw)
refresh_chat_area() {
    # Clear ONLY the protected chat content area
    for ((i=CHAT_CONTENT_START; i<=CHAT_CONTENT_END; i++)); do
        goto $i 2
        echo -ne "$CLEAR_LINE"
        goto $i $((TERMINAL_WIDTH - 1))
    done
    
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        refresh_chat_bottom_up
    else
        refresh_chat_top_down
    fi
}

# Bottom-up chat rendering (newest at bottom)
refresh_chat_bottom_up() {
    local total_messages=${#chat_messages[@]}
    local visible_start=$((total_messages - chat_inner_height + chat_scroll_offset))
    local visible_end=$((total_messages + chat_scroll_offset))
    
    # Ensure bounds
    if ((visible_start < 0)); then
        visible_start=0
    fi
    if ((visible_end > total_messages)); then
        visible_end=$total_messages
    fi
    
    # Calculate effective bottom boundary with padding for visibility
    local effective_bottom=$((CHAT_CONTENT_END - BOTTOM_PADDING))
    
    # Calculate starting row (fill from effective bottom up within protected boundaries)
    local available_rows=$((effective_bottom - CHAT_CONTENT_START + 1))
    local message_count=$((visible_end - visible_start))
    local start_row=$((effective_bottom - message_count + 1))
    
    # Ensure we don't go above the protected boundary
    if ((start_row < CHAT_CONTENT_START)); then
        start_row=$CHAT_CONTENT_START
        # Limit message count to fit in available space
        message_count=$((effective_bottom - CHAT_CONTENT_START + 1))
        visible_start=$((visible_end - message_count))
    fi
    
    # Draw messages from calculated start row, respecting boundaries
    local row=$start_row
    for ((i=visible_start; i<visible_end && row <= effective_bottom; i++)); do
        goto $row 3
        local msg="${chat_messages[i]}"
        local color="${chat_colors[i]}"
        
        # Word wrap long messages with boundary protection
        local max_width=$((TERMINAL_WIDTH - 6))
        if ((${#msg} > max_width)); then
            echo -ne "${color}${msg:0:$max_width}${RESET}"
            ((row++))
            if ((row <= effective_bottom)); then
                goto $row 5
                echo -ne "${DIM}${msg:$max_width}${RESET}"
            fi
        else
            echo -ne "${color}${msg}${RESET}"
        fi
        ((row++))
    done
    
    # Show scroll indicators
    show_scroll_indicators_bottom_up
}

# Top-down chat rendering (traditional)  
refresh_chat_top_down() {
    local start_msg=$chat_scroll_offset
    local end_msg=$((start_msg + chat_inner_height))
    
    # Draw visible messages within protected boundaries
    local row=$CHAT_CONTENT_START
    for ((i=start_msg; i<end_msg && i<${#chat_messages[@]} && row <= CHAT_CONTENT_END; i++)); do
        if [[ $i -ge 0 ]]; then
            goto $row 3
            local msg="${chat_messages[i]}"
            local color="${chat_colors[i]}"
            
            # Word wrap long messages with boundary protection
            local max_width=$((TERMINAL_WIDTH - 6))
            if ((${#msg} > max_width)); then
                echo -ne "${color}${msg:0:$max_width}${RESET}"
                ((row++))
                if ((row <= CHAT_CONTENT_END)); then
                    goto $row 5
                    echo -ne "${DIM}${msg:$max_width}${RESET}"
                fi
            else
                echo -ne "${color}${msg}${RESET}"
            fi
        fi
        ((row++))
    done
    
    # Show scroll indicators
    show_scroll_indicators_top_down
}

# Show scroll indicators for bottom-up mode
show_scroll_indicators_bottom_up() {
    local total_messages=${#chat_messages[@]}
    
    # Can scroll up (see older messages) - only within protected boundaries
    if ((chat_scroll_offset < 0)); then
        goto $CHAT_CONTENT_START $((TERMINAL_WIDTH - 1))
        echo -ne "${DIM}↑${RESET}"
    fi
    
    # Can scroll down (see newer messages) - only within protected boundaries
    if ((total_messages + chat_scroll_offset > chat_inner_height)); then
        goto $CHAT_CONTENT_END $((TERMINAL_WIDTH - 1))
        echo -ne "${DIM}↓${RESET}"
    fi
}

# Show scroll indicators for top-down mode
show_scroll_indicators_top_down() {
    if ((chat_scroll_offset > 0)); then
        goto $CHAT_CONTENT_START $((TERMINAL_WIDTH - 1))
        echo -ne "${DIM}↑${RESET}"
    fi
    if ((chat_scroll_offset + chat_inner_height < ${#chat_messages[@]})); then
        goto $CHAT_CONTENT_END $((TERMINAL_WIDTH - 1))
        echo -ne "${DIM}↓${RESET}"
    fi
}

# Clear input area - respects protected boundaries with padding
clear_input() {
    local input_row=$INPUT_PROMPT_ROW
    goto $input_row 5  # Left padding
    echo -ne "$CLEAR_LINE"
    goto $input_row $((TERMINAL_WIDTH - 1))
    # Redraw prompt with padding
    goto $input_row 5  # Left padding
    echo -ne "${BOLD}You:${RESET} "
}

# Simulate assistant typing
simulate_typing() {
    local message="$1"
    local char_delay=0.05
    
    update_status "Assistant is typing..."
    
    # Show typing indicator in chat
    local typing_row=$((CHAT_HEIGHT + 3))
    goto $typing_row 3
    echo -ne "${ASSISTANT_COLOR}Assistant: ${DIM}"
    
    for ((i=0; i<${#message}; i++)); do
        echo -ne "${message:$i:1}"
        sleep $char_delay
    done
    
    echo -ne "$RESET"
    sleep 0.5
    
    # Clear typing area and add to proper chat
    goto $typing_row 3
    echo -ne "$CLEAR_LINE"
    goto $typing_row $((TERMINAL_WIDTH - 1))
    
    add_message "Assistant" "$message" "$ASSISTANT_COLOR"
    update_status "Ready"
}

# Handle slash commands
handle_slash_command() {
    local cmd="$1"
    case "$cmd" in
        "/quit"|"/exit"|"/q")
            cleanup
            ;;
        "/clear"|"/cls")
            chat_messages=()
            chat_colors=()
            chat_scroll_offset=0
            refresh_chat_area
            add_message "System" "Chat cleared." "$STATUS_COLOR"
            ;;
        "/help"|"/h")
            add_message "System" "Available commands:" "$STATUS_COLOR"
            add_message "System" "/quit, /exit, /q - Exit chat" "$STATUS_COLOR"
            add_message "System" "/clear, /cls - Clear chat history" "$STATUS_COLOR"
            add_message "System" "/scroll [up|down] - Scroll chat" "$STATUS_COLOR"
            add_message "System" "/orientation - Toggle top-down/bottom-up" "$STATUS_COLOR"
            add_message "System" "/spam, /flood - Generate lots of test messages" "$STATUS_COLOR"
            add_message "System" "/status - Show connection status" "$STATUS_COLOR"
            add_message "System" "/time - Show current time" "$STATUS_COLOR"
            add_message "System" "/help, /h - Show this help" "$STATUS_COLOR"
            ;;
        "/scroll up"|"/up")
            if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
                # Bottom-up: scroll up means see older messages (negative offset)
                ((chat_scroll_offset--))
                refresh_chat_area
                add_message "System" "Scrolled up (older messages)" "$STATUS_COLOR"
            else
                # Top-down: traditional scroll up
                if ((chat_scroll_offset > 0)); then
                    ((chat_scroll_offset--))
                    refresh_chat_area
                    add_message "System" "Scrolled up" "$STATUS_COLOR"
                else
                    add_message "System" "Already at top" "$STATUS_COLOR"
                fi
            fi
            ;;
        "/scroll down"|"/down")
            if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
                # Bottom-up: scroll down means see newer messages (positive offset toward 0)
                if ((chat_scroll_offset < 0)); then
                    ((chat_scroll_offset++))
                    refresh_chat_area
                    add_message "System" "Scrolled down (newer messages)" "$STATUS_COLOR"
                else
                    add_message "System" "Already at bottom" "$STATUS_COLOR"
                fi
            else
                # Top-down: traditional scroll down
                if ((chat_scroll_offset + chat_inner_height < ${#chat_messages[@]})); then
                    ((chat_scroll_offset++))
                    refresh_chat_area
                    add_message "System" "Scrolled down" "$STATUS_COLOR"
                else
                    add_message "System" "Already at bottom" "$STATUS_COLOR"
                fi
            fi
            ;;
        "/status")
            add_message "System" "Status: Connected | Messages: ${#chat_messages[@]} | Terminal: ${TERMINAL_WIDTH}x${TERMINAL_HEIGHT}" "$STATUS_COLOR"
            ;;
        "/time")
            local current_time=$(date "+%H:%M:%S")
            add_message "System" "Current time: $current_time" "$STATUS_COLOR"
            ;;
        "/debug")
            add_message "System" "Debug: orientation=$CHAT_ORIENTATION, scroll_offset=$chat_scroll_offset, inner_height=$chat_inner_height, msg_count=${#chat_messages[@]}" "$STATUS_COLOR"
            add_message "System" "Boundaries: content_start=$CHAT_CONTENT_START, content_end=$CHAT_CONTENT_END, padding=$BOTTOM_PADDING" "$STATUS_COLOR"
            ;;
        "/orientation")
            if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
                CHAT_ORIENTATION="top"
                add_message "System" "Switched to top-down orientation" "$STATUS_COLOR"
            else
                CHAT_ORIENTATION="bottom"
                add_message "System" "Switched to bottom-up orientation (like a terminal feed!)" "$STATUS_COLOR"
            fi
            chat_scroll_offset=0
            refresh_chat_area
            ;;
        "/spam"|"/flood")
            add_message "System" "🌊 SPAM FLOOD INCOMING! 🌊" "$STATUS_COLOR"
            local spam_messages=(
                "This is spam message #1 - testing the scrolling system!"
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                "The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs."
                "Rolo layout engine is absolutely crushing this chat demo! No flicker, smooth scrolling, beautiful borders!"
                "Bottom-up orientation makes this feel like a real terminal feed. So satisfying!"
                "ANSI escape sequences are pure magic when you know how to use them properly."
                "This spam command is perfect for testing scroll behavior in both orientations."
                "Watch how the messages flow differently in top-down vs bottom-up modes!"
                "Bash can actually create surprisingly sophisticated terminal UIs when done right."
                "Differential rendering beats full-screen redraws every single time."
                "🚀 Performance is incredible - only updating what actually changed!"
                "Terminal UI development is an art form that deserves more appreciation."
                "Claude Code makes prototyping terminal applications so much faster."
                "The scroll indicators show exactly when there's more content to see."
                "Word wrapping works seamlessly even with the custom layout engine."
                "Status bar updates without affecting the chat content - surgical precision!"
                "Input area stays fixed while chat content flows independently above."
                "This would be perfect for a real-time log viewer or chat application."
                "Imagine building a terminal-based IDE with this kind of layout control!"
                "🎯 Mission accomplished: flicker-free terminal chat interface!"
            )
            
            for msg in "${spam_messages[@]}"; do
                add_message "SpamBot" "$msg" "$ASSISTANT_COLOR"
                sleep 0.1  # Small delay to see the flood effect
            done
            
            add_message "System" "🏁 Spam flood complete! Try scrolling to see all messages." "$STATUS_COLOR"
            ;;
        *)
            add_message "System" "Unknown command: $cmd (try /help)" "$STATUS_COLOR"
            ;;
    esac
}

# Main chat loop
chat_loop() {
    draw_layout
    update_status "Ready"
    
    # Welcome message
    local orientation_desc="top-down"
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        orientation_desc="bottom-up (terminal feed style)"
    fi
    add_message "System" "Welcome to Rolo Chat! Mode: $orientation_desc" "$STATUS_COLOR"
    add_message "System" "Type /help for commands, /orientation to toggle, /quit to exit." "$STATUS_COLOR"
    add_message "Assistant" "Hello! I'm your chat assistant. Try /orientation to see the difference!" "$ASSISTANT_COLOR"
    
    # Position cursor for input - use padded prompt row
    local input_row=$INPUT_PROMPT_ROW
    
    # Ensure layout is fully rendered before showing cursor
    sleep 0.1
    echo -e "$SHOW_CURSOR"
    goto $input_row 10  # Position after "You: " with left padding
    
    while true; do
        # Ensure proper cursor positioning and show prompt with padding
        goto $input_row 5  # Left padding
        echo -ne "${BOLD}You:${RESET} "
        read -e user_input
        
        # Handle empty input
        if [[ -z "$user_input" ]]; then
            clear_input
            continue
        fi
        
        # Handle slash commands
        if [[ "$user_input" =~ ^/ ]]; then
            handle_slash_command "$user_input"
            clear_input
            continue
        fi
        
        # Add user message
        add_message "You" "$user_input" "$USER_COLOR"
        clear_input
        
        # Generate assistant response
        local responses=(
            "That's an interesting point! Let me think about that."
            "I understand what you're asking. Here's my perspective..."
            "Good question! The layout engine is working smoothly, isn't it?"
            "This flicker-free approach really makes a difference in user experience."
            "I can see how rolo could be much better than traditional terminal UIs."
            "The differential update system prevents that annoying screen flash."
            "Notice how the chat scrolls smoothly without redrawing everything?"
        )
        
        local response="${responses[$((RANDOM % ${#responses[@]}))]}"
        simulate_typing "$response"
        
        # Reset cursor for next input
        goto $input_row 8
    done
}

# Cleanup on exit
cleanup() {
    echo -e "$ENABLE_SCROLLBACK$SHOW_CURSOR$CLEAR_SCREEN"
    goto 1 1
    echo "Thanks for trying Rolo Chat Demo!"
    exit 0
}

trap cleanup INT TERM

# Main entry point
main() {
    echo "Rolo Chat Terminal Demo"
    echo "======================"
    echo
    echo "Features demonstrated:"
    echo "• Flicker-free chat interface"
    echo "• Smooth scrolling chat history"
    echo "• Real-time typing indicators"
    echo "• Efficient selective updates"
    echo "• No terminal scrollback interference (uses alternate screen)"
    echo
    echo "Slash commands: /help, /quit, /orientation, /spam, /clear, /status"
    echo "Environment: CHAT_ORIENTATION=${CHAT_ORIENTATION} (set to 'bottom' for feed mode)"
    echo
    read -p "Press Enter to start chat..."
    
    chat_loop
    cleanup
}

# Check if running directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi