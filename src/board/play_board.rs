use std::{fs, io};

use eframe::egui::{self, Key, PointerButton};

use crate::{
    board::Board, click::Click, field_generator::play_field_generator::PlayFieldGenerator,
    get_mine_amount, get_size, position::Position, replay_file::ReplayFile,
};

pub(crate) struct PlayBoard {
    board: Board<PlayFieldGenerator>,

    clicks: Vec<Click>,
}

impl PlayBoard {
    pub(crate) fn new() -> Self {
        Self {
            board: Board::new(),

            clicks: Vec::new(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.board.reset();
        self.clicks.clear();
    }

    pub(crate) fn create_replay(&self) -> Option<ReplayFile> {
        if self.board.still_playing() {
            None
        } else {
            Some(ReplayFile::new(
                self.board
                    .fields
                    .as_ref()
                    .unwrap()
                    .get_fields()
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| f.is_mine())
                    .map(|(i, _)| Position::from_index(i))
                    .collect(),
                self.clicks.clone(),
                self.board.time.duration(),
            ))
        }
    }

    pub(crate) fn handle_inputs(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        }

        if !self.board.still_playing() {
            if ctx.input(|i| i.key_pressed(Key::S)) {
                if let io::Result::Err(err) = fs::write(
                    chrono::Utc::now()
                        .format(
                            format!(
                                "minesweeper_{}x{}-{}_{:.3}_%d-%m-%Y_%H-%M-%S.json",
                                get_size().0,
                                get_size().1,
                                get_mine_amount(),
                                self.board.time.duration().as_secs_f32()
                            )
                            .as_str(),
                        )
                        .to_string(),
                    serde_json::to_string(&self.create_replay().unwrap()).unwrap(),
                ) {
                    println!("Failed to save game: {:?}", err);
                } else {
                    println!("Saved game");
                }
            }

            return;
        }

        if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary)) {
            if let Some(pos) =
                Position::from_mouse(ctx.input(|i| i.pointer.interact_pos().unwrap()))
            {
                self.board.reveal(&pos, true);
                self.clicks
                    .push(Click::Reveal(pos, self.board.time.duration()));
            }
        }

        if ctx.input(|i| i.pointer.button_pressed(PointerButton::Secondary)) {
            if let Some(pos) =
                Position::from_mouse(ctx.input(|i| i.pointer.interact_pos().unwrap()))
            {
                self.board.flag(&pos, true);
                self.clicks
                    .push(Click::Flag(pos, self.board.time.duration()));
            }
        }
    }

    pub(crate) fn draw(&self, ui: &mut egui::Ui) {
        self.board.draw(ui);
    }
}
