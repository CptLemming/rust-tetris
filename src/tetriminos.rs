use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::config::TETRIS_HEIGHT;

pub type Piece = Vec<Vec<u8>>;
pub type States = Vec<Piece>;

#[derive(Debug)]
pub struct Tetrimino {
    pub states: States,
    pub x: isize,
    pub y: usize,
    pub current_state: u8,
}

impl Tetrimino {
    pub fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }

        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) {
                self.current_state = tmp_state;
                self.x += x;
                break;
            }
        }
    }

    pub fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
      if self.test_position(game_map, self.current_state as usize, new_x, new_y) {
        self.x = new_x as isize;
        self.y = new_y;
        true
      } else {
        false
      }
    }

    pub fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
      self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }

    pub fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0
                    && (y + decal_y >= game_map.len()
                        || x < 0
                        || x as usize >= game_map[y + decal_y].len()
                        || game_map[y + decal_y][x as usize] != 0)
                {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, textures: &[Texture<'_>; 8], grid_x: i32, grid_y: i32) {
        for (line_nb, line) in self.states[self.current_state as usize].iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue
                }

                canvas.copy(
                    &textures[*case as usize - 1],
                    None,
                    Rect::new(
                        grid_x + (self.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        grid_y + (self.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");

                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + (self.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        grid_y + (self.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + (self.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        grid_y + (self.y + line_nb) as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");
        
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + (self.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        grid_y + (self.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + (self.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        grid_y + (self.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
            }
        }
    }

    pub fn preview(&self, canvas: &mut Canvas<Window>, textures: &[Texture<'_>; 8], grid_x: i32, height: u32) {
        for (line_nb, line) in self.states[self.current_state as usize].iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue
                }

                canvas.copy(
                    &textures[*case as usize - 1],
                    None,
                    Rect::new(
                        grid_x + TETRIS_HEIGHT as i32 * 10 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                        height as i32 / 2 + line_nb as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");

                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + TETRIS_HEIGHT as i32 * 10 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                        height as i32 / 2 + line_nb as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + TETRIS_HEIGHT as i32 * 10 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                        height as i32 / 2 + line_nb as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        TETRIS_HEIGHT as u32,
                        2,
                    )
                ).expect("Failed to draw piece");

                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + TETRIS_HEIGHT as i32 * 10 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                        height as i32 / 2 + line_nb as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
                canvas.copy(
                    &textures[7],
                    None,
                    Rect::new(
                        grid_x + TETRIS_HEIGHT as i32 * 10 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32 + (TETRIS_HEIGHT as i32 - 2),
                        height as i32 / 2 + line_nb as i32 * TETRIS_HEIGHT as i32,
                        2,
                        TETRIS_HEIGHT as u32,
                    )
                ).expect("Failed to draw piece");
            }
        }
    }
}

pub trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

pub struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![
                vec![4, 4, 0, 0],
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
