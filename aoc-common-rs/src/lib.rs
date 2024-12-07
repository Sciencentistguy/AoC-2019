#![feature(pattern)]

use std::{
    ops::{Add, AddAssign, Neg},
    str::pattern::Pattern,
};

use collect_slice::CollectSlice;
use ndarray::{Dimension, Ix, Ix2, Ixs, NdIndex};
use smallvec::SmallVec;
pub trait ArraySplit {
    type Out<'a>
    where
        Self: 'a;

    fn array_split<const N: usize>(&self, pat: impl Pattern) -> [Self::Out<'_>; N];
    fn array_lines<const N: usize>(&self) -> [Self::Out<'_>; N];
}

impl ArraySplit for str {
    type Out<'a> = &'a Self;

    fn array_split<const N: usize>(&self, pat: impl Pattern) -> [Self::Out<'_>; N] {
        let mut a = [""; N];
        self.split(pat).collect_slice_checked(&mut a);
        a
    }

    fn array_lines<const N: usize>(&self) -> [Self::Out<'_>; N] {
        let mut a = [""; N];
        self.lines().collect_slice_checked(&mut a);
        a
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vec2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2D<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> From<(T, T)> for Vec2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vec2D<T>> for [T; 2]
where
    T: Copy,
{
    fn from(vec: Vec2D<T>) -> Self {
        [vec.x, vec.y]
    }
}

impl<T> Add<(T, T)> for Vec2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: (T, T)) -> Self {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Vec2D<usize> {
    pub fn wrapping_add_assign_signed(&mut self, rhs: Vec2D<isize>) {
        self.x = self.x.wrapping_add_signed(rhs.x);
        self.y = self.y.wrapping_add_signed(rhs.y);
    }
    pub fn wrapping_add_signed(&self, rhs: Vec2D<isize>) -> Self {
        Self {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
        }
    }
    pub fn checked_add_signed(&self, rhs: Vec2D<isize>) -> Option<Self> {
        self.x
            .checked_add_signed(rhs.x)
            .and_then(|x| self.y.checked_add_signed(rhs.y).map(|y| Self { x, y }))
    }
}
impl<T> Vec2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn rotate_right(&mut self)
    where
        T: Neg<Output = T> + Copy,
    {
        *self = Self::new(-self.y, self.x);
    }
}

// Safety: this is identical to the impl for `(usize, usize)` in ndarray.
unsafe impl NdIndex<Ix2> for Vec2D<usize> {
    #[inline]
    fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
        dim.stride_offset_checked(strides, &Ix2(self.x, self.y))
    }
    #[inline]
    fn index_unchecked(&self, strides: &Ix2) -> isize {
        fn stride_offset(n: Ix, stride: Ix) -> isize {
            (n as isize) * (stride as Ixs)
        }
        stride_offset(self.x, strides[0]) + stride_offset(self.y, strides[1])
    }
}

pub trait MonadicJoin {
    type Output;
    fn join(self) -> Self::Output;
}

impl<T> MonadicJoin for Option<Option<T>> {
    type Output = Option<T>;
    fn join(self) -> Self::Output {
        self.and_then(|x| x)
    }
}
impl<'a, T> MonadicJoin for Option<&'a Option<T>> {
    type Output = Option<&'a T>;
    fn join(self) -> Self::Output {
        self.and_then(|x| x.as_ref())
    }
}

#[inline]
pub fn generate_combs_n<T: Clone, const N: usize>(vals: &[T], n: usize) -> Vec<SmallVec<T, N>> {
    #[inline]
    fn generate_combs_recursive<T: Clone, const N: usize>(
        vals: &[T],
        n: usize,
        current: &mut SmallVec<T, N>,
        result: &mut Vec<SmallVec<T, N>>,
    ) {
        if n == 0 {
            result.push(current.clone());
            return;
        }
        for c in vals {
            current.push(c.clone());
            generate_combs_recursive(vals, n - 1, current, result);
            current.pop();
        }
    }

    let num_combs = vals.len().pow(n as u32);
    let mut result = Vec::with_capacity(num_combs);
    let mut current = SmallVec::<_, N>::with_capacity(n);
    generate_combs_recursive(vals, n, &mut current, &mut result);
    result
}