Terminal-based Space Shooter Game in Rust

This project is a simple space shooter game implemented in Rust, using terminal-based rendering with crossterm. The player controls a character represented by an emoji, moving within a dynamically shifting map while avoiding enemies and shooting bullets. The game's map is a series of horizontal borders, and the enemies spawn and move down the screen towards the player.

Table of Contents

Installation
Features
Game Controls
How to Play
Code Overview
Acknowledgments
Installation

Clone the repository:
bash
Copy code
git clone 
Install dependencies:
You will need to install Rust and Cargo. If Rust is not already installed, you can install it from here.
Build the project: Navigate to the project directory and run the following command:
arduino
Copy code
cargo build --release
Run the game: After building the project, run the executable:
arduino
Copy code
cargo run --release
Features

Terminal Graphics: Utilizes the crossterm library to render the game directly in the terminal.
Real-time Player Movement: The player can move left, right, up, and down on the screen.
Shooting Mechanism: Pressing the spacebar will fire bullets upward, allowing the player to shoot at enemies.
Random Enemy Generation: Enemies spawn randomly at the top of the screen and move downward, attempting to hit the player.
Dynamic Map Boundaries: The map's horizontal boundaries move over time, shrinking or expanding the available space for the player.
Game Controls

W: Move the player up.
S: Move the player down.
A: Move the player left.
D: Move the player right.
Spacebar: Shoot bullets upwards.
Q: Quit the game.
How to Play

Start the Game: Run the game from the terminal by executing the command:
arduino
Copy code
cargo run --release
Your goal is to survive as long as possible by avoiding enemies and shooting them down.
Player Movement: Use the W, A, S, and D keys to move your character around the screen.
Firing Bullets: Press the Spacebar to shoot bullets upward. You can only fire one bullet at a time.
Avoid Enemies: Enemies are represented by the letter "E", and they will move down the screen towards you. If they hit you, the game is over.
Game Over: Once you lose, the game will exit with a "Thanks for playing!" message.
