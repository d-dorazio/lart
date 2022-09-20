use std::ops::{Index, IndexMut};

use crate::{Path, V};

impl Path {
    pub const fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            points: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, a: impl Into<V>) {
        self.points.push(a.into())
    }

    pub fn pop(&mut self) -> Option<V> {
        self.points.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = V> + '_ {
        self.points.iter().copied()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> + '_ {
        self.points.iter_mut()
    }

    pub fn points(&self) -> &[V] {
        &self.points
    }

    pub fn segments(&self) -> impl Iterator<Item = (V, V)> + '_ {
        self.iter().zip(self.iter().skip(1))
    }

    pub fn norm(&self) -> f64 {
        self.segments().map(|(a, b)| a.dist(b)).sum()
    }

    pub fn norm2(&self) -> f64 {
        self.segments().map(|(a, b)| a.dist2(b)).sum()
    }
}

impl Index<usize> for Path {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.points.index(index)
    }
}

impl IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.points.index_mut(index)
    }
}

impl<W, I> From<I> for Path
where
    V: From<W>,
    I: IntoIterator<Item = W>,
{
    fn from(it: I) -> Self {
        Self {
            points: it.into_iter().map(V::from).collect(),
        }
    }
}

impl<W: Into<V>> FromIterator<W> for Path {
    fn from_iter<T: IntoIterator<Item = W>>(iter: T) -> Self {
        Self {
            points: iter.into_iter().map(|w| w.into()).collect(),
        }
    }
}
