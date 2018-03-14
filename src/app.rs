extern crate rustyline;
use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use board;

pub struct Update();

impl Update {
    pub fn update(board: &mut board::Board) {
        let mut r1 = Editor::<()>::new();
        loop {
            if Update::is_finished(board) {
                break;
            }

            let readline = r1.readline("[w/a/s/d]: ");
            match readline {
                Ok(line) => {
                    match &*line.to_string() {
                        "w" => {
                            if Update::up(board) {
                                board.gen();
                            }
                        }
                        "s" => {
                            if Update::down(board) {
                                board.gen();
                            }
                        }
                        "a" => {
                            if Update::left(board) {
                                board.gen();
                            }
                        }
                        "d" => {
                            if Update::right(board) {
                                board.gen();
                            }
                        }
                        _ => {
                            println!("miss typing.");
                        }
                    }
                    board.render();
                    board.render_parameter_box();
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn up(board: &mut board::Board) -> bool {
        let mut moved = false;
        for re in 0..4 {
            for i in 0..3 {
                for v in 0..4 {
                    let t1 = board.tiles[4 * i + v].is_exist();
                    let t2 = board.tiles[4 * (i + 1) + v].is_exist();

                    if !t1 && t2 {
                        moved = true;
                        board.tiles[4 * i + v] = board.tiles[4 * (i + 1) + v].clone();
                        board.tiles[4 * (i + 1) + v] = board::Tile::new(2, board::Status::Nothing);
                    } else if t1 && t2 {
                        let t1_score = board.tiles[4 * i + v].get_score();
                        let t2_score = board.tiles[4 * (i + 1) + v].get_score();
                        if t1_score == t2_score {
                            moved = true;
                            if re == 3 {
                                board.tiles[4 * i + v] =
                                    board::Tile::new(t1_score * 2, board::Status::Exist);
                                board.tiles[4 * (i + 1) + v] =
                                    board::Tile::new(2, board::Status::Nothing);
                            }
                        }
                    }
                }
            }
        }
        moved
    }

    pub fn down(board: &mut board::Board) -> bool {
        let mut moved = false;
        for re in 0..4 {
            for v in 0..4 {
                for &i in &[2, 1, 0] {
                    let t1 = board.tiles[4 * (i + 1) + v].is_exist();
                    let t2 = board.tiles[4 * i + v].is_exist();

                    if !t1 && t2 {
                        moved = true;
                        board.tiles[4 * (i + 1) + v] = board.tiles[4 * i + v].clone();
                        board.tiles[4 * i + v] = board::Tile::new(2, board::Status::Nothing);
                    } else if t1 && t2 {
                        let t1_score = board.tiles[4 * (i + 1) + v].get_score();
                        let t2_score = board.tiles[4 * i + v].get_score();
                        if t1_score == t2_score {
                            moved = true;
                            if re == 3 {
                                board.tiles[4 * (i + 1) + v] =
                                    board::Tile::new(t1_score * 2, board::Status::Exist);
                                board.tiles[4 * i + v] =
                                    board::Tile::new(2, board::Status::Nothing);
                            }
                        }
                    }
                }
            }
        }
        moved
    }

    pub fn left(board: &mut board::Board) -> bool {
        let mut moved = false;
        for re in 0..4 {
            for v in 0..3 {
                for i in 0..4 {
                    let t1 = board.tiles[4 * i + v].is_exist();
                    let t2 = board.tiles[4 * i + v + 1].is_exist();

                    if !t1 && t2 {
                        moved = true;
                        board.tiles[4 * i + v] = board.tiles[4 * i + v + 1].clone();
                        board.tiles[4 * i + v + 1] = board::Tile::new(2, board::Status::Nothing);
                    } else if t1 && t2 {
                        let t1_score = board.tiles[4 * i + v].get_score();
                        let t2_score = board.tiles[4 * i + v + 1].get_score();
                        if t1_score == t2_score {
                            moved = true;
                            if re == 3 {
                                board.tiles[4 * i + v] =
                                    board::Tile::new(t1_score * 2, board::Status::Exist);
                                board.tiles[4 * i + v + 1] =
                                    board::Tile::new(2, board::Status::Nothing);
                            }
                        }
                    }
                }
            }
        }
        moved
    }

    pub fn right(board: &mut board::Board) -> bool {
        let mut moved = false;
        for re in 0..4 {
            for i in 0..4 {
                for &v in &[2, 1, 0] {
                    let t1 = board.tiles[4 * i + v + 1].is_exist();
                    let t2 = board.tiles[4 * i + v].is_exist();

                    if !t1 && t2 {
                        moved = true;
                        board.tiles[4 * i + v + 1] = board.tiles[4 * i + v].clone();
                        board.tiles[4 * i + v] = board::Tile::new(2, board::Status::Nothing);
                    } else if t1 && t2 {
                        let t1_score = board.tiles[4 * i + v + 1].get_score();
                        let t2_score = board.tiles[4 * i + v].get_score();
                        if t1_score == t2_score {
                            moved = true;
                            if re == 3 {
                                board.tiles[4 * i + v + 1] =
                                    board::Tile::new(t1_score * 2, board::Status::Exist);
                                board.tiles[4 * i + v] =
                                    board::Tile::new(2, board::Status::Nothing);
                            }
                        }
                    }
                }
            }
        }
        moved
    }

    pub fn is_finished(board: &board::Board) -> bool {
        for i in 0..16 {
            if board.get_tile(i).is_none() {
                return false;
            }
        }

        for i in 0..3 {
            for v in 0..4 {
                let t1_score = board.tiles[4 * i + v].get_score();
                let t2_score = board.tiles[4 * (i + 1) + v].get_score();
                if t1_score == t2_score {
                    return false;
                }
            }
        }

        for i in 0..4 {
            for v in 0..3 {
                let t1_score = board.tiles[4 * i + v].get_score();
                let t2_score = board.tiles[4 * i + v + 1].get_score();
                if t1_score == t2_score {
                    return false;
                }
            }
        }
        true
    }
}
