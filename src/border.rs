use eframe::{
    egui::{self, Image},
    epaint::Rect,
};
use egui_extras::{image::FitTo, RetainedImage};

use crate::{get_size, BORDER_HEIGHT, BORDER_WIDTH, FIELD_SIZE, NUMBER_HEIGHT, NUMBER_MARGIN};

const TOP_LEFT: &str = include_str!("../assets/top_left.svg");
const TOP_RIGHT: &str = include_str!("../assets/top_right.svg");
const BOTTOM_LEFT: &str = include_str!("../assets/bottom_left.svg");
const BOTTOM_RIGHT: &str = include_str!("../assets/bottom_right.svg");

const HORIZONTAL: &str = include_str!("../assets/horizontal.svg");
const VERTICAL: &str = include_str!("../assets/vertical.svg");

const T_LEFT: &str = include_str!("../assets/t_left.svg");
const T_RIGHT: &str = include_str!("../assets/t_right.svg");

pub(crate) struct Border {
    border_textures: Vec<RetainedImage>,
}

impl Border {
    pub(crate) fn new() -> Self {
        Self {
            border_textures: [
                TOP_LEFT,
                TOP_RIGHT,
                BOTTOM_LEFT,
                BOTTOM_RIGHT,
                T_LEFT,
                T_RIGHT,
            ]
            .iter()
            .map(|svg| {
                RetainedImage::from_svg_bytes_with_size(
                    "I have no debug name unfortunately",
                    svg.as_bytes(),
                    FitTo::Size(BORDER_WIDTH as u32, BORDER_HEIGHT as u32),
                )
                .unwrap()
            })
            .chain(
                [
                    RetainedImage::from_svg_bytes_with_size(
                        "I have a debug name: horizontal",
                        HORIZONTAL.as_bytes(),
                        FitTo::Size(1, BORDER_HEIGHT as u32),
                    )
                    .unwrap(),
                    RetainedImage::from_svg_bytes_with_size(
                        "I have a debug name: vertical",
                        VERTICAL.as_bytes(),
                        FitTo::Size(BORDER_WIDTH as u32, 1),
                    )
                    .unwrap(),
                ]
                .into_iter(),
            )
            .collect(),
        }
    }

    pub(crate) fn draw(&self, ui: &mut egui::Ui) {
        /* #region Draw border */

        /* #region Draw horizontal stripes */
        Image::new(
            self.border_textures[6].texture_id(ui.ctx()),
            self.border_textures[6].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(
                    BORDER_WIDTH * 2.0 + FIELD_SIZE * get_size().0 as f32,
                    BORDER_HEIGHT,
                ),
            ),
        );
        Image::new(
            self.border_textures[6].texture_id(ui.ctx()),
            self.border_textures[6].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(0.0, BORDER_HEIGHT + NUMBER_MARGIN * 2.0 + NUMBER_HEIGHT),
                egui::vec2(
                    BORDER_WIDTH * 2.0 + FIELD_SIZE * get_size().0 as f32,
                    BORDER_HEIGHT,
                ),
            ),
        );
        Image::new(
            self.border_textures[6].texture_id(ui.ctx()),
            self.border_textures[6].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(
                    0.0,
                    BORDER_HEIGHT * 2.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + FIELD_SIZE * get_size().1 as f32,
                ),
                egui::vec2(
                    BORDER_WIDTH * 2.0 + FIELD_SIZE * get_size().0 as f32,
                    BORDER_HEIGHT,
                ),
            ),
        );
        /* #endregion */

        /* #region Draw vertical stripes */
        Image::new(
            self.border_textures[7].texture_id(ui.ctx()),
            self.border_textures[7].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(
                    BORDER_WIDTH,
                    BORDER_HEIGHT * 3.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + FIELD_SIZE * get_size().0 as f32,
                ),
            ),
        );
        Image::new(
            self.border_textures[7].texture_id(ui.ctx()),
            self.border_textures[7].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(BORDER_WIDTH + FIELD_SIZE * get_size().0 as f32, 0.0),
                egui::vec2(
                    BORDER_WIDTH,
                    BORDER_HEIGHT * 3.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + FIELD_SIZE * get_size().0 as f32,
                ),
            ),
        );
        /* #endregion */

        /* #region Draw corners */
        Image::new(
            self.border_textures[0].texture_id(ui.ctx()),
            self.border_textures[0].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );
        Image::new(
            self.border_textures[1].texture_id(ui.ctx()),
            self.border_textures[1].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(BORDER_WIDTH + FIELD_SIZE * get_size().0 as f32, 0.0),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );
        Image::new(
            self.border_textures[2].texture_id(ui.ctx()),
            self.border_textures[2].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(
                    0.0,
                    BORDER_HEIGHT * 2.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + FIELD_SIZE * get_size().1 as f32,
                ),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );
        Image::new(
            self.border_textures[3].texture_id(ui.ctx()),
            self.border_textures[3].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(
                    BORDER_WIDTH + FIELD_SIZE * get_size().0 as f32,
                    BORDER_HEIGHT * 2.0
                        + NUMBER_MARGIN * 2.0
                        + NUMBER_HEIGHT
                        + FIELD_SIZE * get_size().1 as f32,
                ),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );

        Image::new(
            self.border_textures[4].texture_id(ui.ctx()),
            self.border_textures[4].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(0.0, BORDER_HEIGHT + NUMBER_MARGIN * 2.0 + NUMBER_HEIGHT),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );
        Image::new(
            self.border_textures[5].texture_id(ui.ctx()),
            self.border_textures[5].size_vec2(),
        )
        .paint_at(
            ui,
            Rect::from_min_size(
                egui::pos2(
                    BORDER_WIDTH + FIELD_SIZE * get_size().0 as f32,
                    BORDER_HEIGHT + NUMBER_MARGIN * 2.0 + NUMBER_HEIGHT,
                ),
                egui::vec2(BORDER_WIDTH, BORDER_HEIGHT),
            ),
        );
        /* #endregion */

        /* #endregion */
    }
}
