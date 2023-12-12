#!/bin/bash/

if (( $# < 1 )) || (( $1 < 1  || $1 > 24 )); then
    echo "Invalid input"
    return
fi

day_zfilled=$(printf "%02g" $1)
year=2023
session="53616c7465645f5fcaf05e0e6dff74fe1912702e5c14adea7b678d03bb93109e098fdf0a255d37a01e28824b824688e72132b50cf3dd52bc89b3afd242998662"

input_url="https://adventofcode.com/$year/day/$1/input"
curl $input_url -b "session=$session" > ./day$day_zfilled/input.txt
