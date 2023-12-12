use std::{fs::File, io::BufReader};

use crate::{Args, Error, Input};

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

    pub fn find_idx<F>(&self, f: F) -> Option<(usize, usize)>
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
        F: FnMut((usize, usize), &T) -> S,
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

    pub fn iter_pos(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, val)| (self.idx_to_pos(idx), val))
    }

    pub fn clone_with_fn<S, F>(&self, mut map: F) -> Grid<S>
    where
        F: FnMut((usize, usize), &T) -> S,
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

    fn pos_to_idx(&self, (x, y): (usize, usize)) -> usize {
        x + y * self.width
    }

    fn idx_to_pos(&self, raw: usize) -> (usize, usize) {
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

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[self.pos_to_idx(index)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.pos_to_idx(index);
        &mut self.inner[idx]
    }
}