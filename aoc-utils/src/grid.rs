use std::{fs::File, io::BufReader};

use crate::{Args, Error, Input};

pub type Idx2D<T = usize> = (T, T);

#[derive(Clone)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn find_idx<F>(&self, f: F) -> Option<Idx2D>
    where
        F: Fn(&T) -> bool,
    {
        let (idx, _) = self.inner.iter().enumerate().find(|(_, val)| f(val))?;
        Some(self.idx_to_pos(idx))
    }

    pub fn map<S, F>(self, f: F) -> Grid<S>
    where
        F: Fn(T) -> S,
    {
        let Grid { inner, width } = self;
        let inner = inner.into_iter().map(f).collect();
        Grid { width, inner }
    }

    pub fn debug_render<F, S>(&self, mut f: F)
    where
        F: FnMut(Idx2D, &T) -> S,
        S: std::fmt::Display,
    {
        for (idx, elem) in self.inner.iter().enumerate() {
            if idx % self.width == 0 {
                eprintln!();
            }
            eprint!("{}", f(self.idx_to_pos(idx), elem));
        }
        eprintln!();
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = (Idx2D, &T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, val)| (self.idx_to_pos(idx), val))
    }

    pub fn clone_with_fn<S, F>(&self, mut map: F) -> Grid<S>
    where
        F: FnMut(Idx2D, &T) -> S,
    {
        Grid {
            width: self.width,
            inner: self.iter_pos().map(|(pos, elem)| map(pos, elem)).collect(),
        }
    }

    pub fn clone_with<S: Clone>(&self, elem: S) -> Grid<S> {
        Grid {
            width: self.width,
            inner: vec![elem; self.inner.len()],
        }
    }

    pub fn walk<D: Into<Idx2D<isize>>>(&self, (x, y): Idx2D, dir: D) -> Option<Idx2D> {
        let (x_off, y_off) = dir.into();
        let x_off_abs = x_off.unsigned_abs();
        let y_off_abs = y_off.unsigned_abs();
        match (x_off >= 0, y_off >= 0) {
            (true, true) if x + x_off_abs < self.width && y + y_off_abs < self.height() => {
                Some((x + x_off_abs, y + y_off_abs))
            }
            (true, false) if x + x_off_abs < self.width && y >= y_off_abs => {
                Some((x + x_off_abs, y - y_off_abs))
            }
            (false, true) if x >= x_off_abs && y + y_off_abs < self.height() => {
                Some((x - x_off_abs, y + y_off_abs))
            }
            (false, false) if x >= x_off_abs && y >= y_off_abs => {
                Some((x - x_off_abs, y - y_off_abs))
            }
            _ => None,
        }
    }

    pub fn col(&self, x: usize) -> impl Iterator<Item = (Idx2D, &T)> + '_ {
        (0..self.height()).map(move |y| ((x, y), &self[(x, y)]))
    }

    pub fn row(&self, y: usize) -> impl Iterator<Item = (Idx2D, &T)> + '_ {
        (0..self.width()).map(move |x| ((x, y), &self[(x, y)]))
    }

    fn pos_to_idx(&self, (x, y): Idx2D) -> usize {
        x + y * self.width
    }

    fn idx_to_pos(&self, raw: usize) -> Idx2D {
        (raw % self.width, raw / self.width)
    }
}

impl<T, E> Input for Grid<T>
where
    T: TryFrom<u8, Error = E>,
    Error: From<E>,
{
    fn from_args(args: Args) -> crate::Result<Self> {
        let file = File::open(args.file)?;
        let reader = BufReader::new(file);
        let mut width = None;
        let mut inner = vec![];
        let mut byte_count = 0;
        for byte in std::io::Read::bytes(reader) {
            let byte = byte?;
            match byte {
                b'\n' if width.is_none() => width = Some(byte_count),
                b'\n' => {}
                byte => {
                    let val = T::try_from(byte)?;
                    inner.push(val)
                }
            }
            byte_count += 1
        }
        Ok(Grid {
            width: width.unwrap_or(byte_count),
            inner,
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, elem) in self.inner.iter().enumerate() {
            if idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", elem)?;
        }
        writeln!(f)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, elem) in self.inner.iter().enumerate() {
            if idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{:?}", elem)?;
        }
        writeln!(f)
    }
}

impl<T> std::ops::Index<Idx2D> for Grid<T> {
    type Output = T;

    fn index(&self, index: Idx2D) -> &Self::Output {
        &self.inner[self.pos_to_idx(index)]
    }
}

impl<T> std::ops::IndexMut<Idx2D> for Grid<T> {
    fn index_mut(&mut self, index: Idx2D) -> &mut Self::Output {
        let idx = self.pos_to_idx(index);
        &mut self.inner[idx]
    }
}
