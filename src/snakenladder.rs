/* Copyright (c) 2019 Saraswathi Govind Datar NA
[This program is licensed under the "MIT License"]
 Please see the file LICENSE in the source
 distribution of this software for license terms.*/

#[macro_use]
extern crate text_io;

use rand::Rng;
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

//Snake and ladder class consisting of Board layout and number of players in the game
pub struct SNL {
    boardlayout: Board,
    numplayers: Numplayers,
}

//create a new game object
impl SNL {
    pub fn new() -> SNL {
        SNL {
            boardlayout: Board::new(),
            numplayers: Numplayers::new(),
        }
    }

    //draws the initial board layout
    pub fn ramstart(&mut self) {
        println!("************JAI SIYARAM ************");
        self.boardlayout.drawlayout();
        let numplayers = self.numplayers.getplayers();
    }
}

//Board class with layout
pub struct Board {
    layout: bool,
}

//create a new board object
impl Board {
    pub fn new() -> Board {
        return Board { layout: true };
    }

    //Function to draw the layout of the board
    pub fn drawlayout(&mut self) {
        let border = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! S  ~SNAKES & LADDERS-> R !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
        let mut input = vec![
            vec![
                "|Begin     ",
                "|02->[22]  ",
                "|03        ",
                "|04        ",
                "|05->[11]  ",
                "|06        ",
                "|07        ",
                "|08        ",
                "|09->[17]  ",
                "|10         |",
            ],
            vec![
                "|11        ",
                "|12        ",
                "|13~{Begin}",
                "|14        ",
                "|15        ",
                "|16        ",
                "|17        ",
                "|18->[32]  ",
                "|19        ",
                "|20         |",
            ],
            vec![
                "|21        ",
                "|22        ",
                "|23        ",
                "|24        ",
                "|25~{04}   ",
                "|26        ",
                "|27        ",
                "|28        ",
                "|29->[36]  ",
                "|30         |",
            ],
            vec![
                "|31        ",
                "|32        ",
                "|33        ",
                "|34        ",
                "|35~{16}   ",
                "|36        ",
                "|37        ",
                "|38->[57]  ",
                "|39        ",
                "|40         |",
            ],
            vec![
                "|41        ",
                "|42        ",
                "|43        ",
                "|44~{22}   ",
                "|45        ",
                "|46        ",
                "|47->[68]  ",
                "|48        ",
                "|49        ",
                "|50         |",
            ],
            vec![
                "|51        ",
                "|52        ",
                "|53->[64]  ",
                "|54        ",
                "|55        ",
                "|56        ",
                "|57        ",
                "|58        ",
                "|59        ",
                "|60         |",
            ],
            vec![
                "|61        ",
                "|62        ",
                "|63        ",
                "|64        ",
                "|65        ",
                "|66~{51}   ",
                "|67        ",
                "|68        ",
                "|69->[76]  ",
                "|70         |",
            ],
            vec![
                "|71        ",
                "|72        ",
                "|73        ",
                "|74->[92]  ",
                "|75        ",
                "|76        ",
                "|77~{45}   ",
                "|78        ",
                "|79        ",
                "|80         |",
            ],
            vec![
                "|81        ",
                "|82        ",
                "|83->[91]  ",
                "|84        ",
                "|85        ",
                "|86~{61}   ",
                "|87        ",
                "|88        ",
                "|89->[95]  ",
                "|90         |",
            ],
            vec![
                "|91        ",
                "|92        ",
                "|93        ",
                "|94->{82}  ",
                "|95        ",
                "|96        ",
                "|97        ",
                "|98~{78}   ",
                "|99        ",
                "|End        |",
            ],
        ];

        println!("{:?}", border);
        for (i, row) in input.iter().enumerate() {
            for (j, mut col) in row.iter().enumerate() {
                print!("{}", col);
            }
            println!();
        }
        println!("{:?}", border);
    }

    //function to update the layout
    pub fn updatelayout(&mut self) {
        let border = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! S  ~SNAKES & LADDERS-> R !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
        let mut input = vec![
            vec![
                "|Begin     ",
                "|02->[22]  ",
                "|03        ",
                "|04        ",
                "|05->[11]  ",
                "|06        ",
                "|07        ",
                "|08        ",
                "|09->[17]  ",
                "|10         |",
            ],
            vec![
                "|11        ",
                "|12        ",
                "|13~{Begin}",
                "|14        ",
                "|15        ",
                "|16        ",
                "|17        ",
                "|18->[32]  ",
                "|19        ",
                "|20         |",
            ],
            vec![
                "|21        ",
                "|22        ",
                "|23        ",
                "|24        ",
                "|25~{04}   ",
                "|26        ",
                "|27        ",
                "|28        ",
                "|29->[36]  ",
                "|30         |",
            ],
            vec![
                "|31        ",
                "|32        ",
                "|33        ",
                "|34        ",
                "|35~{16}   ",
                "|36        ",
                "|37        ",
                "|38->[57]  ",
                "|39        ",
                "|40         |",
            ],
            vec![
                "|41        ",
                "|42        ",
                "|43        ",
                "|44~{22}   ",
                "|45        ",
                "|46        ",
                "|47->[68]  ",
                "|48        ",
                "|49        ",
                "|50         |",
            ],
            vec![
                "|51        ",
                "|52        ",
                "|53->[64]  ",
                "|54        ",
                "|55        ",
                "|56        ",
                "|57        ",
                "|58        ",
                "|59        ",
                "|60         |",
            ],
            vec![
                "|61        ",
                "|62        ",
                "|63        ",
                "|64        ",
                "|65        ",
                "|66~{51}   ",
                "|67        ",
                "|68        ",
                "|69->[76]  ",
                "|70         |",
            ],
            vec![
                "|71        ",
                "|72        ",
                "|73        ",
                "|74->[92]  ",
                "|75        ",
                "|76        ",
                "|77~{45}   ",
                "|78        ",
                "|79        ",
                "|80         |",
            ],
            vec![
                "|81        ",
                "|82        ",
                "|83->[91]  ",
                "|84        ",
                "|85        ",
                "|86~{61}   ",
                "|87        ",
                "|88        ",
                "|89->[95]  ",
                "|90         |",
            ],
            vec![
                "|91        ",
                "|92        ",
                "|93        ",
                "|94->{82}  ",
                "|95        ",
                "|96        ",
                "|97        ",
                "|98~{78}   ",
                "|99        ",
                "|End        |",
            ],
        ];

        println!("{:?}", border);
        for (i, row) in input.iter().enumerate() {
            for (j, mut col) in row.iter().enumerate() {
                print!("{}", col);
            }
            println!();
        }
        println!("{:?}", border);
    }
}

//Number of players class consisting of num_players
pub struct Numplayers {
    num_players: usize,
}

//create a new number of players object
impl Numplayers {
    pub fn new() -> Numplayers {
        return Numplayers { num_players: 0 };
    }

    pub fn getplayers(&mut self) -> usize {
        let mut num_players: usize = 0;
        num_players
    }
}

struct Players<'a> {
    postion: usize,
    avatar: &'a str,
    avatar_display: &'a str,
    avatar_overlap: &'a str,
}

/*
impl<'a> Players<'a>{
    pub fn new(numplayers: usize) -> Players{

        return Players{
            postion: 0,
            avatar: " ",
            avatar_display: " ",
            avatar_overlap: " "};
    }
}*/
