#!/bin/bash

# Rolo Chat with Selective Boxy + Top-Right Popup System
# Regular chat = efficient, System messages = boxy, Popups = floating notifications

# ANSI escape codes
SAVE_CURSOR="\033[s"
RESTORE_CURSOR="\033[u"
CLEAR_SCREEN="\033[2J"
CLEAR_LINE="\033[K"
HIDE_CURSOR="\033[?25l"
SHOW_CURSOR="\033[?25h"
DISABLE_SCROLLBACK="\033[?47h"
ENABLE_SCROLLBACK="\033[?47l"

# Colors for regular chat
USER_COLOR="\033[36m"       # Cyan
ASSISTANT_COLOR="\033[32m"  # Green
DIM="\033[2m"
BOLD="\033[1m"
RESET="\033[0m"

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

# Popup positioning
POPUP_START_ROW=2
POPUP_START_COL=$((TERMINAL_WIDTH - 50))  # 50 chars from right edge
POPUP_WIDTH=45

# Chat state
CHAT_ORIENTATION=${CHAT_ORIENTATION:-top}
declare -a chat_messages=()
declare -a chat_types=()
chat_scroll_offset=0
chat_inner_height=$((CHAT_CONTENT_END - CHAT_CONTENT_START + 1))

# MASSIVE random facts database! 🎉
declare -a RANDOM_FACTS=(
    "Honey never spoils - archaeologists found 3000-year-old honey that's still edible!"
    "Octopi have 3 hearts and blue blood!"
    "A group of flamingos is called a 'flamboyance'!"
    "Bananas are berries, but strawberries aren't!"
    "There are more possible chess games than atoms in the observable universe!"
    "Wombat poop is cube-shaped!"
    "A shrimp's heart is in its head!"
    "Dolphins have names for each other!"
    "The immortal jellyfish can reverse its aging process!"
    "Sharks are older than trees (by 50 million years)!"
    "A cloud can weigh over a million pounds!"
    "Your stomach gets entirely new lining every 3-5 days!"
    "Cleopatra lived closer to the moon landing than the building of the pyramids!"
    "Oxford University is older than the Aztec Empire!"
    "There's enough DNA in your body to stretch from Earth to Pluto and back 17 times!"
    "A group of pugs is called a 'grumble'!"
    "Lobsters were once considered prison food!"
    "The shortest war in history lasted 38-45 minutes!"
    "A single strand of spaghetti is called a 'spaghetto'!"
    "Sea otters hold hands while sleeping so they don't drift apart!"
    "Butterflies taste with their feet!"
    "A group of crows is called a 'murder'!"
    "Penguins have knees!"
    "The Eiffel Tower can be 15cm taller in summer due to thermal expansion!"
    "A day on Venus is longer than its year!"
    "Bubble wrap was originally invented as wallpaper!"
    "The unicorn is Scotland's national animal!"
    "Tigers have striped skin, not just striped fur!"
    "A bolt of lightning is 5 times hotter than the sun!"
    "Avocados are toxic to birds!"
    "The human brain uses 20% of your body's energy!"
    "Snails can sleep for up to 3 years!"
    "A group of hedgehogs is called a 'prickle'!"
    "Elephants can't jump!"
    "The heart of a blue whale is the size of a small car!"
    "Humans share 60% of their DNA with bananas!"
    "A jiffy is an actual unit of time: 1/100th of a second!"
    "The longest word in English has 189,819 letters!"
    "Goldfish have memories longer than 3 seconds (actually months)!"
    "The Great Wall of China isn't visible from space with the naked eye!"
)

# Position cursor
goto() {
    echo -ne "\033[${1};${2}H"
}

# Create boxy system message
create_system_boxy() {
    local message="$1"
    local theme="${2:-warning}"
    local header="${3:-⚙️ System}"

    echo "$message" | boxy --color "$theme" --header "$header" --width max --layout "hl"
}

# Create and display popup in top-right corner
show_popup() {
    local message="$1"
    local theme="${2:-azure}"
    local header="${3:-💡 Popup}"
    local duration="${4:-3}"  # Default 3 seconds

    # Generate popup content
    local popup_content=$(echo "$message" | boxy --color "$theme" --header "$header" --width $POPUP_WIDTH --layout "hl")

    # Save current cursor position
    echo -ne "$SAVE_CURSOR"

    # Position and display popup
    goto $POPUP_START_ROW $POPUP_START_COL
    echo "$popup_content"

    # Auto-dismiss after duration (in background)
    (
        sleep $duration
        # Clear popup area
        for ((i=0; i<4; i++)); do
            goto $((POPUP_START_ROW + i)) $POPUP_START_COL
            printf "%*s" $POPUP_WIDTH ""  # Clear with spaces
        done
    ) &

    # Restore cursor position
    echo -ne "$RESTORE_CURSOR"
}

# Show random fact popup
show_random_fact() {
    local fact="${RANDOM_FACTS[$((RANDOM % ${#RANDOM_FACTS[@]}))]}"
    show_popup "$fact" "emerald" "🧠 Fun Fact" 4
}

# Draw basic layout
draw_layout() {
    echo -e "$DISABLE_SCROLLBACK$CLEAR_SCREEN$HIDE_CURSOR"

    # Simple title
    goto 1 1
    echo -ne "${BOLD}🚀 Rolo Chat - Popup Demo${RESET}"

    # Basic separator
    goto $((CHAT_HEIGHT + 1)) 1
    for ((i=1; i<=TERMINAL_WIDTH; i++)); do echo -n "─"; done

    # Input prompt placeholder
    goto $INPUT_PROMPT_ROW 5
    echo -n "${DIM}Type /popup for fun facts!${RESET}"
}

# Add regular chat message
add_chat_message() {
    local sender="$1"
    local message="$2"
    local type="${3:-user}"

    chat_messages+=("${sender}: ${message}")
    chat_types+=("$type")

    # Auto-scroll
    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        chat_scroll_offset=0
    else
        if ((${#chat_messages[@]} > chat_inner_height)); then
            chat_scroll_offset=$(( ${#chat_messages[@]} - chat_inner_height ))
        fi
    fi

    refresh_chat_area
}

# Add system message with boxy styling
add_system_message() {
    local message="$1"
    local theme="${2:-amber}"
    local header="${3:-⚙️ System}"

    local boxy_output=$(create_system_boxy "$message" "$theme" "$header")
    local current_row=$((CHAT_CONTENT_END - 3))

    goto $current_row 1
    echo "$boxy_output"

    goto $((current_row + 4)) 1
    echo ""
}

# Refresh chat area (leave space for popup)
refresh_chat_area() {
    # Clear regular chat content (avoid popup area)
    for ((i=CHAT_CONTENT_START; i<=$((CHAT_CONTENT_END - 5)); i++)); do
        goto $i 2
        # Only clear left side (leave popup area)
        printf "%-*s" $((TERMINAL_WIDTH - POPUP_WIDTH - 5)) ""
    done

    if [[ "$CHAT_ORIENTATION" == "bottom" ]]; then
        refresh_chat_bottom_up
    else
        refresh_chat_top_down
    fi
}

# Bottom-up chat rendering
refresh_chat_bottom_up() {
    local total_messages=${#chat_messages[@]}
    local visible_start=$((total_messages - chat_inner_height + chat_scroll_offset))
    local visible_end=$((total_messages + chat_scroll_offset))

    # Bounds checking
    if ((visible_start < 0)); then visible_start=0; fi
    if ((visible_end > total_messages)); then visible_end=$total_messages; fi

    # Calculate positioning
    local effective_bottom=$((CHAT_CONTENT_END - BOTTOM_PADDING - 5))
    local message_count=$((visible_end - visible_start))
    local start_row=$((effective_bottom - message_count + 1))

    if ((start_row < CHAT_CONTENT_START)); then
        start_row=$CHAT_CONTENT_START
        message_count=$((effective_bottom - CHAT_CONTENT_START + 1))
        visible_start=$((visible_end - message_count))
    fi

    # Render regular messages
    local row=$start_row
    for ((i=visible_start; i<visible_end && row <= effective_bottom; i++)); do
        local msg="${chat_messages[i]}"
        local type="${chat_types[i]}"

        if [[ "$type" != "system" ]]; then
            goto $row 3

            # Truncate message to avoid popup area
            local max_msg_width=$((TERMINAL_WIDTH - POPUP_WIDTH - 10))
            if ((${#msg} > max_msg_width)); then
                msg="${msg:0:$max_msg_width}..."
            fi

            if [[ "$msg" =~ ^You: ]]; then
                echo -ne "${USER_COLOR}${msg}${RESET}"
            else
                echo -ne "${ASSISTANT_COLOR}${msg}${RESET}"
            fi
            ((row++))
        fi
    done
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

            if [[ "$type" != "system" ]]; then
                goto $row 3

                # Truncate message to avoid popup area
                local max_msg_width=$((TERMINAL_WIDTH - POPUP_WIDTH - 10))
                if ((${#msg} > max_msg_width)); then
                    msg="${msg:0:$max_msg_width}..."
                fi

                if [[ "$msg" =~ ^You: ]]; then
                    echo -ne "${USER_COLOR}${msg}${RESET}"
                else
                    echo -ne "${ASSISTANT_COLOR}${msg}${RESET}"
                fi
            fi
        fi
        ((row++))
    done
}

# Clear input area
clear_input() {
    goto $INPUT_PROMPT_ROW 5
    echo -ne "$CLEAR_LINE"
    goto $INPUT_PROMPT_ROW 5
    echo -ne "${BOLD}You:${RESET} "
}

# Enhanced slash commands with popup support
handle_slash_command() {
    local cmd="$1"
    case "$cmd" in
        "/quit"|"/exit"|"/q")
            show_popup "Thanks for using Rolo Chat!" "violet" "👋 Goodbye" 2
            sleep 2
            cleanup
            ;;
        "/clear"|"/cls")
            chat_messages=()
            chat_types=()
            chat_scroll_offset=0
            refresh_chat_area
            add_system_message "Chat history cleared" "green" "✅ Success"
            ;;
        "/popup")
            show_random_fact
            ;;
        "/popup 5")
            for i in {1..5}; do
                show_random_fact
                sleep 1
            done
            ;;
        "/popup spam")
            add_system_message "FACT EXPLOSION MODE ACTIVATED!" "crimson" "🌊 Spam Mode"
            for i in {1..15}; do
                show_random_fact
                sleep 0.5
            done
            ;;
        "/alert")
            show_popup "This is an important alert!" "crimson" "⚠️ Alert" 5
            ;;
        "/celebrate")
            show_popup "🎉 Congratulations! You found the celebration popup!" "violet" "🎊 Party" 4
            ;;
        "/help"|"/h")
            add_system_message "Commands: /popup (fact), /popup 5 (5 facts), /popup spam (explosion!), /alert, /celebrate" "azure" "📖 Help"
            show_popup "Try /popup for random facts!" "emerald" "💡 Tip" 3
            ;;
        *)
            add_system_message "Unknown command: $cmd (try /help)" "crimson" "❌ Error"
            show_popup "Command not found! Try /help" "amber" "⚠️ Warning" 3
            ;;
    esac
}

# Main chat loop
chat_loop() {
    draw_layout

    # Welcome messages
    add_system_message "Welcome to Popup-Enhanced Chat!" "green" "🚀 Welcome"
    sleep 0.5
    show_popup "Welcome! Try /popup for fun facts!" "azure" "👋 Hello" 4
    sleep 1
    add_chat_message "Assistant" "Hello! Try /popup for random fun facts in the top-right!" "assistant"

    local input_row=$INPUT_PROMPT_ROW
    echo -e "$SHOW_CURSOR"
    goto $input_row 10

    while true; do
        goto $input_row 5
        echo -ne "${BOLD}You:${RESET} "
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

        # Regular user message
        add_chat_message "You" "$user_input" "user"
        clear_input

        # Simulate assistant response
        sleep 0.3
        local responses=(
            "That's fascinating!"
            "Have you tried /popup for a random fact?"
            "I love the floating popup system!"
            "The top-right notifications are so clean!"
            "This layered layout approach is working great!"
        )

        local response="${responses[$((RANDOM % ${#responses[@]}))]}"
        add_chat_message "Assistant" "$response" "assistant"

        # Sometimes trigger random popup
        if ((RANDOM % 10 == 0)); then
            sleep 1
            show_popup "Random fact triggered!" "violet" "🎲 Surprise" 3
            sleep 0.5
            show_random_fact
        fi

        goto $input_row 10
    done
}

# Cleanup
cleanup() {
    echo -e "$ENABLE_SCROLLBACK$SHOW_CURSOR$CLEAR_SCREEN"
    goto 1 1
    echo "Thanks for trying Popup Chat Demo!"
    exit 0
}

trap cleanup INT TERM

# Main entry
main() {
    echo "🚀 Rolo Chat - Popup Demo"
    echo "========================"
    echo
    echo "Features:"
    echo "• Fast chat + beautiful system messages"
    echo "• Top-right floating popup notifications"
    echo "• 40+ random fun facts database!"
    echo "• Auto-dismissing notifications"
    echo
    echo "Try: /popup, /popup 5, /popup spam, /alert, /celebrate"
    echo
    read -p "Press Enter to start the popup madness..."

    chat_loop
    cleanup
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi