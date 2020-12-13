use std::{fmt, ops::Index};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid<T: Eq + Default> {
    pub width: usize,
    pub height: usize,
    pub items: Vec<T>,
    default: T,
}

pub type Pos = (i32, i32);

impl<T: Eq + Default> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, (i, j): Pos) -> &T {
        if 0 <= i && i < self.height as i32 && 0 <= j && j < self.width as i32 {
            &self.items[i as usize * self.width + j as usize]
        } else {
            &self.default
        }
    }
}

impl<T: fmt::Display + Eq + Default> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", self[(i as i32, j as i32)])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Eq + Default> Grid<T> {
    pub fn new<'a>(width: usize, height: usize, items: Vec<T>) -> Grid<T> {
        assert_eq!(width * height, items.len());
        Grid {
            width,
            height,
            items,
            default: T::default(),
        }
    }
    pub fn from_fn<F>(width: usize, height: usize, f: F) -> Grid<T>
    where
        F: Fn(Pos) -> T,
    {
        let mut items = Vec::with_capacity(width * height);
        for i in 0..height {
            for j in 0..width {
                items.push(f((i as i32, j as i32)));
            }
        }
        Grid {
            width,
            height,
            items,
            default: T::default(),
        }
    }
}
