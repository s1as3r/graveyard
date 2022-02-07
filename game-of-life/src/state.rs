use rand::{self, Rng};
use std::fs;

type Board = Vec<Vec<u8>>;

pub fn dead_state(width: usize, height: usize) -> Board {
    vec![vec![0; width]; height]
}

pub fn random_state(width: usize, height: usize) -> Board {
    let mut state = dead_state(width, height);
    let mut rng = rand::thread_rng();
    for y in 0..height {
        for x in 0..width {
            state[y][x] = rng.gen_range(0..=1);
        }
    }
    state
}

pub fn count_alive_neighbors(board: &Board, x: usize, y: usize) -> u8 {
    let height = board.len();
    let width = board[0].len();
    let mut count = 0;
    let mut counted = Vec::new();
    for j in (y.saturating_sub(1))..=(y.saturating_add(1)) {
        for i in (x.saturating_sub(1))..=(x.saturating_add(1)) {
            let i = i.clamp(0, width - 1);
            let j = j.clamp(0, height - 1);
            if !counted.contains(&(i, j)) {
                count += board[j][i];
                counted.push((i, j));
            }
        }
    }
    count - board[y][x]
}

pub fn next_board_state(board: Board) -> Board {
    let height = board.len();
    let width = board[0].len();
    let mut new_state = dead_state(width, height);

    for y in 0..height {
        for x in 0..width {
            let alive = count_alive_neighbors(&board, x, y);
            match alive {
                0 | 1 => new_state[y][x] = 0,
                2 => new_state[y][x] = board[y][x],
                3 => new_state[y][x] = 1,
                _ => new_state[y][x] = 0,
            }
        }
    }
    new_state
}

pub fn load_board_state(file_path: &str) -> Board {
    let raw_data = fs::read_to_string(file_path).unwrap();
    raw_data
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Board>()
}

pub fn render(board: &Board) {
    let width = board[0].len() + 2;
    println!("{}", "-".repeat(width));
    for row in board {
        print!("|");
        for &cell in row {
            if cell == 1 {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("|")
    }
    println!("{}", "-".repeat(width));
}

#[cfg(test)]
mod teste {
    use std::vec;

    use super::*;

    #[test]
    fn test_all_dead() {
        let state = dead_state(3, 3);
        assert_eq!(state, next_board_state(state.clone()))
    }

    #[test]
    fn test_one_alive() {
        let mut state = dead_state(3, 3);
        state[1][1] = 1;
        assert_eq!(dead_state(3, 3), next_board_state(state))
    }

    #[test]
    fn test_three_neighbors_alive() {
        #[rustfmt::skip]
        let initial_state = vec![
            vec![0, 0, 1],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];

        #[rustfmt::skip]
        let expected_next_state = vec![
            vec![0, 1, 1],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];

        assert_eq!(expected_next_state, next_board_state(initial_state))
    }
    #[test]
    fn test_death() {
        #[rustfmt::skip]
        let initial_state = vec![
            vec![1, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1]
        ];

        #[rustfmt::skip]
        let expected_next_state = vec![
            vec![1, 0, 1, 0],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 1],
        ];

        let s = next_board_state(initial_state);
        render(&s);
        assert_eq!(expected_next_state, s)
    }
}
