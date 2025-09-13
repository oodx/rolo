#!/bin/bash

# Rolo Chat with Proper Boxy Integration
# Uses boxy for message formatting, manual layout for positioning

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
DISABLE_SCROLLBACK="\033[?47h"
ENABLE_SCROLLBACK="\033[?47l"

# Layout dimensions
TERMINAL_HEIGHT=$(tput lines)
TERMINAL_WIDTH=$(tput cols)
CHAT_HEIGHT=$((TERMINAL_HEIGHT - 8))
INPUT_HEIGHT=4

# Chat state
declare -a chat_messages=()
declare -a chat_themes=()
declare -a chat_senders=()
chat_scroll_offset=0
CHAT_ORIENTATION=${CHAT_ORIENTATION:-top}

# Position cursor
goto() {
    echo -ne "\033[${1};${2}H"
}

# Create a boxy message and capture its output
create_boxy_message() {
    local sender="$1"
    local message="$2" 
    local theme="$3"
    local width="$4"
    
    # Format sender prefix
    local prefix=""
    case "$sender" in
        "You") prefix="💬 You" ;;
        "Assistant") prefix="🤖 Assistant" ;;
        "System") prefix="⚙️ System" ;;
        "SpamBot") prefix="🤖 Bot" ;;
        *) prefix="$sender" ;;
    esac
    
    # Create the boxy message
    echo "$message" | boxy --theme "$theme" --title "$prefix" --width "$width" --no-color=false
}

# Simple layout without fancy boxes
draw_simple_layout() {
    echo -e "$DISABLE_SCROLLBACK$CLEAR_SCREEN$HIDE_CURSOR"
    
    # Title area
    goto 1 1
    echo -n "$(echo "🚀 Rolo Chat with Boxy Messages" | boxy --theme info --width $TERMINAL_WIDTH)"
    
    # Calculate where chat area starts after title
    local chat_start=4
    local status_row=$((TERMINAL_HEIGHT - 4))
    local input_row=$((TERMINAL_HEIGHT - 2))
    
    # Status line
    goto $status_row 1
    echo -n "$(echo "Ready" | boxy --theme success --width $TERMINAL_WIDTH --title "Status")"
    
    # Input area placeholder
    goto $input_row 1
    echo -n "Type your message..."
}

# Display messages in chat area
display_chat_messages() {
    local chat_start=4
    local chat_end=$((TERMINAL_HEIGHT - 6))
    local available_rows=$((chat_end - chat_start))
    
    # Clear chat area
    for ((i=chat_start; i<=chat_end; i++)); do
        goto $i 1
        echo -ne "$CLEAR_LINE"
    done
    
    # Calculate which messages to show
    local total_messages=${#chat_messages[@]}
    local start_idx=0
    
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        # Bottom-up: show latest messages at bottom
        start_idx=$((total_messages - available_rows + chat_scroll_offset))
        if ((start_idx < 0)); then start_idx=0; fi
    else
        # Top-down: traditional scrolling
        start_idx=$chat_scroll_offset
        if ((start_idx < 0)); then start_idx=0; fi
    fi
    
    # Display messages
    local current_row=$chat_start
    local messages_shown=0
    
    for ((i=start_idx; i<total_messages && messages_shown<available_rows; i++)); do
        if ((i >= 0)); then
            goto $current_row 1
            
            # Create boxy message
            local boxed_msg=$(create_boxy_message "${chat_senders[i]}" "${chat_messages[i]}" "${chat_themes[i]}" $((TERMINAL_WIDTH - 4)))
            
            # Display the boxy message (it's multi-line)
            echo "$boxed_msg"
            
            # Advance cursor past the boxy message (typically 3 lines)
            ((current_row += 4))  # Box takes ~3 lines + padding
            ((messages_shown += 4))
        fi
    done
}

# Add message to chat
add_message() {
    local sender="$1"
    local message="$2"
    local theme="${3:-info}"
    
    chat_senders+=("$sender")
    chat_messages+=("$message")
    chat_themes+=("$theme")
    
    # Auto-scroll to latest
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        chat_scroll_offset=0
    else
        # Calculate if we need to scroll
        local total=${#chat_messages[@]}
        local available_rows=$((TERMINAL_HEIGHT - 10))
        if ((total * 4 > available_rows)); then
            chat_scroll_offset=$((total - available_rows / 4))
        fi
    fi
    
    display_chat_messages
}

# Update s