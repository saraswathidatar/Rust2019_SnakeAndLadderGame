# Rust2019_SnakeAndLadderGame
Copyright (c) 2019 Saraswathi Govind Datar NA

This project is licensed under the "MIT License". Please see the file LICENSE in this distribution for license terms.
https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/blob/master/LICENSE

Introduction:

Snake and Ladder terminal based game written in Rust for CS510 Spring 2019 Rust Programming course at Portland State University.

Screenshots:
![Screenshot1.png](https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/blob/master/static/Screenshot1.png)
![Screenshot2.png](https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/blob/master/static/Screenshot2.png)
![Screenshot3.png](https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/blob/master/static/Screenshot3.png)

Getting Started:

Installation-
Install Rust using this link https://www.rust-lang.org/tools/install.

Git clone https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame.

Using a command line tool, navigate to project directory.

Execute cargo build and cargo run to install the dependencies/build the program and run it.

Unit tests can be run with cargo test.

Dependencies:
termion crate in Cargo.toml is used to color the text in the terminal.
Termion does not support Windows.

How to Play:

Board Setup-
The board consists of blocks from 1 to 100. There are several ladders denoted by -> and snakes represented as ~ on the board. Players are supposed to climb the ladder to further thier moves. If a snake is encountered then player has to slide down the snake to move backward in the board. There can be 2 to 4 players playing this game. Each player is assigned an Avatar. Player has to roll dice on their turn. Random number generation is used for dice. The player to first land on block 100 wins. The board gets displayed each time the player's roll their dice along with their position in the board. 

Bug or Issue Tracker

You can report the bugs in the following link: https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/issues

Contact information:
For any queries related to this project, please feel free to contact. My Email is saras3@pdx.edu

Future Goals:
1. Displaying board, Players, ladders and snakes using 2D graphics crate such as Piston
2. Adding animation using animation crate
3. Adding sound using Audio crate
4. Random number generation for dice could be made more random

TODOs:
1. Fix the warnings.
2. Run cargo clippy.
3. Right now if players land up on same block only one player is displayed on board cropping the rest. Need to change that.
4. Try to incorporate piston 2d graphics crate for drawing a grid, ladders and snakes; and animation crate when a player moves.
5. Trying termion
6. Handle cases for non integers in How many players println
7. Write tests
8. code refactor
9. Rustdoc comments
