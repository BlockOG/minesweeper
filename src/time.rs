use std::time::{Duration, Instant};

use crate::{board::replay_board::get_time_modifier, get_in_replay_mode, get_replay_file};

#[derive(Clone, Copy)]
pub(crate) enum Time {
    None,
    Playing(Instant),
    Paused(Duration),
    Ended(Duration),
}

impl Time {
    pub(crate) fn reset(&mut self) {
        *self = Self::None;
    }

    pub(crate) fn start(&mut self) {
        *self = Self::Playing(Instant::now());
    }

    pub(crate) fn pause(&mut self) {
        if let Self::Playing(start_time) = self {
            *self = Self::Paused(start_time.elapsed());
        }
    }

    pub(crate) fn resume(&mut self) {
        if let Self::Paused(duration) = self {
            *self = Self::Playing(Instant::now() - *duration);
        }
    }

    pub(crate) fn end(&mut self) {
        if let Self::Playing(start_time) = self {
            *self = Self::Ended(start_time.elapsed());
        } else if let Self::None = self {
            *self = Self::Ended(Duration::from_secs(0));
        }
    }

    pub(crate) fn modifier_decreased(&mut self) {
        match self {
            Time::None => {}
            Time::Playing(start_time) => *start_time -= start_time.elapsed(),
            Time::Paused(duration) | Time::Ended(duration) => *duration *= 2,
        }
    }

    pub(crate) fn modifier_increased(&mut self) {
        match self {
            Time::None => {}
            Time::Playing(start_time) => *start_time += start_time.elapsed() / 2,
            Time::Paused(duration) | Time::Ended(duration) => *duration /= 2,
        }
    }

    pub(crate) fn duration(&self) -> Duration {
        let res = get_time_modifier().apply(match self {
            Self::None => Duration::from_secs(0),
            Self::Playing(start_time) => start_time.elapsed(),
            Self::Paused(duration) | Self::Ended(duration) => *duration,
        });
        if *get_in_replay_mode() {
            res.min(get_replay_file().duration)
        } else {
            res
        }
    }

    pub(crate) fn is_playing(&self) -> bool {
        matches!(self, Self::Playing(_))
    }
}
