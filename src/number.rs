#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            value => panic!("Invalid number: {}", value),
        }
    }
}

impl From<Number> for usize {
    fn from(value: Number) -> Self {
        (&value).into()
    }
}

impl From<&Number> for usize {
    fn from(value: &Number) -> Self {
        match value {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
        }
    }
}

impl PartialEq<usize> for Number {
    fn eq(&self, other: &usize) -> bool {
        *other == usize::from(self)
    }
}
