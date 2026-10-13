use std::borrow::Borrow;
use std::ops::Deref;

use crate::MyCow::*;

pub enum MyCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

impl<'a, B: ?Sized + ToOwned> Borrow<B> for MyCow<'a, B> {
    fn borrow(&self) -> &B {
        match self {
            Borrowed(b) => b,
            Owned(b) => b.borrow(),
        }
    }
}

impl<B: ?Sized + ToOwned> Clone for MyCow<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            Borrowed(b) => Borrowed(b),
            Owned(ref b) => Owned(b.borrow().to_owned()),
        }
    }
}

impl<B: ?Sized + ToOwned> MyCow<'_, B> {
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            Borrowed(_) => true,
            Owned(_) => false,
        }
    }

    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Owned(ref mut b) => b,
            Borrowed(..) => {
                *self = self.to_owned();
                match *self {
                    Borrowed(..) => unreachable!(),
                    Owned(ref mut b) => b,
                }
            }
        }
    }

    pub fn into_owned(self) -> <B as ToOwned>::Owned {
        match self {
            Owned(owned) => owned,
            Borrowed(borrowed) => borrowed.to_owned(),
        }
    }
}

impl<B: ?Sized + ToOwned> Deref for MyCow<'_, B> {
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            Owned(ref b) => b.borrow(),
            Borrowed(b) => b,
        }
    }
}

// Start here next time: https://doc.rust-lang.org/src/alloc/borrow.rs.html#361

fn main() {
    println!("Hello, world!");
}
