#!/bin/bash

CONFIG="$HOME/.config/probman/config.json"


if [ ! -f "$CONFIG" ]; then
    read -p "Enter the full path to your src/problems directory: " PROBLEM_DIR
    mkdir -p "$(dirname "$CONFIG")"
    echo "{ \"problemdir\": \"$PROBLEM_DIR\" }" > "$CONFIG"
else
    PROBLEM_DIR=$(jq -r '.problemdir' "$CONFIG")
fi


action=$1 # action
probid=$2 # problem id

if [ -z "$action" ] || [ -z "$probid" ]; then
	echo "Usage: $0 <get|test> <problem_id>"
	exit 1
fi


echo "[ probman ]: Getting problem $probid"
rm -f "$PROBLEM_DIR"/Solution.*
"$PROBLEM_DIR"/ProblemManager "$action" "$probid" "$PROBLEM_DIR"
