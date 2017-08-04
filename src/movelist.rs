use std::mem;
use std::slice;
use std::ops::{Deref, DerefMut};
use std::ptr;

use types::Move;

const CAPACITY: usize = 256;

pub struct MoveList {
    xs: [Move; CAPACITY],
    len: usize,
}

impl MoveList {
    pub const CAPACITY: usize = CAPACITY;

    pub fn new() -> MoveList {
        MoveList {
            xs: unsafe { mem::uninitialized() },
            len: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize { self.len }

    #[inline]
    pub fn push(&mut self, m: Move) {
        assert!(self.len() < Self::CAPACITY);
        unsafe {
            self.push_unchecked(m);
        }
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, m: Move) {
        let len = self.len();
        debug_assert!(len < Self::CAPACITY);
        ptr::write(self.get_unchecked_mut(len), m);
        self.len = len + 1;
    }

    pub fn swap_retain<F>(&mut self, mut f: F)
        where F: FnMut(&mut Move) -> bool
    {
        let base = self.xs.as_mut_ptr();
        let mut cur = 0;

        while cur < self.len {
            if !f(unsafe { self.get_unchecked_mut(cur) }) {
                self.len -= 1;
                unsafe {
                    ptr::copy(base.offset(self.len as isize),
                              base.offset(cur as isize), 1);
                }
            } else {
                cur += 1;
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = &'a Move;
    type IntoIter = slice::Iter<'a, Move>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl Default for MoveList {
    fn default() -> MoveList {
        MoveList::new()
    }
}

impl Deref for MoveList {
    type Target = [Move];

    #[inline]
    fn deref(&self) -> &[Move] {
        unsafe {
            slice::from_raw_parts(self.xs.as_ptr(), self.len())
        }
    }
}

impl DerefMut for MoveList {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Move] {
        unsafe {
            slice::from_raw_parts_mut(self.xs.as_mut_ptr(), self.len())
        }
    }
}

impl AsMut<[Move]> for MoveList {
    #[inline]
    fn as_mut(&mut self) -> &mut [Move] { self }
}
