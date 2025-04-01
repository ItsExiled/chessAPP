#!/bin/bash

# Create assets directory if it doesn't exist
mkdir -p assets

# Base URL for chess piece SVGs from Wikimedia Commons
# Using the "Chess Pieces Sprite" set which is available under Creative Commons
BASE_URL="https://commons.wikimedia.org/wiki/Special:Redirect/file/Chess"

# Download assets
PIECES=("Pawn" "Knight" "Bishop" "Rook" "Queen" "King")
COLORS=("White" "Black")

for color in "${COLORS[@]}"; do
    for piece in "${PIECES[@]}"; do
        # Convert to lowercase for URL
        piece_lower=$(echo $piece | tr '[:upper:]' '[:lower:]')
        color_lower=$(echo $color | tr '[:upper:]' '[:lower:]')
        
        # Form filename
        filename="assets/${color_lower}_${piece_lower}.svg"
        
        # Download SVG
        echo "Downloading ${color} ${piece}..."
        curl -o "$filename" "${BASE_URL}_${piece}_${color}.svg"
        
        # Add small delay to be nice to the server
        sleep 1
    done
done

echo "Done downloading chess pieces!"

