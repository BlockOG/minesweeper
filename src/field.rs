use eframe::{
    egui::{self, Image},
    epaint::Rect,
};
use egui_extras::RetainedImage;

use crate::{tile::Tile, visibility::Visibility, FIELD_SIZE};

#[derive(Clone, Copy)]
pub(crate) struct Field {
    tile: Tile,
    visibility: Visibility,
}

impl Field {
    pub(crate) fn new() -> Self {
        Self {
            tile: Tile::Empty,
            visibility: Visibility::Closed,
        }
    }

    pub(crate) fn get_tile(&self) -> Tile {
        self.tile
    }

    pub(crate) fn get_visibility(&self) -> Visibility {
        self.visibility
    }

    /* #region Tile checks */
    pub(crate) fn is_mine(&self) -> bool {
        matches!(self.tile, Tile::Mine)
    }
    /* #endregion */

    /* #region Tile changes */
    pub(crate) fn set_number(&mut self, number: usize) {
        if number <= 0 {
            self.tile = Tile::Empty;
        } else {
            self.tile = Tile::Number(number.into());
        }
    }

    pub(crate) fn set_mine(&mut self) {
        self.tile = Tile::Mine;
    }
    /* #endregion */

    /* #region Visibility checks */
    pub(crate) fn is_open(&self) -> bool {
        matches!(self.visibility, Visibility::Open)
    }

    pub(crate) fn is_closed(&self) -> bool {
        matches!(self.visibility, Visibility::Closed)
    }

    pub(crate) fn is_flagged(&self) -> bool {
        matches!(self.visibility, Visibility::Flagged)
    }
    /* #endregion */

    /* #region Visiblity changes */
    pub(crate) fn open(&mut self) {
        self.visibility = Visibility::Open;
    }

    pub(crate) fn close(&mut self) {
        self.visibility = Visibility::Closed;
    }

    pub(crate) fn flag(&mut self) {
        self.visibility = Visibility::Flagged;
    }
    /* #endregion */

    pub(crate) fn draw(
        &self,
        x: f32,
        y: f32,
        field_textures: &Vec<RetainedImage>,
        ui: &mut egui::Ui,
    ) {
        let texture = match (self.tile, self.visibility) {
            (_, Visibility::Closed) => field_textures[0].texture_id(ui.ctx()),
            (_, Visibility::Flagged) => field_textures[3].texture_id(ui.ctx()),

            (Tile::Empty, Visibility::Open) => field_textures[1].texture_id(ui.ctx()),
            (Tile::Mine, Visibility::Open) => field_textures[2].texture_id(ui.ctx()),
            (Tile::Number(num), Visibility::Open) => {
                field_textures[5 + usize::from(num)].texture_id(ui.ctx())
            }
        };

        Image::new(texture, egui::vec2(FIELD_SIZE, FIELD_SIZE)).paint_at(
            ui,
            Rect::from_min_size(egui::pos2(x, y), egui::vec2(FIELD_SIZE, FIELD_SIZE)),
        );
    }

    pub(crate) fn draw_empty(
        x: f32,
        y: f32,
        field_textures: &Vec<RetainedImage>,
        ui: &mut egui::Ui,
    ) {
        Image::new(
            field_textures[0].texture_id(ui.ctx()),
            field_textures[0].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(egui::pos2(x, y), egui::vec2(FIELD_SIZE, FIELD_SIZE)),
        );
    }

    pub(crate) fn draw_red_mine(
        x: f32,
        y: f32,
        field_textures: &Vec<RetainedImage>,
        ui: &mut egui::Ui,
    ) {
        Image::new(
            field_textures[4].texture_id(ui.ctx()),
            field_textures[4].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(egui::pos2(x, y), egui::vec2(FIELD_SIZE, FIELD_SIZE)),
        );
    }

    pub(crate) fn draw_wrong_flag(
        x: f32,
        y: f32,
        field_textures: &Vec<RetainedImage>,
        ui: &mut egui::Ui,
    ) {
        Image::new(
            field_textures[5].texture_id(ui.ctx()),
            field_textures[5].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(egui::pos2(x, y), egui::vec2(FIELD_SIZE, FIELD_SIZE)),
        );
    }
}
