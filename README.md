# Rust2019_SnakeAndLadderGame
Copyright (c) 2019 Saraswathi Govind Datar NA

This project is licensed under the "MIT License". Please see the file LICENSE in this distribution for license terms.
https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame/blob/master/LICENSE

Introduction
Snake and Ladder terminal based game written in Rust for CS510 Spring 2019 Rust Programming course at Portland State University.

Getting Started
Installation:
git clone https://github.com/saraswathidatar/Rust2019_SnakeAndLadderGame
Using a command line tool, navigate to project directory and execute cargo run.

How to Play
Board Setup:
The board consists of blocks from 1 to 100. There are several ladders denoted by -> and snakes represented as ~ on the board. Players are supposed to climb the ladder to further thier moves. If a snake is encountered then player has to slide down the snake to move backward in the board. There can be 2 to 4 players playing this game. Each player is assigned an Avatar. Player has to roll dice on their turn. Random number generation is used for dice. A player rolling 6 gets another turn. The player to first land on block 100 wins. The board gets displayed each time the player's roll their dice along with their position in the board. 

TODOs
Fix the warnings 
Run cargo clippy
Right now if players land up on same block only one player is displayed on board cropping the rest. Need to change that.
Try to incorporate piston 2d graphics crate for drawing a grid, ladders and snakes; and animation crate when a player moves.
