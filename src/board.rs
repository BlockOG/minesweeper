pub(crate) mod play_board;
pub(crate) mod replay_board;

use std::marker::PhantomData;

use eframe::{
    egui,
    epaint::{Color32, Rect, Rounding},
};

use crate::{
    border::Border,
    field_generator::FieldGenerator,
    fields::Fields,
    get_mine_amount, get_size, get_sound_player,
    position::Position,
    seven_segment_number::SevenSegmentNumber,
    sounds::{SoundPlayer, Sounds},
    textures::Textures,
    time::Time,
    won::Won,
    BORDER_HEIGHT, BORDER_WIDTH, FIELD_SIZE,
};

pub(crate) struct Board<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    pub(crate) time: Time,

    pub(crate) fields: Option<Fields<TheFieldGenerator>>,
    the_field_generator_phantom: PhantomData<TheFieldGenerator>,

    border: Border,
    textures: Textures,
    sound_player: &'static mut SoundPlayer,
}

impl<TheFieldGenerator> Board<TheFieldGenerator>
where
    TheFieldGenerator: FieldGenerator,
{
    pub(crate) fn new() -> Self {
        Self {
            time: Time::None,

            fields: None,
            the_field_generator_phantom: PhantomData,

            border: Border::new(),
            textures: Textures::new(),
            sound_player: get_sound_player(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.time.reset();
        self.fields = None;
    }

    pub(crate) fn still_playing(&self) -> bool {
        if let Some(fields) = &self.fields {
            fields.won.still_playing()
        } else {
            true
        }
    }

    pub(crate) fn reveal(&mut self, position: &Position, play_sound: bool) {
        if let Some(fields) = &self.fields {
            if !fields.won.still_playing() {
                return;
            }
        }

        if let Some(opened_from_flags) = self
            .fields
            .get_or_insert_with(|| {
                self.time.start();
                Fields::<TheFieldGenerator>::new(position)
            })
            .reveal(position, true)
        {
            if play_sound {
                self.sound_player.play(if opened_from_flags {
                    Sounds::OpenFromFlags
                } else {
                    Sounds::Open
                });
            }
        }

        self.fields.as_mut().unwrap().check_won();
        match self.fields.as_ref().unwrap().won {
            Won::None => {}
            Won::Won => {
                if play_sound {
                    self.sound_player.play(Sounds::Win);
                }
                self.time.end();
            }
            Won::Lost(_) => {
                if play_sound {
                    self.sound_player.play(Sounds::Lose);
                }
                self.time.end();
            }
        }
    }

    pub(crate) fn flag(&mut self, position: &Position, play_sound: bool) {
        if let Some(fields) = &mut self.fields {
            if !fields.won.still_playing() {
                return;
            }

            if let Some(flagged) = fields.flag(position) {
                if play_sound {
                    self.sound_player.play(if flagged {
                        Sounds::Flag
                    } else {
                        Sounds::Unflag
                    });
                }
            }
        }
    }

    pub(crate) fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            Rect::EVERYTHING,
            Rounding::none(),
            Color32::from_rgb(192, 192, 192),
        );

        let Textures {
            field_textures,
            number_textures,
            number_bg_texture,
        } = &self.textures;

        match &self.fields {
            Some(fields) => fields.draw(field_textures, ui),
            None => Fields::<TheFieldGenerator>::draw_empty(field_textures, ui),
        }
        self.border.draw(ui);

        self.fields
            .as_ref()
            .map(|fields| fields.mines)
            .unwrap_or_else(|| SevenSegmentNumber::new(*get_mine_amount() as i32))
            .draw(
                BORDER_WIDTH - 1.0,
                BORDER_HEIGHT,
                number_bg_texture,
                number_textures,
                ui,
            );

        SevenSegmentNumber::new(self.time.duration().as_secs() as i32).draw(
            BORDER_WIDTH + FIELD_SIZE * get_size().0 as f32 - number_bg_texture.width() as f32,
            BORDER_HEIGHT,
            number_bg_texture,
            number_textures,
            ui,
        );
    }
}
