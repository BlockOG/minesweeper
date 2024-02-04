use std::{
    fmt::{self, Display, Formatter},
    fs, io,
    time::{Duration, Instant},
};

use eframe::{
    egui::{self, Key, Layout, Slider},
    emath::Align,
    epaint::Color32,
};

use crate::{
    board::Board,
    click::Click,
    field_generator::replay_field_generator::ReplayFieldGenerator,
    get_allow_save_replay, get_mine_amount, get_replay_file, get_size,
    time::Time,
    utils::{ease_in_out_quad, lerp},
    CLICK_ANIMATION_DURATION, INNER_CIRCLE_RADIUS, OUTER_CIRCLE_RADIUS,
};

pub(crate) enum TimeModifier {
    Slowdown(u32),
    None,
    Speedup(u32),
}

pub(crate) fn get_time_modifier() -> &'static mut TimeModifier {
    static mut TIME_MODIFIER: TimeModifier = TimeModifier::None;
    unsafe { &mut TIME_MODIFIER }
}

impl TimeModifier {
    pub(crate) fn apply(&self, duration: Duration) -> Duration {
        match self {
            TimeModifier::Slowdown(slowdown) => duration / *slowdown,
            TimeModifier::None => duration,
            TimeModifier::Speedup(speedup) => duration * *speedup,
        }
    }

    pub(crate) fn reset(&mut self) {
        *self = TimeModifier::None;
    }

    fn decrease(&mut self) -> bool {
        match self {
            TimeModifier::Slowdown(slowdown) => {
                if *slowdown < 16 {
                    *slowdown *= 2;
                    true
                } else {
                    false
                }
            }
            TimeModifier::None => {
                *self = TimeModifier::Slowdown(2);
                true
            }
            TimeModifier::Speedup(speedup) => {
                if *speedup <= 2 {
                    *self = TimeModifier::None;
                } else {
                    *speedup /= 2;
                }
                true
            }
        }
    }

    fn increase(&mut self) -> bool {
        match self {
            TimeModifier::Slowdown(slowdown) => {
                if *slowdown <= 2 {
                    *self = TimeModifier::None;
                } else {
                    *slowdown /= 2;
                }
                true
            }
            TimeModifier::None => {
                *self = TimeModifier::Speedup(2);
                true
            }
            TimeModifier::Speedup(speedup) => {
                if *speedup < 16 {
                    *speedup *= 2;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Display for TimeModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TimeModifier::Slowdown(slowdown) => write!(f, "{}x", 1.0 / *slowdown as f32),
            TimeModifier::None => write!(f, "1x"),
            TimeModifier::Speedup(speedup) => write!(f, "{}x", speedup),
        }
    }
}

pub(crate) struct ReplayBoard {
    board: Board<ReplayFieldGenerator>,

    next_click: usize,
    curr_click_start: Option<Instant>,
}

impl ReplayBoard {
    pub(crate) fn new() -> Self {
        Self {
            board: Board::new(),

            next_click: 0,
            curr_click_start: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.board.reset();
        self.next_click = 0;
        self.curr_click_start = None;
    }

    pub(crate) fn handle_inputs(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        }

        match ctx
            .input(|i| i.key_pressed(Key::ArrowUp) as i32 - i.key_pressed(Key::ArrowDown) as i32)
        {
            -1 => {
                if get_time_modifier().decrease() {
                    self.board.time.modifier_decreased();
                }
            }
            1 => {
                if get_time_modifier().increase() {
                    self.board.time.modifier_increased();
                }
            }
            _ => {}
        }

        if *get_allow_save_replay() && ctx.input(|i| i.key_pressed(Key::S)) {
            if let io::Result::Err(err) = fs::write(
                chrono::Utc::now()
                    .format(
                        format!(
                            "minesweeper_{}x{}-{}_{:.3}_%d-%m-%Y_%H-%M-%S.json",
                            get_size().0,
                            get_size().1,
                            get_mine_amount(),
                            get_replay_file().duration.as_secs_f32()
                        )
                        .as_str(),
                    )
                    .to_string(),
                serde_json::to_string(get_replay_file()).unwrap(),
            ) {
                println!("Failed to save game: {:?}", err);
            } else {
                println!("Saved game");
                *get_allow_save_replay() = false;
            }
        }

        if !self.board.still_playing() {
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Space)) {
            match self.board.time {
                Time::None => self.board.time.start(),
                Time::Playing(_) => self.board.time.pause(),
                Time::Paused(_) => self.board.time.resume(),
                _ => unreachable!(),
            }
        }

        if !self.board.time.is_playing() {
            return;
        }

        let curr_duration = self.board.time.duration();
        for click in get_replay_file()
            .clicks
            .iter()
            .skip(self.next_click)
            .take_while(|click| curr_duration >= click.duration())
        {
            match click {
                Click::Reveal(pos, _) => self.board.reveal(pos, true),
                Click::Flag(pos, _) => self.board.flag(pos, true),
            }

            self.next_click += 1;
            self.curr_click_start = Some(Instant::now());
        }
    }

    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        self.board.draw(ui);

        let click_pos = match (
            self.next_click,
            get_replay_file().clicks.get(self.next_click),
        ) {
            (0, Some(click)) => click.position().coordinates(),
            (_, Some(click)) => {
                let pos = click.position().coordinates();
                let (prev_pos, prev_duration) = match get_replay_file().clicks[self.next_click - 1]
                {
                    Click::Reveal(pos, duration) | Click::Flag(pos, duration) => {
                        (pos.coordinates(), duration)
                    }
                };

                let curr_duration = self.board.time.duration();

                let click_progress = if curr_duration > prev_duration {
                    (curr_duration - prev_duration).as_secs_f32()
                        / (click.duration() - prev_duration).as_secs_f32()
                } else {
                    0.0
                };

                (
                    lerp(prev_pos.0, pos.0, ease_in_out_quad(click_progress)),
                    lerp(prev_pos.1, pos.1, ease_in_out_quad(click_progress)),
                )
            }
            (_, None) => get_replay_file()
                .clicks
                .last()
                .unwrap()
                .position()
                .coordinates(),
        };

        ui.painter().circle_filled(
            click_pos.into(),
            match self.curr_click_start {
                Some(start) => {
                    let elapsed = start.elapsed();
                    if elapsed >= CLICK_ANIMATION_DURATION {
                        self.curr_click_start = None;
                        OUTER_CIRCLE_RADIUS
                    } else {
                        ui.ctx().request_repaint();
                        lerp(
                            INNER_CIRCLE_RADIUS,
                            OUTER_CIRCLE_RADIUS,
                            elapsed.as_secs_f32() / CLICK_ANIMATION_DURATION.as_secs_f32(),
                        )
                    }
                }
                None => OUTER_CIRCLE_RADIUS,
            },
            Color32::from_rgba_premultiplied(125, 125, 125, 77),
        );
        ui.painter().circle_filled(
            click_pos.into(),
            INNER_CIRCLE_RADIUS,
            Color32::from_rgba_premultiplied(128, 128, 128, 159),
        );

        egui::TopBottomPanel::bottom("replay_controls").show(ui.ctx(), |ui| {
            ui.vertical(|ui| {
                let width = ui.available_width();
                ui.style_mut().spacing.slider_width = width;
                let prev = self.next_click;
                if ui
                    .add(
                        Slider::new(&mut self.next_click, 0..=get_replay_file().clicks.len())
                            .trailing_fill(true)
                            .show_value(false),
                    )
                    .changed()
                    || ui.ctx().input(|i| {
                        let left = i.key_pressed(Key::ArrowLeft) && self.next_click > 0;
                        if left {
                            self.next_click -= 1;
                        }
                        let right = i.key_pressed(Key::ArrowRight)
                            && self.next_click < get_replay_file().clicks.len();
                        if right {
                            self.next_click += 1;
                        }
                        left ^ right
                    })
                {
                    match self.next_click {
                        0 => self.board.reset(),
                        click if click == get_replay_file().clicks.len() => {
                            if click != prev {
                                for click in get_replay_file().clicks.iter().skip(prev) {
                                    match click {
                                        Click::Reveal(pos, _) => self.board.reveal(pos, false),
                                        Click::Flag(pos, _) => self.board.flag(pos, false),
                                    }
                                }

                                self.curr_click_start = None;
                            }
                            self.board.time =
                                Time::Ended(get_time_modifier().apply(get_replay_file().duration));
                        }
                        click => {
                            if click > prev {
                                for click in get_replay_file()
                                    .clicks
                                    .iter()
                                    .skip(prev)
                                    .take(click - prev)
                                {
                                    match click {
                                        Click::Reveal(pos, _) => self.board.reveal(pos, false),
                                        Click::Flag(pos, _) => self.board.flag(pos, false),
                                    }
                                }
                            } else {
                                self.board.reset();
                                for click in get_replay_file().clicks.iter().take(click) {
                                    match click {
                                        Click::Reveal(pos, _) => self.board.reveal(pos, false),
                                        Click::Flag(pos, _) => self.board.flag(pos, false),
                                    }
                                }
                            }

                            self.curr_click_start = None;
                            self.board.time = Time::Paused(
                                get_time_modifier()
                                    .apply(get_replay_file().clicks[click - 1].duration()),
                            );
                        }
                    }

                    ui.ctx().request_repaint();
                }

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} {:.3}/{:.3}",
                        get_time_modifier(),
                        self.board.time.duration().as_secs_f32(),
                        get_replay_file().duration.as_secs_f32()
                    ));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(format!(
                            "{}/{}",
                            self.next_click,
                            get_replay_file().clicks.len()
                        ));
                    });
                });
            });
        });

        if self.board.time.is_playing() {
            ui.ctx().request_repaint();
        }
    }
}
