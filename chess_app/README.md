# Chess Game

A simple chess game implemented in Rust using the Iced GUI framework.

## Features

- Full chess rules implementation
- Graphical user interface
- Move validation
- Check and checkmate detection
- Basic game state management

## Building and Running

1. First, generate the chess piece assets:
   ```bash
   # Make the script executable
   chmod +x src/scripts/create_placeholder_pieces.sh
   
   # Run the script to generate piece SVGs
   ./src/scripts/create_placeholder_pieces.sh
   ```

2. Build and run the game:
   ```bash
   cargo run
   ```

## How to Play

- Click on a piece to select it
- Valid moves will be highlighted in blue
- Click on a highlighted square to move the selected piece
- The game automatically handles turn switching and checks for valid moves

## Development

Run the tests:
```bash
cargo test
```

## Project Structure

- `src/main.rs`: Main application entry point
- `src/gui/`: GUI implementation using Iced
- `src/rules/`: Chess rules and move validation
- `src/state/`: Game state management
- `src/assets/`: Asset handling for piece sprites
- `assets/`: SVG files for chess pieces

## License

This project is licensed under the MIT License - see the LICENSE file for details.

