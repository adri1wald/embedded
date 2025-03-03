pub struct Grid<T>([T; 4]);

#[derive(Clone, Copy)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Position::TopLeft => f.write_str("top left"),
            Position::TopRight => f.write_str("top right"),
            Position::BottomRight => f.write_str("bottom right"),
            Position::BottomLeft => f.write_str("bottom left"),
        }
    }
}

fn get_index(pos: Position) -> usize {
    match pos {
        Position::TopLeft => 0,
        Position::TopRight => 1,
        Position::BottomRight => 2,
        Position::BottomLeft => 3,
    }
}

fn get_position(index: usize) -> Position {
    match index {
        0 => Position::TopLeft,
        1 => Position::TopRight,
        2 => Position::BottomRight,
        3 => Position::BottomLeft,
        _ => panic!("invalid index"),
    }
}

impl<T> Grid<T> {
    pub fn new(items: [T; 4]) -> Self {
        Self(items)
    }
}

impl<T> Grid<T> {
    pub fn map<U>(self, f: impl Fn(T, Position) -> U) -> Grid<U> {
        let mut index = 0;
        Grid(self.0.map(|item| {
            let pos = get_position(index);
            index += 1;
            f(item, pos)
        }))
    }
}

impl<T> Grid<T> {
    pub fn get(&self, pos: Position) -> &T {
        let index = get_index(pos);
        &self.0[index]
    }

    pub fn get_mut(&mut self, pos: Position) -> &mut T {
        let index = get_index(pos);
        &mut self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
