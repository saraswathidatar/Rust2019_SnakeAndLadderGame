//! # Snake and Ladder Game
//!
//!  Copyright (c) 2019 Saraswathi Govind Datar NA
//This program is licensed under the "MIT License"
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#[macro_use]
extern crate text_io;
extern crate termion;
use rand::Rng;
use std::collections::HashMap;
use termion::color;

//Snake and ladder class consisting of number of players and player info in the game
pub struct Snl {
    numplayers: Numplayers,
    players: Players<'static>,
}

//create a new game object
impl Snl {
    pub fn new() -> Snl {
        Snl {
            numplayers: Numplayers::new(),
            players: Players::new(),
        }
    }
    ///draws the initial board layout
    pub fn gamestart(&mut self) {
        let numplayers = self.numplayers.getplayers();
        let arr_players = self.players.updateplayerinfo(numplayers);
        self.players.gameplay(arr_players, numplayers);
    }
}

//Number of players class consisting of num_players
pub struct Numplayers {
    num_players: usize,
}

//create a new number of players object
impl Numplayers {
    pub fn new() -> Numplayers {
        Numplayers { num_players: 0 }
    }

    pub fn getplayers(&mut self) -> usize {
        println!("{:?}", "How many of you dare to enter the pit - 2 3 or 4?");
        //Use function to select input players
        let mut num_players: usize;
        scan!("{}", num_players);
        while num_players > 4 || num_players < 2 {
            println!("{:?}", "Please choose between 2 3 or 4?");
            scan!("{}", num_players);
        }
        self.num_players = num_players;
        //Return number of players. This value is used throughout the game.
        num_players
    }
}

//Create a struct to hold all player related information
#[derive(Copy, Clone)]
struct Players<'a> {
    position: usize,
    avatar: &'a str,
    avatar_display: &'a str,
    avatar_overlap: &'a str,
}

impl<'a> Players<'a> {
    //PLayer constructor
    pub fn new() -> Players<'static> {
        let arr_players = Players {
            position: 1,
            avatar: "|*A*       ",
            avatar_overlap: "|*A*#B#    ",
            avatar_display: "*A*",
        };
        arr_players
    }

    //This function helps initialize  players' position on the board
    pub fn updateplayerinfo(&mut self, numplayers: usize) -> Vec<Players<'static>> {
        //Uses a vec of type players to push all player related data
        let mut arr_players = vec![];
        match numplayers {
            2 => {
                arr_players.push(Players {
                    position: 1,
                    avatar: "|*A*       ",
                    avatar_overlap: "|*A*#B#    ",
                    avatar_display: "*A*",
                });
                println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|#B#       ",
                    avatar_overlap: "|*A*#B#    ",
                    avatar_display: "#B#",
                });
                println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
            }
            3 => {
                arr_players.push(Players {
                    position: 1,
                    avatar: "|*A*       ",
                    avatar_overlap: "|*A*#B#$C$ ",
                    avatar_display: "*A*",
                });
                println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|#B#       ",
                    avatar_overlap: "|*A*#B#$C$ ",
                    avatar_display: "#B#",
                });
                println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|$C$       ",
                    avatar_overlap: "|*A*#B#$C$ ",
                    avatar_display: "$C$",
                });
                println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
            }
            4 => {
                arr_players.push(Players {
                    position: 1,
                    avatar: "|*A*       ",
                    avatar_overlap: "|*A#B$C%D  ",
                    avatar_display: "*A*",
                });
                println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|#B#       ",
                    avatar_overlap: "|*A#B$C%D  ",
                    avatar_display: "#B#",
                });
                println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|$C$       ",
                    avatar_overlap: "|*A#B$C%D  ",
                    avatar_display: "$C$",
                });
                println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
                arr_players.push(Players {
                    position: 1,
                    avatar: "|%D%       ",
                    avatar_overlap: "|*A#B$C%D  ",
                    avatar_display: "%D%",
                });
                println!("Player D's avatar is {:?}", arr_players[3].avatar_display);
            }
            _ => (),
        }
        arr_players
    }

    ///function to Define a hash for snakes and ladders
    pub fn laddersnakeshash(&mut self, position: usize) -> usize {
        //Defining hash for ladders
        let mut ladders = HashMap::new();
        let mut new_position = position;
        ladders.insert(02, 22);
        ladders.insert(05, 11);
        ladders.insert(09, 17);
        ladders.insert(18, 32);
        ladders.insert(29, 36);
        ladders.insert(38, 57);
        ladders.insert(47, 68);
        ladders.insert(53, 64);
        ladders.insert(69, 76);
        ladders.insert(74, 92);
        ladders.insert(83, 91);
        ladders.insert(89, 95);

        //check if a player gets a ladder
        if let Some(x) = ladders.get(&position) {
                println!("YAY! You rolled & reached a Ladder!!");
                new_position = *x;
                println!(
                    "You reach {:?} and climb your way up to {:?}",
                    position, new_position
                );
            }


        //Defining a hash for the snakes
        let mut snakes = HashMap::new();
        snakes.insert(13, 1);
        snakes.insert(25, 4);
        snakes.insert(35, 16);
        snakes.insert(44, 22);
        snakes.insert(66, 51);
        snakes.insert(77, 45);
        snakes.insert(86, 61);
        snakes.insert(94, 82);
        snakes.insert(98, 78);

        //check if a player gets a snake
        if let Some(x) = snakes.get(&position) {
                println!("Oh NO!!!! You got bit by a Snake!!***");
                new_position = *x;
                println!(
                    "You reach {:?} to slide all the way back to {:?}",
                    position, new_position
                );
            }
        new_position
    }

    pub fn gameplay(&mut self, mut arr_players: Vec<Players<'static>>, num_players: usize) {
        //Variables to define board layout
        let border = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! S  ~SNAKES & LADDERS-> R !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
        let input = vec![
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

        let mut row_num: [usize; 4] = [0; 4];
        let mut column: [usize; 4] = [0; 4];

        //Draw initial layout
        for k in 0..num_players {
            row_num[k] = (arr_players[k].position - 1) / 10;
            column[k] = (arr_players[k].position - 1) % 10;
        }
        //Display the board
        println!();
        println!("**** {:?} Players have entered the pit - Chaos is a ladder [or a snake]. You either climb or get bit! Roll & enjoy the chaos...****", num_players);
        println!("*************Snakes & Ladders initial board layout: ***************");
        println!("{:?}", border);
        for (i, row) in input.iter().enumerate() {
            for (j, mut col) in row.iter().enumerate() {
                for k in 0..num_players {
                    if i == row_num[k] && j == column[k] {
                        col = &arr_players[k].avatar_overlap;
                    }
                }
                print!("{}", col);
            }
            println!()
        }
        println!("{:?}", border);

         //Keep the game in loop till any player reaches end (block 100)
        'snl: loop {
            for n in 0..num_players {
                let mut random = rand::thread_rng();
                let mut roll: usize;
                let mut correctroll: bool = false;
                //Keep looping till user 'rolls' as expected. Also update position based on roll
                while correctroll == false {
                    println!(
                        "Player {:?}'s current position {:?}",
                        arr_players[n].avatar_display, arr_players[n].position
                    );
                    println!(
                        "Player {:?}, please roll dice by typing 'r' or 'R' ",
                        arr_players[n].avatar_display
                    );
                    let mut wait_roll: String;
                    scan!("{}", wait_roll);

                    let r = "r".to_string();
                    let a = "R".to_string();
                    let new_pos; 

                    if wait_roll == r || wait_roll == a {
                        /* Random number generator for rolling dice. Need to do this for each player */
                        roll = random.gen::<usize>() % 6;
                        println!(
                            "Player {:?} rolled {:?}",
                            arr_players[n].avatar_display,
                            roll + 1
                        );
                        new_pos = arr_players[n].position + (roll + 1);
                        //You must land on 100 to win
                        if new_pos > 100 {
                            println!("Oops, rolled too high. Wait for your turn now!");
                        } else {
                            arr_players[n].position = new_pos;
                            arr_players[n].position = self.laddersnakeshash(arr_players[n].position);
                            println!(
                                "Player {:?}'s new position {:?}",
                                arr_players[n].avatar_display, arr_players[n].position
                            );
                            row_num[n] = (arr_players[n].position - 1) / 10; 
                            column[n] = (arr_players[n].position - 1) % 10;
                        }

                        correctroll = true;

                       //If any player reaches end (block 100) then quit. Cleanly!
                        if arr_players[n].position == 100 {
                            println!("Game over!!");
                            break 'snl;
                        }
                    }
                }
                /* End of function for rolling dice */
            }

            //Define call for upated board display
            println!();
            println! ("##############################################################################################");
            println!("UPDATED board is:");
            println!("{}Red", color::Fg(color::Red));
            for n in 0..num_players {
                print!("{}", color::Fg(color::LightBlue));
                println!(
                    "Player {:?}'s is @:{:?}",
                    arr_players[n].avatar_display, arr_players[n].position
                );
            }
            println!("{}Red", color::Fg(color::Yellow));
            println!("{:?}", border);
            for (i, row) in input.iter().enumerate() {
                for (j, mut col) in row.iter().enumerate() {
                    let mut player = false;
                    for n in 0..num_players {
                        if i == row_num[n] && j == column[n] {
                            col = &mut arr_players[n].avatar;
                            player = true;
                        }
                    }//Use color based markers for players and positions
                    if player == true {
                        print!("{}", color::Fg(color::Red));
                        print!("{}", col);
                    } else {
                        print!("{}", color::Fg(color::Yellow));
                        print!("{}", col);
                    }
                }
                println!()
            }
            println!("{:?}", border);
        }
        /* Logic for Snake and Ladder ends */
    } //gameplay ends
}
