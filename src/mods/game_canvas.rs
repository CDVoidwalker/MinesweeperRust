extern crate sdl2;
use crate::Game;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait GameCanvas {
    fn draw_square(&mut self, x: u32, y: u32, game: &Game, bg_color: Color, fg_color: Color);
    fn draw_text(
        &mut self,
        rect: Rect,
        font: &sdl2::ttf::Font,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        color: Color,
        text: &str,
    );
}
/*                                let surface = font.render(mines_nearby.to_string().as_str()).solid(c).unwrap();
let texture = canvas.texture_creator().create_texture_from_surface(surface);
canvas.copy(texture, None, ) */

impl GameCanvas for Canvas<Window> {
    fn draw_square(&mut self, x: u32, y: u32, game: &Game, bg_color: Color, fg_color: Color) {
        let border_percentage = game.get_game_square_border_percentage();
        let rect_dimensions = game.get_game_square_dimensions();
        let outer_rect = Rect::new(
            x as i32 * rect_dimensions.0 as i32,
            y as i32 * rect_dimensions.1 as i32,
            rect_dimensions.0,
            rect_dimensions.1,
        );
        let inner_rect = Rect::new(
            (border_percentage.0 * rect_dimensions.0 as f32) as i32
                + x as i32 * rect_dimensions.0 as i32,
            (border_percentage.1 * rect_dimensions.1 as f32) as i32
                + y as i32 * rect_dimensions.1 as i32,
            rect_dimensions.0 - (border_percentage.0 * rect_dimensions.0 as f32 * 2f32) as u32,
            rect_dimensions.1 - (border_percentage.1 * rect_dimensions.1 as f32 * 2f32) as u32,
        );
        // bigger rect
        self.set_draw_color(bg_color);
        self.fill_rect(outer_rect);
        
        // smaller rect
        self.set_draw_color(fg_color);
        self.fill_rect(inner_rect);
    }
    fn draw_text(
        &mut self,
        rect : Rect,
        font: &sdl2::ttf::Font,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        color: Color,
        text: &str,
    ) {
        let surface = font
            .render(text)
            .solid(color)
            .unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        self.copy(&texture, None, Some(rect));
    }
}
