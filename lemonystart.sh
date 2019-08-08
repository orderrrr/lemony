#!/bin/bash
killall lemonbar
mpd --kill
killall mpd
sleep 1
# variables
font_size=8
padding=29

x=$(xrandr --listmonitors | wc -l &)

if [ $x -gt 2 ]
then
  bspc config -m DP-0 top_padding 0
  bspc config -m HDMI-0 top_padding $padding
else
  bspc config top_padding $padding
fi

# lemony is required to be built and in your PATH
lemony $1 | lemonbar -p -g 1900x25+10+10 -d -f "FixedsysExcelsiorIIIb Nerd Font Mono:size=$font_size" -B$(cat ~/.cache/walmate/colors | head -1) &
sleep 1

xdo above -t $(xdo id -n root) $(xdo id -n lemonbar)
