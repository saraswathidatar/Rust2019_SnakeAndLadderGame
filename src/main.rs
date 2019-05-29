/* Copyright (c) 2019 Saraswathi Govind Datar NA
[This program is licensed under the "MIT License"]
 Please see the file LICENSE in the source
 distribution of this software for license terms.*/

#[macro_use]
extern crate text_io;
#[macro_use]
extern crate snakenladder;
use snakenladder::SNL;

//extern crate scan
use rand::Rng;
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

//use scan::Scan;
//Player's class consisting of their avatar, position in the board, avatar_display for each player, and avatar_overlap
//if player's land up on same block
struct players<'a> {
    postion: usize,
    //avatar: String
    avatar: &'a str,
    avatar_display: &'a str,
    avatar_overlap: &'a str,
}

//function to Define a hash for snakes and ladders
pub fn laddersnakeshash(position: usize) -> usize {
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
    match ladders.get(&position) {
        Some(x) => {
            println!("YAY! You rolled & reached a Ladder!!");
            //println!("RAM : Old position was {:?}", position);
            //arr_players[0].postion = *x;
            new_position = *x;
            println!(
                "You reach {:?} and climb your way up to {:?}",
                position, new_position
            );
        }
        //No ladder matched
        None => (),
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
    match snakes.get(&position) {
        Some(x) => {
            println!("Oh NO!!!! You got bit by a Snake!!***");
            //println!("RAM : Old position was {:?}", position);
            //arr_players[0].postion = *x;
            new_position = *x;
            println!(
                "You reach {:?} to slide all the way back to {:?}",
                position, new_position
            );
        }
        //No snake matched
        None => (),
    }
    return new_position;
}

//main function for snake and ladders game
fn main() {
    println!("{}", "Let's PLay Snakes and Ladders");
    /* Display the board */
    let border = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!  ~SNAKES & LADDERS->  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
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
    let flatten_array: Vec<&str> = input
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();
    //println!("{:?}", flatten_array[99]);

    // This is used to display the board the first time
    println!("Snakes & Ladders initial board layout");
    let mut snl = SNL::new();
    snl.ramstart();

    //AK: Do we want to do only 1 player and use some automation for second player!!!!!! Too much work
    println!("{}", "How many of you dare to enter the pit - 2 3 or 4?");

    //Use function to select input players
    let mut num_players: usize;
    scan!("{}", num_players);
    //println!("Num players is:{}", num_players.unwrap());
    //DO error checking for whether it is a valid integer or not

    //This still need to be in a loop as well. AK need to figure this out wrt about scan function & error check!
    while num_players > 4 || num_players < 2 {
        println!("{}", "RAM: Please choose between 2 3 or 4?");
        scan!("{}", num_players);
    }

    /* Based on munber of players, assigning avatar. AK put this in a separate class */
    let mut arr_players = vec![];
    match num_players {
        2 => {
            arr_players.push(players {
                postion: 1,
                avatar: "|*A*       ",
                avatar_overlap: "|*A*#B#    ",
                avatar_display: "*A*",
            });
            println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|#B#       ",
                avatar_overlap: "|*A*#B#    ",
                avatar_display: "#B#",
            });
            println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
        }
        3 => {
            arr_players.push(players {
                postion: 1,
                avatar: "|*A*       ",
                avatar_overlap: "|*A*#B#$C$ ",
                avatar_display: "*A*",
            });
            println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|#B#       ",
                avatar_overlap: "|*A*#B#$C$ ",
                avatar_display: "#B#",
            });
            println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|$C$       ",
                avatar_overlap: "|*A*#B#$C$ ",
                avatar_display: "$C$",
            });
            println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
        }
        4 => {
            arr_players.push(players {
                postion: 1,
                avatar: "|*A*       ",
                avatar_overlap: "|*A#B$C%D  ",
                avatar_display: "*A*",
            });
            println!("Player A's avatar is {:?}", arr_players[0].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|#B#       ",
                avatar_overlap: "|*A#B$C%D  ",
                avatar_display: "#B#",
            });
            println!("Player B's avatar is {:?}", arr_players[1].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|$C$       ",
                avatar_overlap: "|*A#B$C%D  ",
                avatar_display: "$C$",
            });
            println!("Player C's avatar is {:?}", arr_players[2].avatar_display);
            arr_players.push(players {
                postion: 1,
                avatar: "|%D%       ",
                avatar_overlap: "|*A#B$C%D  ",
                avatar_display: "%D%",
            });
            println!("Player D's avatar is {:?}", arr_players[3].avatar_display);
        }
        _ => (),
    }

    let mut row_num: [usize; 4] = [0; 4];
    let mut column: [usize; 4] = [0; 4];

    for k in 0..num_players {
        row_num[k] = (arr_players[k].postion - 1) / 10;
        column[k] = (arr_players[k].postion - 1) % 10;
    }
    println!();
    println!("**** {:?} Players have entered the pit - Chaos is a ladder [or a snake]. You either climb or get bit! Roll & enjoy the chaos...****", num_players);
    println!("{:?}", border);
    for (i, row) in input.iter().enumerate() {
        for (j, mut col) in row.iter().enumerate() {
            //print!("{}", col);
            for k in 0..num_players {
                if i == row_num[k] && j == column[k] {
                    //col = &mut &"| *  ";
                    col = &mut arr_players[k].avatar_overlap;
                }
            }
            print!("{}", col);
        }
        println!()
    }
    println!("{:?}", border);

    /* End of Board display */

    /* Random number generator for rolling dice. Need to do this for each player */
    'snl: loop {
        for n in 0..num_players {
            //let mut test;
            let mut random = rand::thread_rng();
            let mut roll: usize;
            let mut correctroll: bool = false;
            while correctroll == false {
                println!(
                    "Player {:?}'s current position {:?}",
                    arr_players[n].avatar_display, arr_players[n].postion
                );
                println!(
                    "Player {:?}, please roll dice by typing 'r' or 'R' ",
                    arr_players[n].avatar_display
                );
                let mut wait_roll: String;
                scan!("{}", wait_roll);

                let r = "r".to_string();
                let a = "R".to_string();
                let mut new_pos = 0;

                if wait_roll == r || wait_roll == a {
                    //AK: Define function call for roll function
                    roll = random.gen::<usize>() % 6;
                    println!(
                        "Player {:?} rolled {:?}",
                        arr_players[n].avatar_display,
                        roll + 1
                    );
                    new_pos = arr_players[n].postion + (roll + 1);
                    //You must land on 100 to win
                    if new_pos > 100 {
                        println!("Oops, rolled too high. Wait for your turn now!");
                    } else {
                        arr_players[n].postion = new_pos;
                        arr_players[n].postion = laddersnakeshash(arr_players[n].postion);
                        println!(
                            "Player {:?}'s new position {:?}",
                            arr_players[n].avatar_display, arr_players[n].postion
                        );
                        row_num[n] = (arr_players[n].postion - 1) / 10;
                        column[n] = (arr_players[n].postion - 1) % 10;
                    }

                    correctroll = true;

                    //AK: Define function call for position check
                    if arr_players[n].postion == 100 {
                        //AK: Please call Update table here
                        println!("RAM - Game over!!");
                        break 'snl;
                        //exit(0);//AK: Works cleanly. Will use later
                    }
                }
            }
            /* End of Random number generator for rolling dice */
        }

        //Define function call for board display
        println!();
        println! ("##############################################################################################");
        println!("UPDATED board is:");
        for n in 0..num_players {
            println!(
                "Player {:?}'s is @:{:?}",
                arr_players[n].avatar_display, arr_players[n].postion
            );
        }
        //println!("UPDATED board is:");
        println!("{:?}", border);
        for (i, row) in input.iter().enumerate() {
            for (j, mut col) in row.iter().enumerate() {
                for n in 0..num_players {
                    if i == row_num[n] && j == column[n] {
                        col = &mut arr_players[n].avatar;
                    }
                }
                print!("{}", col);
            }
            println!()
        }
        println!("{:?}", border);
    }
    /* Logic for Snake and Ladder ends */
}
