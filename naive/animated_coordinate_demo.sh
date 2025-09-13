#!/bin/bash

# Animated Coordinate Demo - Moving patterns with your gorgeous colors! 🎨

# Your beautiful color palette!
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

readonly revc=$'\x1B[7m'
readonly bld=$'\x1B[1m'
readonly reset_color=$'\x1B[0m'
readonly eol=$'\x1B[K'

# Colors array
COLORS=("$red2" "$red" "$orange" "$yellow" "$green" "$blue" "$blue2" "$cyan" "$magenta" "$purple" "$purple2" "$white" "$white2")

# Terminal dimensions
TERM_WIDTH=$(tput cols)
TERM_HEIGHT=$(tput lines)

goto() {
    printf "\033[%d;%dH" "$1" "$2"
}

clear_screen() {
    printf "\033[2J\033[?25l"
}

# Draw a moving sine wave
draw_sine_wave() {
    local offset=$1
    local amplitude=8
    local center_y=$((TERM_HEIGHT / 2))

    for ((pos_x=5; pos_x<TERM_WIDTH-5; pos_x++)); do
        local angle=$(( (pos_x + offset) * 314 / 100 ))  # Scaled for bash math
        local pos_y=$(( center_y + (amplitude * (angle % 628 - 314) / 314) ))

        # Bounds check
        if [ $pos_y -gt 0 ] && [ $pos_y -lt $TERM_HEIGHT ]; then
            local color="${COLORS[$((pos_x % ${#COLORS[@]}))]}"
            goto $pos_y $pos_x
            printf "%s●%s" "$color" "$reset_color"
        fi
    done
}

# Draw expanding circles
draw_expanding_circle() {
    local center_x=$((TERM_WIDTH / 2))
    local center_y=$((TERM_HEIGHT / 2))
    local radius=$1

    for ((angle=0; angle<360; angle+=15)); do
        local rad_angle=$((angle * 314 / 18000))  # Convert to radians (roughly)
        local pos_x=$((center_x + radius * (angle % 180 - 90) / 90))
        local pos_y=$((center_y + radius * ((angle + 90) % 180 - 90) / 90))

        if [ $pos_x -gt 0 ] && [ $pos_x -lt $TERM_WIDTH ] && [ $pos_y -gt 0 ] && [ $pos_y -lt $TERM_HEIGHT ]; then
            local color="${COLORS[$((angle / 30))]}"
            goto $pos_y $pos_x
            printf "%s◆%s" "$color" "$reset_color"
        fi
    done
}

# Draw a bouncing ball
draw_bouncing_ball() {
    local ball_x=$1
    local ball_y=$2
    local color="$3"

    goto $ball_y $ball_x
    printf "%s●%s (%d,%d)" "$color" "$reset_color" "$ball_x" "$ball_y"
}

# Rain effect
draw_rain() {
    for ((i=0; i<20; i++)); do
        local drop_x=$((RANDOM % TERM_WIDTH))
        local drop_y=$((RANDOM % TERM_HEIGHT))
        local color="${COLORS[$((RANDOM % ${#COLORS[@]}))]}"

        goto $drop_y $drop_x
        printf "%s|%s" "$color" "$reset_color"
    done
}

# Animated demos
demo_sine_wave() {
    clear_screen
    goto 1 1
    printf "%s🌊 Sine Wave Animation - Press any key to continue%s" "$bld$cyan" "$reset_color"

    for ((frame=0; frame<50; frame++)); do
        # Clear previous wave
        for ((clear_y=5; clear_y<TERM_HEIGHT-5; clear_y++)); do
            goto $clear_y 5
            printf "%*s" $((TERM_WIDTH-10)) ""
        done

        draw_sine_wave $frame
        sleep 0.1

        # Check for key press
        if read -t 0.01 -n 1; then
            break
        fi
    done
}

demo_expanding_circles() {
    clear_screen
    goto 1 1
    printf "%s◆ Expanding Circles - Press any key to continue%s" "$bld$purple" "$reset_color"

    for ((radius=1; radius<20; radius++)); do
        clear_screen
        goto 1 1
        printf "%s◆ Expanding Circles - Press any key to continue%s" "$bld$purple" "$reset_color"

        draw_expanding_circle $radius
        sleep 0.2

        if read -t 0.01 -n 1; then
            break
        fi
    done
}

demo_bouncing_ball() {
    clear_screen
    goto 1 1
    printf "%s● Bouncing Ball - Press any key to continue%s" "$bld$yellow" "$reset_color"

    local ball_x=10
    local ball_y=10
    local vel_x=1
    local vel_y=1

    for ((frame=0; frame<100; frame++)); do
        # Clear previous ball position
        goto $ball_y $ball_x
        printf "        "

        # Update position
        ball_x=$((ball_x + vel_x))
        ball_y=$((ball_y + vel_y))

        # Bounce off walls
        if [ $ball_x -le 1 ] || [ $ball_x -ge $((TERM_WIDTH-10)) ]; then
            vel_x=$((-vel_x))
        fi
        if [ $ball_y -le 3 ] || [ $ball_y -ge $((TERM_HEIGHT-2)) ]; then
            vel_y=$((-vel_y))
        fi

        # Draw ball
        local color="${COLORS[$((frame % ${#COLORS[@]}))]}"
        draw_bouncing_ball $ball_x $ball_y "$color"

        sleep 0.05

        if read -t 0.01 -n 1; then
            break
        fi
    done
}

demo_rain() {
    clear_screen
    goto 1 1
    printf "%s| Colorful Rain - Press any key to continue%s" "$bld$blue2" "$reset_color"

    for ((frame=0; frame<30; frame++)); do
        draw_rain
        sleep 0.1

        if read -t 0.01 -n 1; then
            break
        fi
    done
}

# Main menu
main() {
    clear_screen

    while true; do
        clear_screen
        goto 1 1
        printf "%s🎨 Animated Coordinate Demos! 🎨%s" "$bld$cyan" "$reset_color"

        goto 3 1
        printf "%s1. Sine Wave Animation%s" "$green" "$reset_color"
        goto 4 1
        printf "%s2. Expanding Circles%s" "$purple" "$reset_color"
        goto 5 1
        printf "%s3. Bouncing Ball%s" "$yellow" "$reset_color"
        goto 6 1
        printf "%s4. Colorful Rain%s" "$blue2" "$reset_color"
        goto 7 1
        printf "%s5. Quit%s" "$red" "$reset_color"

        goto 9 1
        printf "%sChoose a demo (1-5):%s " "$white" "$reset_color"

        read -n 1 choice

        case $choice in
            1) demo_sine_wave ;;
            2) demo_expanding_circles ;;
            3) demo_bouncing_ball ;;
            4) demo_rain ;;
            5) break ;;
        esac
    done

    # Cleanup
    printf "\033[?25h\033[2J"
    goto 1 1
    printf "%sThanks for watching the animated demos!%s\n" "$green" "$reset_color"
}

main