#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Visibility {
    Closed,
    Open,
    Flagged,
}
