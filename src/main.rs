#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod border;
mod click;
mod field;
mod field_generator;
mod fields;
mod number;
mod position;
mod replay_file;
mod seven_segment_number;
mod sounds;
mod textures;
mod tile;
mod time;
mod utils;
mod visibility;
mod won;

use std::{env::args, fs, io, path::PathBuf, time::Duration};

use board::replay_board::get_time_modifier;
use eframe::{
    egui::{self, Key},
    epaint::Vec2,
};
use replay_file::ReplayFile;
use sounds::SoundPlayer;

use crate::board::{play_board::PlayBoard, replay_board::ReplayBoard};

fn get_sound_player() -> &'static mut SoundPlayer {
    static mut SOUND_PLAYER: Option<SoundPlayer> = None;

    if unsafe { SOUND_PLAYER.is_none() } {
        println!("Loading sounds...");
        unsafe { SOUND_PLAYER = Some(SoundPlayer::new()) }
        println!("Sounds loaded!");
    }

    unsafe { SOUND_PLAYER.as_mut().unwrap() }
}

fn get_size() -> &'static mut (usize, usize) {
    static mut SIZE: (usize, usize) = (8, 8);
    unsafe { &mut SIZE }
}

fn set_size(size: (usize, usize), frame: &mut eframe::Frame) {
    *get_size() = size;
    frame.set_window_size(get_window_size());
}

fn get_mine_amount() -> &'static mut usize {
    static mut MINE_AMOUNT: usize = 10;
    unsafe { &mut MINE_AMOUNT }
}

fn get_in_replay_mode() -> &'static mut bool {
    static mut IN_REPLAY_MODE: bool = false;
    unsafe { &mut IN_REPLAY_MODE }
}

fn get_replay_file() -> &'static mut ReplayFile {
    static mut REPLAY_FILE: Option<ReplayFile> = None;
    if unsafe { REPLAY_FILE.is_none() } {
        unsafe { REPLAY_FILE = Some(ReplayFile::default()) }
    }

    unsafe { REPLAY_FILE.as_mut().unwrap() }
}

fn get_allow_save_replay() -> &'static mut bool {
    static mut ALLOW_SAVE_REPLAY: bool = false;
    unsafe { &mut ALLOW_SAVE_REPLAY }
}

const FIELD_SIZE: f32 = 32.0;

const NUMBER_WIDTH: f32 = FIELD_SIZE * 0.66;
const NUMBER_HEIGHT: f32 = FIELD_SIZE * 1.26;

const NUMBER_MARGIN: f32 = FIELD_SIZE * 0.12;

const BORDER_WIDTH: f32 = FIELD_SIZE * 0.72;
const BORDER_HEIGHT: f32 = FIELD_SIZE * 0.66;

const OUTER_CIRCLE_RADIUS: f32 = FIELD_SIZE * 0.39;
const INNER_CIRCLE_RADIUS: f32 = FIELD_SIZE * 0.235;

const CLICK_ANIMATION_DURATION: Duration = Duration::from_millis(200);

fn get_window_size() -> Vec2 {
    egui::vec2(
        BORDER_WIDTH * 2.0 + get_size().0 as f32 * FIELD_SIZE,
        BORDER_HEIGHT * 3.0
            + NUMBER_MARGIN * 2.0
            + NUMBER_HEIGHT
            + get_size().1 as f32 * FIELD_SIZE
            + if *get_in_replay_mode() { 43.0 } else { 0.0 },
    )
}

struct Minesweeper {
    play_board: PlayBoard,
    replay_board: ReplayBoard,

    choose_new_settings: bool,
    new_width_string: String,
    new_height_string: String,
    new_mines_string: String,

    hovered_files: Vec<egui::HoveredFile>,
}

impl Minesweeper {
    fn new(replay: Option<String>) -> Self {
        if let Some(replay) = replay {
            if let Ok(path) = PathBuf::try_from(replay) {
                if !Minesweeper::load_replay_file(path) {
                    println!("Couldn't load replay file.");
                }
            } else {
                println!("Invalid file path.");
            }
        }

        println!("Loading textures...");
        let play_board = PlayBoard::new();
        let replay_board = ReplayBoard::new();
        println!("Textures loaded!");

        Self {
            play_board,
            replay_board,

            choose_new_settings: false,
            new_width_string: String::new(),
            new_height_string: String::new(),
            new_mines_string: String::new(),

            hovered_files: Vec::new(),
        }
    }

    fn load_replay_file(path: PathBuf) -> bool {
        if let io::Result::Ok(contents) = fs::read_to_string(&path) {
            match ReplayFile::from_string(contents) {
                Ok(replay) => {
                    *get_replay_file() = replay;
                    *get_size() = get_replay_file().size;
                    *get_mine_amount() = get_replay_file().mines.len();
                    *get_in_replay_mode() = true;
                    true
                }
                Err(err) => {
                    println!("Error loading replay file: {}", err);
                    false
                }
            }
        } else {
            println!("Couldn't read file: {}", path.display());
            false
        }
    }

    fn play_update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.choose_new_settings = !self.choose_new_settings;
            self.new_width_string.clear();
            self.new_height_string.clear();
            self.new_mines_string.clear();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.choose_new_settings {
                ui.heading("New Settings");

                if ui.button("Beginner").clicked() {
                    set_size((8, 8), frame);
                    *get_mine_amount() = 10;
                    self.play_board.reset();
                }

                if ui.button("Intermediate").clicked() {
                    set_size((16, 16), frame);
                    *get_mine_amount() = 40;
                    self.play_board.reset();
                }

                if ui.button("Expert").clicked() {
                    set_size((30, 16), frame);
                    *get_mine_amount() = 99;
                    self.play_board.reset();
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.text_edit_singleline(&mut self.new_width_string);
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.text_edit_singleline(&mut self.new_height_string);
                });

                ui.horizontal(|ui| {
                    ui.label("Mines:");
                    ui.text_edit_singleline(&mut self.new_mines_string);
                });

                if ui.button("OK").clicked() {
                    let Ok(new_width) = self.new_width_string.parse::<usize>() else {
                        return;
                    };

                    let Ok(new_height) = self.new_height_string.parse::<usize>() else {
                        return;
                    };

                    let Ok(new_mines) = self.new_mines_string.parse::<usize>() else {
                        return;
                    };

                    if new_width < 5 || new_height < 5 {
                        return;
                    }

                    set_size((new_width, new_height), frame);
                    *get_mine_amount() = new_mines.min((new_width * new_height).saturating_sub(9));
                    self.play_board.reset();
                }
            } else {
                self.play_board.handle_inputs(ctx);
                self.play_board.draw(ui);
            }
        });
    }

    fn replay_update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.choose_new_settings = !self.choose_new_settings;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.choose_new_settings {
                ui.heading("Choose new replay");
                ui.label(
                    "Drag and drop a replay file anywhere in this app at any time to load it.",
                );
                ui.separator();

                if !self.hovered_files.is_empty() {
                    if self.hovered_files.len() == 1 {
                        if let Some(ref path) = self.hovered_files[0].path {
                            ui.label(format!("Hovered file path: {}", path.display()));
                        } else {
                            ui.label("Couldn't get file path.");
                        }
                    } else {
                        ui.label("Can't load multiple files at once.");
                    }
                }
            } else {
                self.replay_board.handle_inputs(ctx);
                self.replay_board.draw(ui);
            }
        });
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if !i.raw.hovered_files.is_empty() {
                self.hovered_files = i.raw.hovered_files.clone();
            }
            if !i.raw.dropped_files.is_empty() {
                self.hovered_files.clear();
                if i.raw.dropped_files.len() == 1 {
                    if let Some(ref path) = i.raw.dropped_files[0].path {
                        if Minesweeper::load_replay_file(path.clone()) {
                            self.play_board.reset();
                            self.replay_board.reset();
                            get_time_modifier().reset();
                            self.choose_new_settings = false;
                            frame.set_window_size(get_window_size());
                            *get_allow_save_replay() = false;
                        }
                    }
                }
            }
        });

        if !self.choose_new_settings && ctx.input(|i| i.key_pressed(Key::Enter)) {
            if *get_in_replay_mode() {
                *get_in_replay_mode() = false;
                frame.set_window_size(get_window_size());
            } else {
                if let Some(replay) = self.play_board.create_replay() {
                    *get_replay_file() = replay;
                    *get_allow_save_replay() = true;
                } else {
                    *get_replay_file() = ReplayFile::default();
                    set_size(get_replay_file().size, frame);
                    *get_mine_amount() = get_replay_file().mines.len();
                    *get_allow_save_replay() = false;
                }

                *get_in_replay_mode() = true;
                frame.set_window_size(get_window_size());
            }

            self.play_board.reset();
            self.replay_board.reset();
            get_time_modifier().reset();
        }

        if *get_in_replay_mode() {
            self.replay_update(ctx, frame);
        } else {
            self.play_update(ctx, frame);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let minesweeper = Minesweeper::new(args().skip(1).next());

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        resizable: false,
        initial_window_size: Some(get_window_size()),
        ..Default::default()
    };

    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Box::new(minesweeper)),
    )
}
