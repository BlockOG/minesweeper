use egui_extras::{image::FitTo, RetainedImage};

use crate::{FIELD_SIZE, NUMBER_HEIGHT, NUMBER_WIDTH};

const CLOSED: &str = include_str!("../assets/closed.svg");
const EMPTY: &str = include_str!("../assets/empty.svg");

const MINE: &str = include_str!("../assets/mine.svg");
const FLAG: &str = include_str!("../assets/flag.svg");

const MINE_RED: &str = include_str!("../assets/mine_red.svg");
const MINE_WRONG: &str = include_str!("../assets/mine_wrong.svg");

const ONE: &str = include_str!("../assets/number1.svg");
const TWO: &str = include_str!("../assets/number2.svg");
const THREE: &str = include_str!("../assets/number3.svg");
const FOUR: &str = include_str!("../assets/number4.svg");
const FIVE: &str = include_str!("../assets/number5.svg");
const SIX: &str = include_str!("../assets/number6.svg");
const SEVEN: &str = include_str!("../assets/number7.svg");
const EIGHT: &str = include_str!("../assets/number8.svg");

const NUMBER_ZERO: &str = include_str!("../assets/7segment0.svg");
const NUMBER_ONE: &str = include_str!("../assets/7segment1.svg");
const NUMBER_TWO: &str = include_str!("../assets/7segment2.svg");
const NUMBER_THREE: &str = include_str!("../assets/7segment3.svg");
const NUMBER_FOUR: &str = include_str!("../assets/7segment4.svg");
const NUMBER_FIVE: &str = include_str!("../assets/7segment5.svg");
const NUMBER_SIX: &str = include_str!("../assets/7segment6.svg");
const NUMBER_SEVEN: &str = include_str!("../assets/7segment7.svg");
const NUMBER_EIGHT: &str = include_str!("../assets/7segment8.svg");
const NUMBER_NINE: &str = include_str!("../assets/7segment9.svg");
const NUMBER_MINUS: &str = include_str!("../assets/7segment-.svg");

const NUMBER_BACKGROUND: &str = include_str!("../assets/nums_background.svg");

pub(super) struct Textures {
    pub(super) field_textures: Vec<RetainedImage>,
    pub(super) number_textures: Vec<RetainedImage>,
    pub(super) number_bg_texture: RetainedImage,
}

impl Textures {
    pub(super) fn new() -> Self {
        Self {
            field_textures: [
                CLOSED, EMPTY, MINE, FLAG, MINE_RED, MINE_WRONG, ONE, TWO, THREE, FOUR, FIVE, SIX,
                SEVEN, EIGHT,
            ]
            .iter()
            .map(|svg| {
                RetainedImage::from_svg_bytes_with_size(
                    "I have no debug name unfortunately",
                    svg.as_bytes(),
                    FitTo::Size(FIELD_SIZE as u32, FIELD_SIZE as u32),
                )
                .unwrap()
            })
            .collect(),

            number_textures: [
                NUMBER_ZERO,
                NUMBER_ONE,
                NUMBER_TWO,
                NUMBER_THREE,
                NUMBER_FOUR,
                NUMBER_FIVE,
                NUMBER_SIX,
                NUMBER_SEVEN,
                NUMBER_EIGHT,
                NUMBER_NINE,
                NUMBER_MINUS,
            ]
            .iter()
            .map(|svg| {
                RetainedImage::from_svg_bytes_with_size(
                    "I have no debug name unfortunately",
                    svg.as_bytes(),
                    FitTo::Size(NUMBER_WIDTH as u32, NUMBER_HEIGHT as u32),
                )
                .unwrap()
            })
            .collect(),

            number_bg_texture: RetainedImage::from_svg_bytes_with_size(
                "I have a debug name: number background",
                NUMBER_BACKGROUND.as_bytes(),
                FitTo::Size((FIELD_SIZE * 2.46) as u32, (FIELD_SIZE * 1.5) as u32),
            )
            .unwrap(),
        }
    }
}
