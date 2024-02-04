pub(crate) mod play_field_generator;
pub(crate) mod replay_field_generator;

use crate::{field::Field, position::Position};

pub(crate) trait FieldGenerator {
    fn generate(dont_surround: &Position) -> Vec<Field>;
}
