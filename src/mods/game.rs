extern crate rand;
extern crate sdl2;

use crate::GameCanvas;
use crate::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameStatus {
    Lost,
    Won,
    Playing,
}

pub struct Game {
    window_width: u32,
    window_height: u32,

    game_width: u32,
    game_height: u32,

    game_square_dimensions: (u32, u32),
    game_square_border_percentage: (f32, f32),

    game_fields_array: Vec<Field>,
    game_mines_count: u32,

    // First one for border, second for filling, 3rd is special for pointer's text
    game_mine_color: [Color; 2],
    game_revealed_color: [Color; 2],
    game_unrevealed_color: [Color; 2],
    game_marked_color: [Color; 2],
    game_pointer_color: [Color; 3],

    game_status: GameStatus,
}

impl Game {
    pub fn new(
        window_width: u32,
        window_height: u32,
        game_width: u32,
        game_height: u32,
        game_square_border_percentage: (f32, f32),
        game_mines_count: u32,
    ) -> Game {
        let mut game = Game {
            game_status: GameStatus::Playing,
            window_width,
            window_height,
            game_width,
            game_height,
            game_square_dimensions: (window_width / game_width, window_height / game_height),
            game_square_border_percentage,
            game_fields_array: Vec::with_capacity(game_width as usize * game_height as usize),
            game_mines_count,
            game_mine_color: [Color::from((128, 0, 0)), Color::from((184, 0, 0))],
            game_unrevealed_color: [Color::from((32, 32, 32)), Color::from((64, 64, 64))],
            game_marked_color: [Color::from((102, 0, 51)), Color::from((255, 0, 102))],
            game_pointer_color: [
                Color::from((0, 0, 153)),
                Color::from((0, 51, 204)),
                Color::from((51, 204, 255)),
            ],
            game_revealed_color: [Color::from((0, 0, 0)), Color::from((0, 0, 0))],
        };

        game.setup_mines();
        game.setup_pointers();
        game
    }

    pub fn from_params(params: (u32, u32, u32, u32, (f32, f32), u32)) -> Game {
        Game::new(params.0, params.1, params.2, params.3, params.4, params.5)
    }

    pub fn get_window_width(&self) -> &u32 {
        &self.window_width
    }
    pub fn get_window_height(&self) -> &u32 {
        &self.window_height
    }
    pub fn get_game_square_dimensions(&self) -> &(u32, u32) {
        &self.game_square_dimensions
    }

    pub fn get_game_square_border_percentage(&self) -> &(f32, f32) {
        &self.game_square_border_percentage
    }

    fn arr2d_arr1d(&self, x: i32, y: i32) -> usize {
        (x + y * self.game_width as i32) as usize
    }

    fn setup_mines(&mut self) {
        let mut empty_fields: Vec<usize> = Vec::with_capacity(self.game_fields_array.len());
        for i in 0..self.game_width as usize * self.game_height as usize {
            self.game_fields_array.push(Field {
                field_status: FieldStatus::Unrevealed,
                field_type: FieldType::Empty,
                is_marked: false,
            });
            empty_fields.push(i);
        }

        for _ in 0..self.game_mines_count {
            let index = ((rand::random::<u64>() / 160u64 + empty_fields.len() as u64 * 37u64)
                % empty_fields.len() as u64) as usize;
            self.game_fields_array[empty_fields[index]].field_type = FieldType::Mine;
            empty_fields.remove(index);
        }
    }

    fn setup_pointers(&mut self) {
        for x in 0..self.game_width as i32 {
            for y in 0..self.game_height as i32 {
                let middle_cell_index_index = self.get_cell_index(x, y);

                if middle_cell_index_index.is_none() {
                    continue;
                }

                if self.game_fields_array[middle_cell_index_index.unwrap()].field_type
                    == FieldType::Mine
                {
                    let surrounding_cell_indexs = [
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y),
                        (x + 1, y),
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1),
                    ];

                    for field in surrounding_cell_indexs.iter() {
                        let index = self.get_cell_index(field.0, field.1);

                        if let Some(i) = index {
                            match self.game_fields_array[i].field_type {
                                FieldType::Empty => {
                                    self.game_fields_array[i].field_type =
                                        FieldType::Pointer { mines_nearby: 1 };
                                }
                                FieldType::Pointer { mines_nearby } => {
                                    self.game_fields_array[i].field_type = FieldType::Pointer {
                                        mines_nearby: mines_nearby + 1,
                                    };
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        font: &sdl2::ttf::Font,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) {
        canvas.set_draw_color(Color::from((0u8, 0u8, 0u8)));
        canvas.clear();

        for x in 0..self.game_width {
            for y in 0..self.game_height {
                let element = self.game_fields_array[self.arr2d_arr1d(x as i32, y as i32)];

                match element.field_status {
                    FieldStatus::Unrevealed => {
                        if element.is_marked {
                            canvas.draw_square(
                                x,
                                y,
                                &self,
                                self.game_marked_color[0],
                                self.game_marked_color[1],
                            );
                        } else {
                            canvas.draw_square(
                                x,
                                y,
                                &self,
                                self.game_unrevealed_color[0],
                                self.game_unrevealed_color[1],
                            )
                        }
                    }
                    FieldStatus::Revealed => match element.field_type {
                        FieldType::Empty => {
                            canvas.draw_square(
                                x,
                                y,
                                &self,
                                self.game_revealed_color[0],
                                self.game_revealed_color[1],
                            );
                        }
                        FieldType::Mine => {
                            canvas.draw_square(
                                x,
                                y,
                                &self,
                                self.game_mine_color[0],
                                self.game_mine_color[1],
                            );
                        }
                        FieldType::Pointer { mines_nearby } => {
                            canvas.draw_square(
                                x,
                                y,
                                &self,
                                self.game_pointer_color[0],
                                self.game_pointer_color[1],
                            );
                            let inner_rect = Rect::new(
                                (self.game_square_border_percentage.0
                                    * self.game_square_dimensions.0 as f32)
                                    as i32
                                    + x as i32 * self.game_square_dimensions.0 as i32,
                                (self.game_square_border_percentage.1
                                    * self.game_square_dimensions.1 as f32)
                                    as i32
                                    + y as i32 * self.game_square_dimensions.1 as i32,
                                self.game_square_dimensions.0
                                    - (self.game_square_border_percentage.0
                                        * self.game_square_dimensions.0 as f32
                                        * 2f32) as u32,
                                self.game_square_dimensions.1
                                    - (self.game_square_border_percentage.1
                                        * self.game_square_dimensions.1 as f32
                                        * 2f32) as u32,
                            );
                            canvas.draw_text(
                                inner_rect,
                                font,
                                texture_creator,
                                self.game_pointer_color[2],
                                mines_nearby.to_string().as_str(),
                            )
                        }
                    },
                }
            }
        }
        match self.game_status {
            GameStatus::Lost => {
                canvas.draw_text(
                    Rect::from((0, 0, self.window_width, self.window_height / 2)),
                    font,
                    texture_creator,
                    Color::from((255u8, 255u8, 255u8)),
                    "Defeat",
                );
                canvas.draw_text(
                    Rect::from((
                        0,
                        self.window_height as i32 / 2i32,
                        self.window_width,
                        self.window_height / 4,
                    )),
                    font,
                    texture_creator,
                    Color::from((255u8, 255u8, 255u8)),
                    "Press R to restart the game.",
                );
            }

            GameStatus::Won => {
                canvas.draw_text(
                    Rect::from((0, 0, self.window_width, self.window_height)),
                    font,
                    texture_creator,
                    Color::from((255u8, 255u8, 255u8)),
                    "Victory",
                );
            }
            _ => {}
        }

        canvas.present();
    }

    fn get_cell_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x as u32 > self.game_width - 1 || y as u32 > self.game_height - 1 {
            None
        } else {
            Some(self.arr2d_arr1d(x, y))
        }
    }

    fn flood_reveal(&mut self, x: i32, y: i32) {
        let surrounding_cell_indexes = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let cell_index = self.get_cell_index(x, y);

        if let Some(c) = cell_index {
            if self.game_fields_array[c].field_status == FieldStatus::Revealed {
                return;
            } else {
                self.game_fields_array[c].field_status = FieldStatus::Revealed;
                match self.game_fields_array[c].field_type {
                    FieldType::Pointer { mines_nearby } => {
                        let mut marked_around = 0;
                        for cell_index in surrounding_cell_indexes.iter() {
                            if let Some(index) = self.get_cell_index(cell_index.0, cell_index.1) {
                                if self.game_fields_array[index].is_marked {
                                    marked_around += 1;
                                }
                            }
                        }

                        if marked_around != 0 {
                            if marked_around == mines_nearby {
                                for cell_index in surrounding_cell_indexes.iter() {
                                    if let Some(index) =
                                        self.get_cell_index(cell_index.0, cell_index.1)
                                    {
                                        if !self.game_fields_array[index].is_marked {
                                            self.flood_reveal(cell_index.0, cell_index.1);
                                        }
                                    }
                                }
                            }
                        }
                        return;
                    }
                    _ => {}
                }
            }
        } else {
            return;
        }

        for field in surrounding_cell_indexes.iter() {
            self.flood_reveal(field.0, field.1);
        }
    }

    fn mark_field(&mut self, x: i32, y: i32) {
        let index = self.get_cell_index(x, y);
        if let Some(i) = index {
            if self.game_fields_array[i].field_status == FieldStatus::Revealed {
                return;
            }

            self.game_fields_array[i].is_marked = !self.game_fields_array[i].is_marked;
        }
    }

    pub fn click(
        &mut self,
        x: i32,
        y: i32,
        button: sdl2::mouse::MouseButton,
        canvas: &mut Canvas<Window>,
        font: &sdl2::ttf::Font,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) {
        if self.game_status != GameStatus::Playing {
            return;
        }

        let cell_index_click: (i32, i32) = (
            x / self.game_square_dimensions.0 as i32,
            y / self.game_square_dimensions.1 as i32,
        );

        if button == sdl2::mouse::MouseButton::Left {
            self.flood_reveal(cell_index_click.0, cell_index_click.1);
        } else if button == sdl2::mouse::MouseButton::Right {
            self.mark_field(cell_index_click.0, cell_index_click.1);
        }

        self.check_winning_conditions(x, y, button);
        self.render(canvas, font, texture_creator);
    }

    pub fn set_all_visible(&mut self) {
        for item in self.game_fields_array.iter_mut() {
            item.field_status = FieldStatus::Revealed;
        }
    }

    fn check_winning_conditions(
        &mut self,
        x_click: i32,
        y_click: i32,
        button: sdl2::mouse::MouseButton,
    ) {
        // if loosing
        if button == sdl2::mouse::MouseButton::Left
            && self.game_fields_array[self
                .get_cell_index(
                    x_click / self.game_square_dimensions.0 as i32,
                    y_click / self.game_square_dimensions.1 as i32,
                )
                .unwrap()]
            .field_type
                == FieldType::Mine
        {
            // Reveal all mines
            for item in self.game_fields_array.iter_mut() {
                if item.field_type == FieldType::Mine {
                    item.field_status = FieldStatus::Revealed;
                }
            }

            self.game_status = GameStatus::Lost;
        } else {
            let mut marked_mines = 0;
            for item in self.game_fields_array.iter() {
                if item.field_type == FieldType::Mine && item.is_marked {
                    marked_mines += 1;
                }
            }

            if marked_mines == self.game_mines_count {
                self.game_status = GameStatus::Won;
            }
        }
    }
}
