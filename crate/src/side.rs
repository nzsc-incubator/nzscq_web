#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Left = 0,
    Right = 1,
}

impl Side {
    pub fn if_left<T>(self, item: T) -> Option<T> {
        if self == Side::Left {
            Some(item)
        } else {
            None
        }
    }
}