extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod mods;
use mods::field::*;
use mods::game::*;
use mods::game_canvas::GameCanvas;

const GAME_PARAMS : (u32, u32, u32, u32, (f32, f32), u32) = (800, 600, 20, 15, (1f32 / 4f32, 1f32 / 4f32), 50);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let font_context = sdl2::ttf::init().unwrap();

    let mut font = font_context.load_font("./PxPlus_AmstradPC1512.ttf", 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    
    let mut game = Game::from_params(GAME_PARAMS);

    let window = video_subsystem
        .window(
            "Minesweeper-Rust",
            *game.get_window_width(),
            *game.get_window_height(),
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    game.render(&mut canvas, &font, &texture_creator);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    game.click(x, y, mouse_btn, &mut canvas, &font, &texture_creator);
                }
                Event::Window { .. } => {
                    game.render(&mut canvas, &font, &texture_creator);
                }
                Event::KeyUp {keycode, ..} => {
                    if let Some(k) = keycode {
                        if k == sdl2::keyboard::Keycode::Q {
                            game.set_all_visible();
                        } else if k == sdl2::keyboard::Keycode::R {
                            game = Game::from_params(GAME_PARAMS);
                            game.render(&mut canvas, &font, &texture_creator);
                        }
                    }
                }
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
}
