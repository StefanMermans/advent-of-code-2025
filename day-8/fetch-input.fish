#!/usr/bin/env fish

# Fetch Advent of Code input for a given day and save to `input.txt`.
# Usage: ./fetch-input.fish <day>

if test (count $argv) -lt 1
    echo "Usage: fetch-input.fish <day>"
    exit 2
end

set day $argv[1]

if not string match -r '^[0-9]+$' -- $day
    echo "Error: <day> must be a positive integer" >&2
    exit 3
end

set output_file input.txt
if test -e $output_file
    echo "Error: '$output_file' already exists. Will not overwrite." >&2
    exit 4
end

if not test -f ./cookie.txt
    echo "Error: './cookie.txt' not found. Create it containing your session cookie (e.g. session=...)" >&2
    exit 5
end

set cookie (string trim (cat ./cookie.txt))
if test -z "$cookie"
    echo "Error: './cookie.txt' is empty" >&2
    exit 6
end

set url "https://adventofcode.com/2025/day/$day/input"
echo "Fetching $url ..."

curl -sS -f -L -H "Cookie: $cookie" "$url" -o $output_file
set rc $status
if test $rc -ne 0
    if test -e $output_file
        rm $output_file
    end
    echo "Error: Failed to fetch input (curl exit $rc)." >&2
    exit $rc
end

echo "Saved to $output_file"
