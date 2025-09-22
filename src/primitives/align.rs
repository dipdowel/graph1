// FIXME: Make the text system use this alignment enum instead of its own
// FIXME: Refactor the code that uses the old text enum accordingly
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Right,
    Center,
}

