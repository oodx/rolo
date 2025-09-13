#!/bin/bash

# Multi-Pane Facts Paradise - Chatty facts + flicker-free selective rendering!

# Terminal setup
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor
stty -echo

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# MEGA FACTS DATABASE! 🎉
RANDOM_FACTS=(
    "🌊 The wave emoji looks like a gremlin from far away! 😂"
    "🐙 Octopuses have three hearts and blue blood!"
    "🍯 Honey never spoils - archaeologists found edible honey in Egyptian tombs!"
    "🦋 Butterflies taste with their feet!"
    "🌙 A day on Venus is longer than its year!"
    "🐧 Penguins have knees hidden inside their bodies!"
    "🌈 Bananas are berries, but strawberries aren't!"
    "🧠 Your brain uses 20% of your body's energy despite being 2% of your weight!"
    "🐠 Goldfish can live for over 40 years with proper care!"
    "⚡ Lightning strikes the Earth 8 million times per day!"
    "🌍 Earth is the only known planet where fire can naturally occur!"
    "🦎 Geckos can run up glass surfaces due to van der Waals forces!"
    "📡 Radio waves from Earth have traveled about 100 light-years into space!"
    "🎵 The longest recorded flight of a chicken is 13 seconds!"
    "🌕 The Moon is moving away from Earth at 3.8cm per year!"
    "🐜 Ants can lift 10-50 times their own body weight!"
    "🎨 Shrimp can see 16 types of color receptors (humans only see 3)!"
    "🌋 There are more possible chess games than atoms in the observable universe!"
    "🦆 Ducks have waterproof feathers that never get wet!"
    "🌪️ A cloud can weigh more than a million pounds!"
    "🦘 Kangaroos can't walk backwards!"
    "🐨 Koalas sleep 18-22 hours per day!"
    "🎯 Your stomach gets an entirely new lining every 3-5 days!"
    "🌊 There's more water in the atmosphere than in all rivers combined!"
    "🦖 T-Rex lived closer in time to humans than to Stegosaurus!"
    "🍄 The largest organism on Earth is a fungus in Oregon!"
    "⭐ Neutron stars are so dense that a teaspoon would weigh 6 billion tons!"
    "🐙 Octopuses can change color faster than a chameleon!"
    "🌱 Bamboo can grow up to 3 feet in a single day!"
    "🦅 Peregrine falcons can dive at speeds over 240 mph!"
    "🧬 You share 50% of your DNA with bananas!"
    "🌊 The pressure at the bottom of the ocean would crush you instantly!"
    "🎪 A group of flamingos is called a 'flamboyance'!"
    "🚀 Space is completely silent because there's no air to carry sound!"
    "🦈 Sharks have been around longer than trees!"
    "🌸 Cherry blossoms are actually related to roses!"
    "🧊 Hot water can freeze faster than cold water (Mpemba effect)!"
    "🐧 Emperor penguins can hold their breath for 20+ minutes!"
    "🌈 There are infinite shades of green your eyes can distinguish!"
    "⚡ Your body produces enough heat in 30 minutes to boil water!"
)

# Content pools
CHAT_MESSAGES=(
    "user: these facts are AMAZING! 🤯"
    "bot: I know right? The octopus ones blow my mind!"
    "user: wait bananas are berries??"
    "bot: Yep! And strawberries are NOT berries! 🍓"
    "user: my brain is exploding 🧠💥"
    "bot: That brain is using 20% of your energy right now!"
    "user: this is so much better than boring chat"
    "bot: Facts make everything more interesting! ✨"
    "user: I'm learning so much!"
    "bot: Knowledge is power! Keep those facts coming! 🚀"
)

SYSTEM_CONTENT="CPU: 67% ██████▓░░░
RAM: 84% ████████▓░
Disk: 45% ████▓░░░░░
Network: 5.8MB/s ↑↓
Facts shown: 0"

CODE_CONTENT="$ rolo --facts-paradise enabled
  ✓ Random facts: LOADED
  ✓ Popup system: ACTIVE
  ✓ Selective rendering: ON
$ facts --auto-popup
  🎲 Random fact every 5s
  📊 Database: ${#RANDOM_FACTS[@]} facts
  ⚡ Zero flicker guaranteed!"

# Buffers
declare -a CHAT_BUFFER=()
declare -a FACTS_BUFFER=()
CHAT_SIZE=6
FACTS_SIZE=4
FACTS_SHOWN=0

# Content tracking for change detection
PREV_CHAT_CONTENT=""
PREV_SYSTEM_CONTENT=""
PREV_CODE_CONTENT=""
PREV_FACTS_CONTENT=""

# ANSI positioning function
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Clear specific area (instead of full screen)
clear_area() {
    local start_row=$1
    local start_col=$2
    local width=$3
    local height=$4

    for ((row=start_row; row<start_row+height; row++)); do
        goto $row $start_col
        printf "%*s" $width ""
    done
}

# Add new chat message
add_to_chat() {
    local new_message="${CHAT_MESSAGES[$((RANDOM % ${#CHAT_MESSAGES[@]}))]}"
    CHAT_BUFFER=("$new_message" "${CHAT_BUFFER[@]}")
    if [ ${#CHAT_BUFFER[@]} -gt $CHAT_SIZE ]; then
        CHAT_BUFFER=("${CHAT_BUFFER[@]:0:$CHAT_SIZE}")
    fi
}

# Add new random fact popup!
add_random_fact() {
    local new_fact="${RANDOM_FACTS[$((RANDOM % ${#RANDOM_FACTS[@]}))]}"
    FACTS_BUFFER=("$new_fact" "${FACTS_BUFFER[@]}")
    if [ ${#FACTS_BUFFER[@]} -gt $FACTS_SIZE ]; then
        FACTS_BUFFER=("${FACTS_BUFFER[@]:0:$FACTS_SIZE}")
    fi
    ((FACTS_SHOWN++))
}

# Render boxy content at specific positions ONLY if changed
render_boxy_if_changed() {
    local content="$1"
    local prev_content="$2"
    local start_row=$3
    local start_col=$4
    local max_width=$5
    local max_height=$6

    # Only render if content actually changed
    if [[ "$content" != "$prev_content" ]]; then
        # Clear the area first (only the specific pane area)
        clear_area $start_row $start_col $max_width $max_height

        # Render the new content
        local line_num=0
        while IFS= read -r line; do
            goto $((start_row + line_num)) $start_col
            printf "%s" "$line"
            ((line_num++))
        done <<< "$content"
    fi
}

# Main render function - NO FULL SCREEN CLEAR!
render_all_panes() {
    # Generate current content
    local chat_content=""
    for message in "${CHAT_BUFFER[@]}"; do
        chat_content+="$message"$'\n'
    done

    local facts_content=""
    for fact in "${FACTS_BUFFER[@]}"; do
        facts_content+="$fact"$'\n'
    done

    local current_chat_boxy=$(echo -n "$chat_content" | boxy --style rounded --color azure --header "💬 Chat About Facts" --width 40)
    local current_facts_boxy=$(echo -n "$facts_content" | boxy --style double --color magenta --header "🎲 Random Facts Popup!" --width 45)
    local current_system_boxy=$(echo "$SYSTEM_CONTENT" | boxy --style heavy --color green --header "📊 System + Facts Counter" --width 30)
    local current_code_boxy=$(echo "$CODE_CONTENT" | boxy --style ascii --color yellow --header "⚡ Facts Paradise Control" --width 35)

    # Only update status bar on first render
    if [[ -z "$PREV_CHAT_CONTENT" ]]; then
        goto 1 1
        printf "\033[2K\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Facts Paradise - Chatty Facts + Flicker-Free Magic! Facts: $FACTS_SHOWN - Press 'q' to quit "
    fi

    # Render each pane ONLY if its content changed (true Zellij pattern!)
    render_boxy_if_changed "$current_chat_boxy" "$PREV_CHAT_CONTENT" 3 2 42 10      # Chat
    render_boxy_if_changed "$current_facts_boxy" "$PREV_FACTS_CONTENT" 3 45 47 8    # Facts popup!
    render_boxy_if_changed "$current_system_boxy" "$PREV_SYSTEM_CONTENT" 14 2 32 9  # System
    render_boxy_if_changed "$current_code_boxy" "$PREV_CODE_CONTENT" 14 36 37 10    # Code control

    # Update previous content for next comparison
    PREV_CHAT_CONTENT="$current_chat_boxy"
    PREV_FACTS_CONTENT="$current_facts_boxy"
    PREV_SYSTEM_CONTENT="$current_system_boxy"
    PREV_CODE_CONTENT="$current_code_boxy"

    # Update facts counter in status (minimal change)
    goto 1 65
    printf "Facts: $FACTS_SHOWN"

    # Update bottom info periodically
    if [[ $((SECONDS % 8)) -eq 0 ]]; then
        goto $((TERM_HEIGHT - 1)) 1
        printf "\033[2K\033[90m🎲 Facts Paradise: ${#RANDOM_FACTS[@]} facts loaded → Selective updates → ZERO FLICKER! 🎉\033[0m"
    fi
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
Facts shown: $FACTS_SHOWN"
}

update_code_control() {
    local timestamp=$(date +%H:%M:%S)

    CODE_CONTENT="$ rolo --facts-paradise enabled
  ✓ Random facts: LOADED
  ✓ Popup system: ACTIVE
  ✓ Facts database: ${#RANDOM_FACTS[@]} items
$ facts --status
  📊 Facts shown: $FACTS_SHOWN
  🎲 Last update: $timestamp
  ⚡ FLICKER-FREE: YES!"
}

# Main demo loop
main() {
    echo "🎲 ROLO Facts Paradise - The Ultimate Chatty Facts Experience!"
    echo "Loading ${#RANDOM_FACTS[@]} amazing facts..."

    # Initialize with some content
    for i in {1..3}; do
        add_to_chat
    done

    # Start with one fact
    add_random_fact

    echo "Press any key to start Facts Paradise..."
    read -n 1 -s

    # Initial full render (only time we render everything)
    printf "\033[2J"  # Clear once at start
    render_all_panes

    local last_chat_update=0
    local last_fact_update=0

    # Facts Paradise main loop!
    while true; do
        local current_time=$SECONDS

        # Check for quit
        if read -t 0.1 -n 1 key 2>/dev/null; then
            if [[ "$key" == "q" ]]; then
                break
            fi
        fi

        # Add new chat message every 6 seconds
        if [ $((current_time - last_chat_update)) -ge 6 ]; then
            add_to_chat
            last_chat_update=$current_time
        fi

        # Add new RANDOM FACT every 5 seconds! 🎲✨
        if [ $((current_time - last_fact_update)) -ge 5 ]; then
            add_random_fact
            last_fact_update=$current_time
        fi

        # Update other content at different intervals
        if [ $((current_time % 4)) -eq 0 ]; then
            update_system
        fi

        if [ $((current_time % 7)) -eq 0 ]; then
            update_code_control
        fi

        # Render only changed panes (the magic!)
        render_all_panes

        sleep 0.2  # Smooth updates
    done
}

# Cleanup function
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty echo
    clear
    echo "✨ Facts Paradise Demo complete!"
    echo "🎲 You learned $FACTS_SHOWN amazing facts!"
    echo "🎉 All with flicker-free selective rendering!"
    echo "🚀 Knowledge + zero flicker = Paradise! ✨"
}

trap cleanup EXIT
main