use crate::{field::Field, get_replay_file, get_size, position::Position};

use super::FieldGenerator;

pub(crate) struct ReplayFieldGenerator;

impl FieldGenerator for ReplayFieldGenerator {
    fn generate(_dont_surround: &Position) -> Vec<Field> {
        let mut fields = vec![Field::new(); get_size().0 * get_size().1];

        for mine in get_replay_file().mines.iter() {
            fields[mine.index()].set_mine();
        }

        for index in 0..get_size().0 * get_size().1 {
            if fields[index].is_mine() {
                continue;
            }

            let mines = Position::from_index(index)
                .neighbors()
                .into_iter()
                .filter(|pos| fields[pos.index()].is_mine())
                .count();

            fields[index].set_number(mines);
        }

        fields
    }
}
