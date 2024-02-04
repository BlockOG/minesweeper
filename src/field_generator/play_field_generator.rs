use rand::{thread_rng, Rng};

use crate::{field::Field, get_mine_amount, get_size, position::Position};

use super::FieldGenerator;

pub(crate) struct PlayFieldGenerator;

impl FieldGenerator for PlayFieldGenerator {
    fn generate(dont_surround: &Position) -> Vec<Field> {
        let mut rng = thread_rng();

        let mut fields = vec![Field::new(); get_size().0 * get_size().1];

        let mut mines = 0;
        while mines < *get_mine_amount() {
            let index = rng.gen_range(0..get_size().0 * get_size().1);

            if fields[index].is_mine() || dont_surround.is_near(&Position::from_index(index)) {
                continue;
            }

            fields[index].set_mine();
            mines += 1;
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
