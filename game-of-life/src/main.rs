mod state;

use std::env;
use std::{thread::sleep, time};

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    match env::args().nth(1) {
        Some(t) => from_file(&t),
        None => random_life(),
    }
}

fn from_file(file_path: &str) {
    let mut board = state::load_board_state(file_path);
    loop {
        clear();
        state::render(&board);
        board = state::next_board_state(board);
        sleep(time::Duration::from_millis(150));
    }
}

fn random_life() {
    let mut board = state::random_state(50, 10);
    loop {
        clear();
        state::render(&board);
        board = state::next_board_state(board);
        sleep(time::Duration::from_millis(150));
    }
}
