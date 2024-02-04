use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

use crate::{click::Click, get_size, position::Position, utils};

#[derive(Serialize, Deserialize)]
pub(crate) struct ReplayFile {
    pub(crate) mines: Vec<Position>,
    pub(crate) size: (usize, usize),
    pub(crate) clicks: Vec<Click>,
    pub(crate) duration: Duration,
}

impl ReplayFile {
    pub(crate) fn new(mines: Vec<Position>, clicks: Vec<Click>, duration: Duration) -> Self {
        Self {
            mines,
            size: *get_size(),
            clicks,
            duration,
        }
    }

    pub(crate) fn validate(&self) -> Option<String> {
        if self.clicks.is_empty() {
            return Some("No clicks".to_string());
        }

        if self
            .clicks
            .iter()
            .any(|click| click.position().x >= self.size.0 || click.position().y >= self.size.1)
        {
            return Some("Click out of bounds".to_string());
        }

        if self
            .mines
            .iter()
            .any(|mine| mine.x >= self.size.0 || mine.y >= self.size.1)
        {
            return Some("Mine out of bounds".to_string());
        }

        if !utils::has_unique_elements(&self.mines) {
            return Some("Duplicate mines".to_string());
        }

        let first_click_position = self.clicks.first().unwrap().position();
        if self
            .mines
            .iter()
            .any(|mine| first_click_position.is_near(mine))
        {
            return Some("Invalid board generation: mine near first click".to_string());
        }

        if self.clicks.last().unwrap().duration() > self.duration
            || self.duration - self.clicks.last().unwrap().duration() > Duration::from_secs(1)
        {
            return Some("Invalid duration".to_string());
        }

        None
    }

    pub(crate) fn from_string(string: String) -> Result<Self, String> {
        let file: Self = serde_json::from_str(&string).map_err(|e| e.to_string())?;

        if let Some(error) = file.validate() {
            return Err(error);
        }

        Ok(file)
    }
}

impl Default for ReplayFile {
    fn default() -> Self {
        Self::from_string(include_str!("../assets/default_replay.json").to_owned()).unwrap()
    }
}
