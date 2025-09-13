#!/bin/bash

# Rolo Chat Terminal Demo with Boxy Integration
# Combines manual layout control with boxy theming

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
DISABLE_SCROLLBACK="\033[?47h"
ENABLE_SCROLLBACK="\033[?47l"

# Boxy theme colors (extracted from --colors)
BOXY_INFO="\033[38;5;33m"      # Azure blue for info
BOXY_SUCCESS="\033[38;5;34m"   # Emerald green for success  
BOXY_WARNING="\033[38;5;214m"  # Amber for warnings
BOXY_ERROR="\033[38;5;196m"    # Crimson for errors
BOXY_TEXT="\033[0m"            # Default text
RESET="\033[0m"

# Layout dimensions
TERMINAL_HEIGHT=$(tput lines)
TERMINAL_WIDTH=$(tput cols)
CHAT_HEIGHT=$((TERMINAL_HEIGHT - 6))
INPUT_HEIGHT=4

# Protected zones
CHAT_CONTENT_START=3
CHAT_CONTENT_END=$((CHAT_HEIGHT - 2))
INPUT_PROMPT_ROW=$((CHAT_HEIGHT + 3))
BOTTOM_PADDING=1

# Chat orientation and state
CHAT_ORIENTATION=${CHAT_ORIENTATION:-top}
declare -a chat_messages=()
declare -a chat_colors=()
declare -a chat_themes=()  # Store boxy theme for each message
chat_scroll_offset=0
chat_inner_height=$((CHAT_CONTENT_END - CHAT_CONTENT_START + 1))

# Position cursor
goto() {
    echo -ne "\033[${1};${2}H"
}

# Get boxy border characters for manual drawing
get_boxy_chars() {
    local theme="$1"
    case "$theme" in
        "info"|"") 
            echo -e "${BOXY_INFO}┌─┐│└┘${RESET}"
            ;;
        "success")
            echo -e "${BOXY_SUCCESS}╭─╮│╰╯${RESET}"  # Rounded for success
            ;;
        "error")
            echo -e "${BOXY_ERROR}╔═╗║╚╝${RESET}"   # Double for errors
            ;;
        "warning") 
            echo -e "${BOXY_WARNING}┌─┐│└┘${RESET}"
            ;;
    esac
}

# Draw layout using boxy-inspired styling
draw_layout() {
    echo -e "$DISABLE_SCROLLBACK$CLEAR_SCREEN$HIDE_CURSOR"
    
    # Use boxy info theme for main layout
    local chars=$(get_boxy_chars "info")
    local tl="${BOXY_INFO}┌${RESET}"
    local tr="${BOXY_INFO}┐${RESET}" 
    local bl="${BOXY_INFO}└${RESET}"
    local br="${BOXY_INFO}┘${RESET}"
    local h="${BOXY_INFO}─${RESET}"
    local v="${BOXY_INFO}│${RESET}"
    
    # Top border with boxy styling
    goto 1 1
    echo -n "$tl"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "$h"; done
    echo -n "$tr"
    
    # Chat area title (boxy style)
    goto 1 3
    echo -n "${BOXY_INFO}[ ${BOXY_TEXT}🚀 Rolo Chat Terminal${BOXY_INFO} ]${RESET}"
    
    # Side borders
    for ((i=2; i<=CHAT_HEIGHT; i++)); do
        goto $i 1
        echo -n "$v"
        goto $i $TERMINAL_WIDTH
        echo -n "$v"
    done
    
    # Separator (boxy style)
    local sep_row=$((CHAT_HEIGHT + 1))
    goto $sep_row 1
    echo -n "${BOXY_INFO}├${RESET}"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "$h"; done
    echo -n "${BOXY_INFO}┤${RESET}"
    
    # Status area with boxy theme indicator
    goto $sep_row 3
    echo -n "${BOXY_INFO}[ ${BOXY_TEXT}Status${BOXY_INFO} ]${RESET}"
    
    # Input area borders
    for ((i=$((sep_row + 1)); i<=$((sep_row + INPUT_HEIGHT - 1)); i++)); do
        goto $i 1
        echo -n "$v"
        goto $i $TERMINAL_WIDTH
        echo -n "$v"
    done
    
    # Bottom border
    local bottom_row=$((sep_row + INPUT_HEIGHT - 1))
    goto $bottom_row 1
    echo -n "$bl"
    for ((i=2; i<TERMINAL_WIDTH; i++)); do echo -n "$h"; done
    echo -n "$br"
    
    # Input prompt placeholder
    goto $INPUT_PROMPT_ROW 5
    echo -n "${BOXY_INFO}Type a message or /help for commands...${RESET}"
}

# Update status with boxy theming
update_status() {
    local status_text="$1"
    local theme="${2:-info}"
    local status_row=$((CHAT_HEIGHT + 1))
    
    # Get theme color
    local color="$BOXY_INFO"
    case "$theme" in
        "success") color="$BOXY_SUCCESS";;
        "error") color="$BOXY_ERROR";;
        "warning") color="$BOXY_WARNING";;
    esac
    
    echo -ne "$SAVE_CURSOR"
    goto $status_row $((TERMINAL_WIDTH - ${#status_text} - 2))
    echo -ne "$CLEAR_LINE"
    echo -ne "${color}${status_text}${RESET}"
    echo -ne "$RESTORE_CURSOR"
}

# Add message with boxy theme integration
add_message() {
    local sender="$1"
    local message="$2"
    local theme="${3:-info}"
    
    chat_messages+=("${sender}: ${message}")
    chat_themes+=("$theme")
    
    # Auto-scroll to latest
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        chat_scroll_offset=0
    else
        if ((${#chat_messages[@]} > chat_inner_height)); then
            chat_scroll_offset=$(( ${#chat_messages[@]} - chat_inner_height ))
        fi
    fi
    
    refresh_chat_area
}

# Format message with boxy theme colors
format_message_with_theme() {
    local message="$1"
    local theme="$2"
    
    case "$theme" in
        "success")
            echo -ne "${BOXY_SUCCESS}${message}${RESET}"
            ;;
        "error") 
            echo -ne "${BOXY_ERROR}${message}${RESET}"
            ;;
        "warning")
            echo -ne "${BOXY_WARNING}${message}${RESET}"
            ;;
        "user")
            echo -ne "${BOXY_INFO}${message}${RESET}"
            ;;
        "system")
            echo -ne "${BOXY_WARNING}${message}${RESET}"
            ;;
        *)
            echo -ne "${BOXY_TEXT}${message}${RESET}"
            ;;
    esac
}

# Refresh chat area with theme-aware rendering
refresh_chat_area() {
    # Clear chat content area
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

# Bottom-up rendering with boxy themes
refresh_chat_bottom_up() {
    local total_messages=${#chat_messages[@]}
    local visible_start=$((total_messages - chat_inner_height + chat_scroll_offset))
    local visible_end=$((total_messages + chat_scroll_offset))
    
    # Bounds checking
    if ((visible_start < 0)); then
        visible_start=0
    fi
    if ((visible_end > total_messages)); then
        visible_end=$total_messages
    fi
    
    # Calculate positioning with padding
    local effective_bottom=$((CHAT_CONTENT_END - BOTTOM_PADDING))
    local message_count=$((visible_end - visible_start))
    local start_row=$((effective_bottom - message_count + 1))
    
    if ((start_row < CHAT_CONTENT_START)); then
        start_row=$CHAT_CONTENT_START
        message_count=$((effective_bottom - CHAT_CONTENT_START + 1))
        visible_start=$((visible_end - message_count))
    fi
    
    # Render messages with themes
    local row=$start_row
    for ((i=visible_start; i<visible_end && row <= effective_bottom; i++)); do
        goto $row 3
        local msg="${chat_messages[i]}"
        local theme="${chat_themes[i]}"
        
        format_message_with_theme "$msg" "$theme"
        ((row++))
    done
    
    show_scroll_indicators
}

# Top-down rendering with boxy themes  
refresh_chat_top_down() {
    local start_msg=$chat_scroll_offset
    local end_msg=$((start_msg + chat_inner_height))
    
    local row=$CHAT_CONTENT_START
    for ((i=start_msg; i<end_msg && i<${#chat_messages[@]} && row <= CHAT_CONTENT_END; i++)); do
        if [[ $i -ge 0 ]]; then
            goto $row 3
            local msg="${chat_messages[i]}"
            local theme="${chat_themes[i]}"
            
            format_message_with_theme "$msg" "$theme"
        fi
        ((row++))
    done
    
    show_scroll_indicators
}

# Show scroll indicators with boxy styling
show_scroll_indicators() {
    if ((chat_scroll_offset > 0)) || [[ "$CHAT_ORIENTATION" == "bottom" && $chat_scroll_offset < 0 ]]; then
        goto $CHAT_CONTENT_START $((TERMINAL_WIDTH - 1))
        echo -ne "${BOXY_INFO}↑${RESET}"
    fi
    
    local can_scroll_down=false
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        ((${#chat_messages[@]} + chat_scroll_offset > chat_inner_height)) && can_scroll_down=true
    else
        ((chat_scroll_offset + chat_inner_height < ${#chat_messages[@]})) && can_scroll_down=true
    fi
    
    if $can_scroll_down; then
        goto $CHAT_CONTENT_END $((TERMINAL_WIDTH - 1))
        echo -ne "${BOXY_INFO}↓${RESET}"
    fi
}

# Clear input with boxy styling
clear_input() {
    local input_row=$INPUT_PROMPT_ROW
    goto $input_row 5
    echo -ne "$CLEAR_LINE"
    goto $input_row $((TERMINAL_WIDTH - 1))
    goto $input_row 5
    echo -ne "${BOXY_INFO}You:${RESET} "
}

# Enhanced slash commands with boxy themes
handle_slash_command() {
    local cmd="$1"
    case "$cmd" in
        "/quit"|"/exit"|"/q")
            cleanup
            ;;
        "/clear"|"/cls")
            chat_messages=()
            chat_themes=()
            chat_scroll_offset=0
            refresh_chat_area
            add_message "System" "Chat cleared." "success"
            ;;
        "/theme")
            add_message "System" "Available themes: info, success, error, warning" "info"
            add_message "System" "Examples: /test info, /test error, /test success" "info"
            ;;
        "/test info")
            add_message "Assistant" "This is an info message with boxy theming!" "info"
            ;;
        "/test success")
            add_message "Assistant" "✅ Operation completed successfully!" "success"
            ;;
        "/test error")
            add_message "Assistant" "❌ Something went wrong!" "error"
            ;;
        "/test warning")
            add_message "Assistant" "⚠️ Warning: deprecated feature" "warning"
            ;;
        "/spam"|"/flood")
            add_message "System" "🌊 Boxy-themed spam incoming!" "warning"
            local themes=("info" "success" "error" "warning")
            for i in {1..15}; do
                local theme="${themes[$((i % 4))]}"
                add_message "SpamBot" "Message #$i with $theme theme" "$theme"
                sleep 0.1
            done
            add_message "System" "✅ Spam complete! Notice the themed colors!" "success"
            ;;
        "/orientation")
            if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
                CHAT_ORIENTATION="top"
                add_message "System" "Switched to top-down orientation" "success"
            else
                CHAT_ORIENTATION="bottom"  
                add_message "System" "Switched to bottom-up feed mode" "success"
            fi
            chat_scroll_offset=0
            refresh_chat_area
            ;;
        "/help"|"/h")
            add_message "System" "🚀 Boxy-Enhanced Rolo Chat Commands:" "info"
            add_message "System" "/theme - Show available boxy themes" "info"
            add_message "System" "/test [theme] - Test theme colors" "info"
            add_message "System" "/spam - Themed message flood" "info"
            add_message "System" "/orientation - Toggle chat direction" "info"
            add_message "System" "/quit - Exit chat" "info"
            ;;
        *)
            add_message "System" "Unknown command: $cmd (try /help)" "error"
            ;;
    esac
}

# Main chat loop
chat_loop() {
    draw_layout
    update_status "Ready" "success"
    
    # Welcome with themes
    add_message "System" "🚀 Welcome to Boxy-Enhanced Rolo Chat!" "success"
    add_message "System" "Now featuring professional boxy theming!" "info"
    add_message "Assistant" "Try /theme to see available styles, or /test [theme] to demo!" "info"
    
    local input_row=$INPUT_PROMPT_ROW
    sleep 0.1
    echo -e "$SHOW_CURSOR"
    goto $input_row 10
    
    while true; do
        goto $input_row 5
        echo -ne "${BOXY_INFO}You:${RESET} "
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
        
        # User messages get user theme
        add_message "You" "$user_input" "user"
        clear_input
        
        # Simulate assistant with info theme
        update_status "Assistant typing..." "warning"
        sleep 0.5
        
        local responses=(
            "That's interesting! The boxy themes really make this pop!"
            "I love how the colors distinguish message types."
            "The professional styling makes this feel like a real app!"
            "Notice how each message type has its own boxy theme."
            "This layout engine is coming together nicely!"
        )
        
        local response="${responses[$((RANDOM % ${#responses[@]}))]}"
        add_message "Assistant" "$response" "info"
        update_status "Ready" "success"
        
        goto $input_row 10
    done
}

# Cleanup
cleanup() {
    echo -e "$ENABLE_SCROLLBACK$SHOW_CURSOR$CLEAR_SCREEN"
    goto 1 1
    echo "Thanks for trying Boxy-Enhanced Rolo Chat!"
    exit 0
}

trap cleanup INT TERM

# Main entry
main() {
    echo "🚀 Boxy-Enhanced Rolo Chat Demo"
    echo "================================"
    echo
    echo "Features:"
    echo "• Professional boxy theming (info, success, error, warning)"
    echo "• Theme-aware message colors" 
    echo "• Enhanced visual styling"
    echo "• All original rolo features preserved"
    echo
    echo "New commands: /theme, /test [theme]"
    echo "Environment: CHAT_ORIENTATION=${CHAT_ORIENTATION}"
    echo
    read -p "Press Enter to start boxy chat..."
    
    chat_loop
    cleanup
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi