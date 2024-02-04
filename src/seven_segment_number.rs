use std::ops::{AddAssign, SubAssign};

use eframe::{
    egui::{self, Image, Ui},
    epaint::Rect,
};
use egui_extras::RetainedImage;

use crate::{BORDER_HEIGHT, FIELD_SIZE, NUMBER_HEIGHT, NUMBER_MARGIN, NUMBER_WIDTH};

#[derive(Clone, Copy)]
pub(crate) struct SevenSegmentNumber {
    pub(crate) number: i32,
}

impl SevenSegmentNumber {
    pub(crate) fn new(number: i32) -> Self {
        Self { number }
    }

    pub(crate) fn draw(
        &self,
        x: f32,
        y: f32,
        number_bg_texture: &RetainedImage,
        number_textures: &Vec<RetainedImage>,
        ui: &mut Ui,
    ) {
        Image::new(
            number_bg_texture.texture_id(ui.ctx()),
            number_bg_texture.size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(x, y),
                egui::vec2(FIELD_SIZE * 2.46, FIELD_SIZE * 1.5),
            ),
        );

        let mut num = self.number.clamp(-99, 999);
        let mut sign = num.signum();
        num = num.abs();

        for i in (0..3).rev() {
            if num == 0 && sign == -1 {
                sign = 0;
                Image::new(
                    number_textures[10].texture_id(ui.ctx()),
                    number_textures[10].size_vec2(),
                )
                .paint_at(
                    ui,
                    Rect::from_min_size(
                        egui::pos2(
                            x + (i as f32 * (NUMBER_WIDTH + NUMBER_MARGIN)) + NUMBER_MARGIN,
                            BORDER_HEIGHT + NUMBER_MARGIN,
                        ),
                        egui::vec2(NUMBER_WIDTH, NUMBER_HEIGHT),
                    ),
                );
            } else {
                Image::new(
                    number_textures[(num % 10) as usize].texture_id(ui.ctx()),
                    number_textures[(num % 10) as usize].size_vec2(),
                )
                .paint_at(
                    ui,
                    Rect::from_min_size(
                        egui::pos2(
                            x + (i as f32 * (NUMBER_WIDTH + NUMBER_MARGIN)) + NUMBER_MARGIN,
                            BORDER_HEIGHT + NUMBER_MARGIN,
                        ),
                        egui::vec2(NUMBER_WIDTH, NUMBER_HEIGHT),
                    ),
                );
            }

            num /= 10;
        }
    }
}

impl AddAssign<i32> for SevenSegmentNumber {
    fn add_assign(&mut self, rhs: i32) {
        self.number += rhs;
    }
}

impl SubAssign<i32> for SevenSegmentNumber {
    fn sub_assign(&mut self, rhs: i32) {
        self.number -= rhs;
    }
}
