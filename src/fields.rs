use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use eframe::egui;
use egui_extras::RetainedImage;

use crate::{
    field::Field, field_generator::FieldGenerator, get_mine_amount, get_size, position::Position,
    seven_segment_number::SevenSegmentNumber, tile::Tile, visibility::Visibility, won::Won,
    BORDER_HEIGHT, BORDER_WIDTH, FIELD_SIZE, NUMBER_HEIGHT, NUMBER_MARGIN,
};

pub(crate) struct Fields<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    fields: Vec<Field>,
    the_field_generator_phantom: PhantomData<TheFieldGenerator>,

    pub(crate) mines: SevenSegmentNumber,
    revealed: usize,

    pub(crate) won: Won,
}

impl<TheFieldGenerator> Fields<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    pub(crate) fn new(dont_surround: &Position) -> Self {
        Self {
            fields: TheFieldGenerator::generate(dont_surround),
            the_field_generator_phantom: PhantomData,

            mines: SevenSegmentNumber::new(*get_mine_amount() as i32),
            revealed: 0,

            won: Won::None,
        }
    }

    pub(crate) fn reveal(&mut self, position: &Position, first: bool) -> Option<bool> {
        match self[position].get_visibility() {
            Visibility::Closed => {
                self[position].open();
                self.revealed += 1;

                match self[position].get_tile() {
                    Tile::Empty => {
                        for neighbor in position.neighbors() {
                            self.reveal(&neighbor, false);
                        }
                    }
                    Tile::Mine => {
                        self.won.lose(*position);
                        for field in &mut self.fields {
                            if field.is_mine() && !field.is_flagged() {
                                field.open();
                            }
                        }
                    }
                    Tile::Number(_) => {}
                }

                Some(false)
            }
            Visibility::Open => {
                if first {
                    if let Tile::Number(num) = self[position].get_tile() {
                        let mut flagged = 0;
                        let mut closed = false;

                        for neighbor in position.neighbors() {
                            if self[&neighbor].is_flagged() {
                                flagged += 1;
                            } else if self[&neighbor].is_closed() {
                                closed = true;
                            }
                        }

                        if closed && num == flagged {
                            for neighbor in position.neighbors() {
                                if self[&neighbor].is_closed() {
                                    self.reveal(&neighbor, false);
                                }
                            }

                            return Some(true);
                        }
                    }
                }

                None
            }
            Visibility::Flagged => None,
        }
    }

    pub(crate) fn flag(&mut self, position: &Position) -> Option<bool> {
        match self[position].get_visibility() {
            Visibility::Closed => {
                self[position].flag();
                self.mines -= 1;
                Some(true)
            }
            Visibility::Flagged => {
                self[position].close();
                self.mines += 1;
                Some(false)
            }
            Visibility::Open => {
                if let Tile::Number(num) = self[position].get_tile() {
                    let mut not_open = 0;
                    let mut any_closed = false;

                    for neighbor in position.neighbors() {
                        if !self[&neighbor].is_open() {
                            not_open += 1;
                        }
                        if self[&neighbor].is_closed() {
                            any_closed = true;
                        }
                    }

                    if any_closed && num == not_open {
                        for neighbor in position.neighbors() {
                            if self[&neighbor].is_closed() {
                                self.flag(&neighbor);
                            }
                        }

                        return Some(true);
                    }
                }

                None
            }
        }
    }

    pub(crate) fn check_won(&mut self) {
        if self.won.still_playing()
            && self.revealed == get_size().0 * get_size().1 - *get_mine_amount()
        {
            self.won.win();
        }
    }

    pub(crate) fn get_fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub(crate) fn draw(&self, field_textures: &Vec<RetainedImage>, ui: &mut egui::Ui) {
        for x in 0..get_size().0 {
            for y in 0..get_size().1 {
                match self.won {
                    Won::Lost(pos) if pos.x == x && pos.y == y => {
                        Field::draw_red_mine(
                            BORDER_WIDTH + x as f32 * FIELD_SIZE,
                            BORDER_HEIGHT * 2.0
                                + NUMBER_MARGIN * 2.0
                                + NUMBER_HEIGHT
                                + y as f32 * FIELD_SIZE,
                            field_textures,
                            ui,
                        );
                    }
                    Won::Lost(_)
                        if !self.fields[y * get_size().0 + x].is_mine()
                            && self.fields[y * get_size().0 + x].is_flagged() =>
                    {
                        Field::draw_wrong_flag(
                            BORDER_WIDTH + x as f32 * FIELD_SIZE,
                            BORDER_HEIGHT * 2.0
                                + NUMBER_MARGIN * 2.0
                                + NUMBER_HEIGHT
                                + y as f32 * FIELD_SIZE,
                            field_textures,
                            ui,
                        );
                    }
                    _ => self.fields[y * get_size().0 + x].draw(
                        BORDER_WIDTH + x as f32 * FIELD_SIZE,
                        BORDER_HEIGHT * 2.0
                            + NUMBER_MARGIN * 2.0
                            + NUMBER_HEIGHT
                            + y as f32 * FIELD_SIZE,
                        field_textures,
                        ui,
                    ),
                }
            }
        }
    }

    pub(crate) fn draw_empty(field_textures: &Vec<RetainedImage>, ui: &mut egui::Ui) {
        for x in 0..get_size().0 {
            for y in 0..get_size().1 {
                Field::draw_empty(
                    BORDER_WIDTH + x as f32 * FIELD_SIZE,
                    BORDER_HEIGHT * 2.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + y as f32 * FIELD_SIZE,
                    field_textures,
                    ui,
                );
            }
        }
    }
}

impl<TheFieldGenerator> Index<&Position> for Fields<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    type Output = Field;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.fields[index.index()]
    }
}

impl<TheFieldGenerator> IndexMut<&Position> for Fields<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.fields[index.index()]
    }
}
