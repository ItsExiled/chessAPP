#!/bin/bash

# Create assets directory if it doesn't exist
mkdir -p assets

# Function to create an SVG piece
create_piece() {
    local color=$1
    local piece=$2
    local bg_color=$3
    local text_color=$4
    
    cat > "assets/${color}_${piece}.svg" << EOF
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="45" height="45" viewBox="0 0 45 45" xmlns="http://www.w3.org/2000/svg">
  <circle cx="22.5" cy="22.5" r="20" fill="${bg_color}" stroke="${text_color}" stroke-width="2"/>
  <text x="22.5" y="30" font-family="Arial" font-size="20" fill="${text_color}"
        text-anchor="middle" dominant-baseline="middle">${piece:0:1}</text>
</svg>
EOF
    echo "Created ${color}_${piece}.svg"
}

# Create pieces for both colors
PIECES=("pawn" "knight" "bishop" "rook" "queen" "king")

# White pieces (white background, black text)
for piece in "${PIECES[@]}"; do
    create_piece "white" "$piece" "#FFFFFF" "#000000"
done

# Black pieces (black background, white text)
for piece in "${PIECES[@]}"; do
    create_piece "black" "$piece" "#000000" "#FFFFFF"
done

echo "Done creating chess pieces!"

# Make the script executable
chmod +x create_placeholder_pieces.sh

