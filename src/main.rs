extern crate sdl2;
extern crate rand;

use game::Tetris;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crate::config::TETRIS_HEIGHT;

mod tetriminos;
mod game;
mod config;


fn main() {
    let sdl_context = sdl2::init().expect("SDL init failed");
    let video_subsystem = sdl_context.video().expect("SDL video failed");

    let ttf_context = sdl2::ttf::init().expect("SDL TTF failed");

    let mut font = ttf_context.load_font("assets/Roboto-Regular.ttf", 128).expect("Failed loading font");
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let width = 600;
    let height = 800;

    let mut event_pump = sdl_context.event_pump().expect("SDL event pump failed");

    let grid_x = 20;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

    let window = video_subsystem
        .window("tetris", width, height)
        .position_centered()
        .opengl()
        .build()
        .expect("SDL window failed");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("SDL canvas failed");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(
        &mut canvas,
        &texture_creator,
        Color::RGB(0, 0, 0),
        TETRIS_HEIGHT as u32 * 10,
        TETRIS_HEIGHT as u32 * 16,
    ).expect("Failed to create grid");

    let border = create_texture_rect(
        &mut canvas,
        &texture_creator,
        Color::RGB(255, 255, 255),
        TETRIS_HEIGHT as u32 * 10 + 20,
        TETRIS_HEIGHT as u32 * 16 + 20,
    ).expect("Failed to create border");

    let preview = create_texture_rect(
        &mut canvas,
        &texture_creator,
        Color::RGB(0, 0, 0),
        TETRIS_HEIGHT as u32 * 4,
        TETRIS_HEIGHT as u32 * 4,
    ).expect("Failed to create preview");

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => (
            create_texture_rect(
                &mut canvas,
                &texture_creator,
                Color::RGB($r, $g, $b),
                TETRIS_HEIGHT as u32,
                TETRIS_HEIGHT as u32,
            ).unwrap()
        )
    }

    let textures = [
        texture!(255, 69, 69),
        texture!(255, 220, 69),
        texture!(237, 150, 37),
        texture!(171, 99, 237),
        texture!(77, 149, 239),
        texture!(39, 218, 225),
        texture!(45, 216, 47),
        texture!(0, 0, 0),
    ];
    
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();
    let mut next_piece = tetris.create_new_tetrimino();

    loop {
        if tetris.is_time_over(&mut timer) {
            let mut make_permanent = false;

            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;

                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas.copy(
            &border,
            None,
            Rect::new(
                10,
                (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 - 10,
                TETRIS_HEIGHT as u32 * 10 + 20,
                TETRIS_HEIGHT as u32 * 16 + 20,
            )
        ).expect("Failed to copy border");
        canvas.copy(
            &grid,
            None,
            Rect::new(
                20,
                (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2,
                TETRIS_HEIGHT as u32 * 10,
                TETRIS_HEIGHT as u32 * 16,
            )
        ).expect("Failed to copy grid");
        canvas.copy(
            &preview,
            None,
            Rect::new(
                grid_x + TETRIS_HEIGHT as i32 * 10 + 20,
                height as i32 / 2,
                TETRIS_HEIGHT as u32 * 4,
                TETRIS_HEIGHT as u32 * 4,
            )
        ).expect("Failed to copy preview");
        
        display_game_information(&tetris, &mut canvas, &texture_creator, &font, grid_x + TETRIS_HEIGHT as i32 * 10 + 20);

        if tetris.current_piece.is_none() {
            let current_piece = next_piece;
            next_piece = tetris.create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break;
            }
            tetris.current_piece = Some(current_piece);
        }

        next_piece.preview(&mut canvas, &textures, grid_x, height);

        tetris.draw(&mut canvas, &textures, grid_x, grid_y);

        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                piece.draw(&mut canvas, &textures, grid_x, grid_y);
            }
        }

        canvas.present();

        if quit {
            print_game_information(&tetris);
            break;
        }

        sleep(Duration::from_millis(1000 / 60));
    }
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: Color,
    width: u32,
    height: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(color);
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn print_game_information(tetris: &Tetris) {
    println!("Game over...");
    println!("Score: {}", tetris.score);
    println!("Lines: {}", tetris.nb_lines);
    println!("Current level: {}", tetris.current_level);
}

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump) -> bool {
    let mut make_permanent = false;

    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tmp_x -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let x = piece.x;
                    let mut y = piece.y;

                    while piece.change_position(&tetris.game_map, x, y + 1) {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }

        if !make_permanent {
            if !piece.change_position(&tetris.game_map, tmp_x, tmp_y) && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    } else {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break;
                }
                _ => {}
            }
        }
    }
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>, font: &sdl2::ttf::Font, text: &str, color: Color) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(color) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 10, 30))
}

fn display_game_information<'a>(tetris: &Tetris, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, font: &sdl2::ttf::Font, start_x_point: i32) {
    let score_text = format!("Score: {}", tetris.score);
    let lines_sent_text = format!("Lines: {}", tetris.nb_lines);
    let level_text = format!("Level: {}", tetris.current_level);

    let score = create_texture_from_text(&texture_creator, &font, &score_text, Color::RGB(255, 255, 255)).expect("Failed text");
    let lines = create_texture_from_text(&texture_creator, &font, &lines_sent_text, Color::RGB(255, 255, 255)).expect("Failed text");
    let level = create_texture_from_text(&texture_creator, &font, &level_text, Color::RGB(255, 255, 255)).expect("Failed text");

    canvas.copy(&score, None, get_rect_from_text(&score_text, start_x_point, 90)).expect("Cannot render text");
    canvas.copy(&lines, None, get_rect_from_text(&lines_sent_text, start_x_point, 125)).expect("Cannot render text");
    canvas.copy(&level, None, get_rect_from_text(&level_text, start_x_point, 160)).expect("Cannot render text");
}
