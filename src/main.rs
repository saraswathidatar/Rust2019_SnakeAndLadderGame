//! # Snake and Ladder Game
//!
//!  Copyright (c) 2019 Saraswathi Govind Datar NA
//This program is licensed under the "MIT License"
// Please see the file LICENSE in the source
// distribution of this software for license terms.


extern crate text_io;

use snakenladder::Snl;
use std::process::exit;

//main function for snake and ladders game
fn main() {
    println!("{}", "Let's PLay Snakes and Ladders");
    // This is used to display the board the first time
    let mut snl = Snl::new();
    snl.ramstart();
    exit(0);
}

