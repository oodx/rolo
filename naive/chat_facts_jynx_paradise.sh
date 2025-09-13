#!/bin/bash

# Ultimate Chat Facts Paradise with JYNX PAINTING! 🎨

# Terminal setup - Zellij style!
clear
printf "\033[?1049h"  # Enter alternate screen
printf "\033[?25l"    # Hide cursor initially
# Put terminal in raw mode for character-by-character input like Zellij
stty raw -echo time 1 min 0  # Raw mode with 0.1s timeout

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

# Colors for regular chat
USER_COLOR="\033[36m"       # Cyan
ASSISTANT_COLOR="\033[32m"  # Green
DIM="\033[2m"
BOLD="\033[1m"
RESET="\033[0m"

# ENHANCED FACTS WITH CODE! 🎉
RANDOM_FACTS=(
    "🌊 The wave emoji looks like a gremlin from far away! 😂"
    "🐙 Octopuses have three hearts and blue blood!"
    "💻 Python: print('Hello World') - the most written line in programming!"
    "🦋 Butterflies taste with their feet!"
    "🌙 A day on Venus is longer than its year!"
    "⚡ JavaScript: console.log('async/await is magic!') - async programming!"
    "🐧 Penguins have knees hidden inside their bodies!"
    "🌈 Bananas are berries, but strawberries aren't!"
    "🧠 Your brain uses 20% of your body's energy despite being 2% of your weight!"
    "🔧 Rust: fn main() { println!(\"Zero-cost abstractions!\"); } - systems programming!"
    "🐠 Goldfish can live for over 40 years with proper care!"
    "⚡ Lightning strikes the Earth 8 million times per day!"
    "🌍 Earth is the only known planet where fire can naturally occur!"
    "📡 HTML: <h1>Hello World!</h1> - the foundation of the web!"
    "🦎 Geckos can run up glass surfaces due to van der Waals forces!"
    "📡 Radio waves from Earth have traveled about 100 light-years into space!"
    "🎵 The longest recorded flight of a chicken is 13 seconds!"
    "🐚 CSS: body { background: linear-gradient(45deg, #ff6b6b, #4ecdc4); } - styling!"
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
)

# Code snippets pool for jynx demonstrations!
CODE_SNIPPETS=(
    "def fibonacci(n):
    if n <= 1: return n
    return fibonacci(n-1) + fibonacci(n-2)"

    "function quickSort(arr) {
    if (arr.length <= 1) return arr;
    const pivot = arr[0];
    return [...quickSort(left), pivot, ...quickSort(right)];
}"

    "fn main() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!(\"Vec: {:?}\", vec);
}"

    "class BinaryTree:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None"

    "SELECT users.name, COUNT(orders.id) as order_count
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id;"

    "#!/bin/bash
for i in {1..10}; do
    echo \"Processing item $i\"
    sleep 0.1
done"
)

# Chat buffers and state
declare -a chat_messages=()
declare -a chat_types=()  # "user", "assistant", "system", "code"
declare -a facts_buffer=()
declare -a code_buffer=()
FACTS_SIZE=3
CODE_SIZE=2
FACTS_SHOWN=0
CODE_SHOWN=0

# System content
SYSTEM_CONTENT="CPU: 67% ██████▓░░░
RAM: 84% ████████▓░
Facts learned: 0
Code snippets: 0
Jynx paintings: 0"

CODE_CONTENT="$ rolo --jynx-paradise enabled
  ✓ Selective boxy: ACTIVE
  ✓ Facts paradise: LOADED
  ✓ Jynx highlighting: READY
  ✓ Code snippets: STREAMING
$ jynx --theme auto
  🎨 Syntax highlighting: ON
  🖌️ Auto-detection: ENABLED"

# Content tracking for flicker-free updates
PREV_CHAT_CONTENT=""
PREV_FACTS_CONTENT=""
PREV_CODE_CONTENT_DISPLAY=""
PREV_SYSTEM_CONTENT=""
PREV_CODE_CONTENT=""
PREV_PROMPT_CONTENT=""

# ANSI positioning function
goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

# Clear specific area (but NEVER touch the input line!)
clear_area() {
    local start_row=$1
    local start_col=$2
    local width=$3
    local height=$4
    local input_line=$((TERM_HEIGHT - 1))

    for ((row=start_row; row<start_row+height; row++)); do
        # Skip the input line - don't clear it!
        if [ $row -ne $input_line ]; then
            goto $row $start_col
            printf "%*s" $width ""
        fi
    done
}

# Apply jynx highlighting to code
apply_jynx_highlighting() {
    local code="$1"
    local language="${2:-auto}"

    # Use jynx to highlight the code
    printf "%s" "$code" | jynx 2>/dev/null || printf "%s" "$code"
}

# Create boxy system message
create_system_boxy() {
    local message="$1"
    local theme="${2:-warning}"
    local header="${3:-⚙️ System}"

    printf "%s" "$message" | boxy --color "$theme" --header "$header" --width 38
}

# Add regular chat message
add_chat_message() {
    local sender="$1"
    local message="$2"
    local type="${3:-user}"

    chat_messages+=("${sender}: ${message}")
    chat_types+=("$type")

    # Keep only last 6 messages for chat pane
    if [ ${#chat_messages[@]} -gt 6 ]; then
        chat_messages=("${chat_messages[@]:1}")
        chat_types=("${chat_types[@]:1}")
    fi
}

# Add system message (gets boxy treatment)
add_system_message() {
    local message="$1"
    local theme="${2:-amber}"
    local header="${3:-⚙️ System}"

    chat_messages+=("SYSTEM: $message")
    chat_types+=("system")

    if [ ${#chat_messages[@]} -gt 6 ]; then
        chat_messages=("${chat_messages[@]:1}")
        chat_types=("${chat_types[@]:1}")
    fi
}

# Add random fact to facts buffer
add_random_fact() {
    local new_fact="${RANDOM_FACTS[$((RANDOM % ${#RANDOM_FACTS[@]}))]}"
    facts_buffer=("$new_fact" "${facts_buffer[@]}")
    if [ ${#facts_buffer[@]} -gt $FACTS_SIZE ]; then
        facts_buffer=("${facts_buffer[@]:0:$FACTS_SIZE}")
    fi
    ((FACTS_SHOWN++))
}

# Add code snippet with jynx highlighting!
add_code_snippet() {
    local new_code="${CODE_SNIPPETS[$((RANDOM % ${#CODE_SNIPPETS[@]}))]}"

    # Apply jynx highlighting
    local highlighted_code=$(apply_jynx_highlighting "$new_code")

    code_buffer=("$highlighted_code" "${code_buffer[@]}")
    if [ ${#code_buffer[@]} -gt $CODE_SIZE ]; then
        code_buffer=("${code_buffer[@]:0:$CODE_SIZE}")
    fi
    ((CODE_SHOWN++))
}

# Render boxy content at specific positions ONLY if changed
render_boxy_if_changed() {
    local content="$1"
    local prev_content="$2"
    local start_row=$3
    local start_col=$4
    local max_width=$5
    local max_height=$6

    if [[ "$content" != "$prev_content" ]]; then
        clear_area $start_row $start_col $max_width $max_height
        local line_num=0
        while IFS= read -r line; do
            goto $((start_row + line_num)) $start_col
            printf "%s" "$line"
            ((line_num++))
        done <<< "$content"
    fi
}

# Generate chat pane content with selective boxy rendering
generate_chat_content() {
    local chat_content=""

    for i in "${!chat_messages[@]}"; do
        local msg="${chat_messages[i]}"
        local type="${chat_types[i]}"

        if [[ "$type" == "system" ]]; then
            local system_msg="${msg#SYSTEM: }"
            local boxy_msg=$(printf "%s" "$system_msg" | boxy --color amber --header "⚙️ System" --width max)
            chat_content+="$boxy_msg"$'\n'
        else
            # Regular message with fast line rendering - use printf to preserve colors
            if [[ "$msg" =~ ^You: ]]; then
                chat_content+="${USER_COLOR}${msg}${RESET}"$'\n'
            else
                chat_content+="${ASSISTANT_COLOR}${msg}${RESET}"$'\n'
            fi
        fi
    done

    printf "%s" "$chat_content"
}

# Main render function - FLICKER-FREE!
render_all_panes() {
    # Generate current content
    local current_chat_content=$(generate_chat_content)

    local facts_content=""
    for fact in "${facts_buffer[@]}"; do
        facts_content+="$fact"$'\n'
    done

    local code_content=""
    for code in "${code_buffer[@]}"; do
        code_content+="$code"$'\n\n'
    done

    # Create a proper prompt box at the bottom
    local prompt_content="Type your message and press Enter
Commands: /help /fact /code /spam /quit
Currently: Facts auto-stream every 8s, Code every 12s"

    local current_chat_boxy=$(printf "%s" "$current_chat_content" | boxy --style rounded --color azure --header "💬 Jynx Chat Paradise" --width max)
    local current_facts_boxy=$(printf "%s" "$facts_content" | boxy --style double --color magenta --header "🎲 Smart Facts!" --width 45)
    local current_code_boxy=$(printf "%s" "$code_content" | boxy --style heavy --color cyan --header "🎨 Jynx Code Gallery" --width 45)
    local current_system_boxy=$(printf "%s" "$SYSTEM_CONTENT" | boxy --style ascii --color green --header "📊 Paradise Stats" --width 30)
    local current_prompt_boxy=$(printf "%s" "$prompt_content" | boxy --style rounded --color yellow --header "⌨️ Input Prompt" --width max)

    # Status bar (only on first render)
    if [[ -z "$PREV_CHAT_CONTENT" ]]; then
        goto 1 1
        printf "\033[2K\033[7m%*s\033[0m" $TERM_WIDTH " ROLO Jynx Paradise - Boxy + Facts + Code Highlighting! Press 'q' to quit "
    fi

    # Render panes ONLY if changed (Zellij pattern!)
    render_boxy_if_changed "$current_chat_boxy" "$PREV_CHAT_CONTENT" 3 1 $((TERM_WIDTH-2)) $((TERM_HEIGHT-12))  # Full-width chat!
    # Temporarily disable other panes to focus on chat + prompt
    # render_boxy_if_changed "$current_facts_boxy" "$PREV_FACTS_CONTENT" 15 1 47 6                   # Auto facts
    # render_boxy_if_changed "$current_code_boxy" "$PREV_CODE_CONTENT_DISPLAY" 15 50 47 8            # Jynx highlighted code!
    # render_boxy_if_changed "$current_system_boxy" "$PREV_SYSTEM_CONTENT" 25 1 32 6                 # Stats
    render_boxy_if_changed "$current_prompt_boxy" "$PREV_PROMPT_CONTENT" $((TERM_HEIGHT-8)) 1 $((TERM_WIDTH-2)) 6  # Prompt box higher up!

    # Update previous content
    PREV_CHAT_CONTENT="$current_chat_boxy"
    PREV_FACTS_CONTENT="$current_facts_boxy"
    PREV_CODE_CONTENT_DISPLAY="$current_code_boxy"
    PREV_SYSTEM_CONTENT="$current_system_boxy"
    PREV_PROMPT_CONTENT="$current_prompt_boxy"

    # Update counters in status
    goto 1 70
    printf "Facts: $FACTS_SHOWN | Code: $CODE_SHOWN"
}

# Update functions
update_system_stats() {
    local cpu=$((RANDOM % 100))
    local ram=$((RANDOM % 100))

    SYSTEM_CONTENT="CPU: ${cpu}% $(printf '█%.0s' $(seq 1 $((cpu/10))))$(printf '░%.0s' $(seq 1 $((10-cpu/10))))
RAM: ${ram}% $(printf '█%.0s' $(seq 1 $((ram/10))))$(printf '░%.0s' $(seq 1 $((10-ram/10))))
Facts learned: $FACTS_SHOWN
Code snippets: $CODE_SHOWN
Jynx paintings: $CODE_SHOWN"
}

# Handle slash commands with enhanced code support
handle_slash_command() {
    local cmd="$1"
    case "$cmd" in
        "/quit"|"/exit"|"/q")
            add_system_message "Thanks for the jynx paradise! You saw $CODE_SHOWN highlighted code snippets!" "crimson" "👋 Goodbye"
            return 1
            ;;
        "/help"|"/h")
            add_system_message "Commands: /fact, /code, /clear, /spam, /quit. Auto-facts + code every 8s!" "azure" "📖 Help"
            ;;
        "/fact")
            add_random_fact
            add_system_message "Random fact added!" "green" "🎲 Fact"
            ;;
        "/code")
            add_code_snippet
            add_system_message "Jynx-highlighted code added to gallery!" "cyan" "🎨 Code"
            ;;
        "/clear")
            chat_messages=()
            chat_types=()
            add_system_message "Chat cleared! Facts and code continue streaming." "green" "✅ Clear"
            ;;
        "/spam")
            add_system_message "Generating code-aware spam..." "amber" "🌊 Spam"
            for i in {1..4}; do
                add_chat_message "CodeBot" "Here's message #$i with some code: print('jynx rocks!')" "assistant"
            done
            add_code_snippet
            add_system_message "Spam + jynx code complete!" "green" "✅ Complete"
            ;;
        *)
            add_system_message "Unknown command: $cmd (try /help)" "crimson" "❌ Error"
            ;;
    esac
    return 0
}

# Position cursor in the prompt area and SHOW the cursor!
show_input_prompt() {
    # Position cursor just below the prompt box for actual input
    local input_row=$((TERM_HEIGHT - 1))
    goto $input_row 1
    printf "${BOLD}> ${RESET}"  # Show prompt
    # Don't show cursor here - we'll do it in the main loop
}

# Main demo loop
main() {
    echo "🎨 ROLO Jynx Paradise - Syntax Highlighting Meets Facts!"
    echo "Loading facts + code snippets with jynx highlighting..."

    # Initialize with some content
    add_chat_message "Assistant" "Welcome to Jynx Paradise! 🎨" "assistant"
    add_chat_message "Assistant" "Watch code get beautifully highlighted!" "assistant"
    add_random_fact
    add_code_snippet
    add_system_message "Jynx Paradise initialized - syntax highlighting ready!" "green" "🚀 Welcome"

    echo "Press any key to start..."
    read -n 1 -s

    # Initial render
    printf "\033[2J"
    render_all_panes

    local last_fact_update=0
    local last_code_update=0
    local last_stats_update=0

    local last_chat_update=0

    # Input buffer to track what user is typing
    local current_input=""

    # Show cursor and input prompt
    show_input_prompt

    # Main loop with chatty bot and background updates
    while true; do
        local current_time=$SECONDS

        # Temporarily disable auto-facts and code to focus on chat
        # Auto-add facts every 8 seconds
        # if [ $((current_time - last_fact_update)) -ge 8 ]; then
        #     add_random_fact
        #     last_fact_update=$current_time
        # fi

        # Auto-add code snippets every 12 seconds
        # if [ $((current_time - last_code_update)) -ge 12 ]; then
        #     add_code_snippet
        #     last_code_update=$current_time
        # fi

        # Chatty bot keeps company every 15 seconds!
        if [ $((current_time - last_chat_update)) -ge 15 ]; then
            local chatty_messages=(
                "How's the typing feeling? Smooth now? 💬"
                "The cursor should be staying put! ✨"
                "No more clunky rendering interrupting your flow! 🚀"
                "Typing should feel natural now! ⌨️"
                "The selective updates are working perfectly! 🎯"
                "Much better input experience! 😊"
            )
            local chatty_msg="${chatty_messages[$((RANDOM % ${#chatty_messages[@]}))]}"
            add_chat_message "Assistant" "$chatty_msg" "assistant"
            last_chat_update=$current_time
            # Mark that we need to render because content changed
            should_render=true
        fi

        # Temporarily disable stats updates
        # Update stats every 5 seconds
        # if [ $((current_time - last_stats_update)) -ge 5 ]; then
        #     update_system_stats
        #     last_stats_update=$current_time
        # fi

        # Only render if content actually changed
        local should_render=false

        if [ "$should_render" = true ]; then
            render_all_panes
            # Reposition cursor after render
            local input_row=$((TERM_HEIGHT - 1))
            goto $input_row $((3 + ${#current_input}))  # Position after "> " + current input
            printf "\033[?25h"  # Show cursor
        fi

        # Handle input Zellij-style with raw mode character reading
        local char=""
        read -n 1 -t 0.1 char 2>/dev/null

        if [[ -n "$char" ]]; then
            if [[ "$char" == "q" && -z "$current_input" ]]; then
                break
            elif [[ "$char" == $'\n' || "$char" == $'\r' ]]; then
                if [[ -n "$current_input" ]]; then
                    # Process the completed input
                    if [[ "$current_input" =~ ^/ ]]; then
                        if ! handle_slash_command "$current_input"; then
                            break
                        fi
                    else
                        add_chat_message "You" "$current_input" "user"
                        local quick_responses=(
                            "That's interesting! Love the chat flow! 💬"
                            "The input preservation is working great! 🎉"
                            "Notice how smooth the selective updates are? ⚡"
                            "Boxy styling looks beautiful! 🎨"
                            "The cursor stays put now! ✨"
                        )
                        local response="${quick_responses[$((RANDOM % ${#quick_responses[@]}))]}"
                        add_chat_message "Assistant" "$response" "assistant"
                        # Force render because new messages were added
                        render_all_panes
                    fi
                    # Clear input and refresh prompt
                    current_input=""
                    goto $input_row 1
                    printf "\033[2K"
                    show_input_prompt
                    # Position cursor right after prompt and show it
                    goto $input_row 3
                    printf "\033[?25h"
                fi
            elif [[ "$char" == $'\177' || "$char" == $'\b' ]]; then
                # Backspace
                if [[ ${#current_input} -gt 0 ]]; then
                    current_input="${current_input%?}"
                    # Completely redraw the input line
                    local input_row=$((TERM_HEIGHT - 1))
                    goto $input_row 1
                    printf "\033[2K${BOLD}> ${RESET}%s" "$current_input"
                    # Position cursor at end of input and show it
                    goto $input_row $((3 + ${#current_input}))
                    printf "\033[?25h"
                fi
            else
                # Regular character - add to input buffer and display
                current_input+="$char"
                printf "%c" "$char"
            fi
        fi

        sleep 0.1
    done
}

# Cleanup - restore terminal properly like Zellij
cleanup() {
    printf "\033[?25h"    # Show cursor
    printf "\033[?1049l"  # Exit alternate screen
    stty sane             # Restore normal terminal mode
    clear
    echo "✨ Jynx Paradise complete!"
    echo "🎲 You learned $FACTS_SHOWN amazing facts!"
    echo "🎨 You saw $CODE_SHOWN beautifully highlighted code snippets!"
    echo "🚀 Boxy + Jynx + Facts = Ultimate terminal paradise!"
}

trap cleanup EXIT
main