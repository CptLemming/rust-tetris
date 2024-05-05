use std::time::SystemTime;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::{config::{LEVEL_LINES, LEVEL_TIMES, TETRIS_HEIGHT}, tetriminos::*};

pub struct Tetris {
    pub game_map: Vec<Vec<u8>>,
    pub current_level: u32,
    pub score: u32,
    pub nb_lines: u32,
    pub current_piece: Option<Tetrimino>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }

        Tetris {
            game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    pub fn create_new_tetrimino(&self) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_num = rand::random::<u8>() % 7;
        if unsafe { PREV } == rand_num {
            rand_num = rand::random::<u8>() % 7;
        }
        unsafe { PREV = rand_num };
        match rand_num {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }

            if complete {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
          // A "tetris"
          score_add += 1000;
        }
        self.update_score(score_add);

        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn increase_line(&mut self) {
      self.nb_lines += 1;
      if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
        self.current_level += 1;
      }
    }

    pub fn make_permanent(&mut self) {
      let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len()
                && piece.y + shift_y < self.game_map.len()
            {
                let mut shift_x = 0;

                while shift_x < piece.states[piece.current_state as usize][shift_y].len()
                    && (piece.x + shift_x as isize)
                        < self.game_map[piece.y + shift_y].len() as isize
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.current_level
        }

        self.update_score(to_add);
        self.check_lines();
        self.current_piece = None;
    }

    pub fn update_score(&mut self, to_add: u32) {
      self.score += to_add;
    }

    pub fn is_time_over(&self, timer: &mut SystemTime) -> bool {
      match timer.elapsed() {
        Ok(elapsed) => {
          let millis = elapsed.as_millis() as u32;
          millis > LEVEL_TIMES[self.current_level as usize - 1]
        }
        Err(_) => false,
      }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, textures: &[Texture<'_>; 8], grid_x: i32, grid_y: i32) {
        for (line_nb, line) in self.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue
                }

                canvas.copy(
                    &textures[*case as usize - 1],
                    None,
                    Rect::new(
                        grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                        grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw grid");

                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                        grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                        grid_y + line_nb as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");
        
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                        grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + case_nb as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
            }
        }
    }
}
