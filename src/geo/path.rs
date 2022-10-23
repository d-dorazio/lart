use std::ops::{Index, IndexMut, RangeBounds};

use crate::{bbox_union, Bbox, Path, Rect, V};

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

    pub fn first(&self) -> Option<V> {
        self.points.first().cloned()
    }

    pub fn last(&self) -> Option<V> {
        self.points.last().cloned()
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

    pub fn segment(&self, i: usize) -> Self {
        Self::from([self.points[i], self.points[(i + 1) % self.points.len()]])
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

    pub fn slice(&self, r: impl RangeBounds<usize>) -> Path {
        let mut p = Path::new();
        p.points
            .extend_from_slice(&self.points[(r.start_bound().cloned(), r.end_bound().cloned())]);
        p
    }
}

impl Bbox for Path {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(&self.points)
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

impl<I: IntoIterator<Item = V>> From<I> for Path {
    fn from(it: I) -> Self {
        Self {
            points: it.into_iter().map(V::from).collect(),
        }
    }
}

impl FromIterator<V> for Path {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            points: iter.into_iter().collect(),
        }
    }
}
