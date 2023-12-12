#!/bin/bash/

current_month=$(date +%m)
day=0

if (( $# > 1 )); then
    day=$1
elif (( $current_month == 12 )); then
    day=$(date +%d)
    echo "Using todays date: $day"
fi

if (( $day < 1  || $day > 24 )); then
    echo "Invalid input"
    return
fi

day_zfilled=$(printf "%02g" $day)
day_directory="day$day_zfilled"

echo "Creating day $day..."

mkdir $day_directory
cd $day_directory && cargo init && cargo add itertools && cargo add anyhow && cd ..

echo $(source get_puzzle_input.sh $day)

