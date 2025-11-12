#![no_std]

use core::{
    iter::FusedIterator,
    ops::ControlFlow,
};

pub trait Iteratail: Sized {
    type Item;
    type Tail;

    fn next_or_tail(self) -> ControlFlow<Self::Tail, (Self, Self::Item)>;

    fn into_iter(self) -> IntoIter<Self, Self::Tail> {
        IntoIter {
            iter: Some(self),
            tail: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntoIter<T, U> {
    iter: Option<T>,
    tail: Option<U>,
}

impl<T: Iteratail> Iterator for IntoIter<T, T::Tail> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(iter) = self.iter.take() else {
            return None;
        };
        match iter.next_or_tail() {
            ControlFlow::Continue((iter, item)) => {
                self.iter.replace(iter);
                Some(item)
            }
            ControlFlow::Break(tail) => {
                self.tail.replace(tail);
                None
            }
        }
    }
}

impl<T: Iteratail> FusedIterator for IntoIter<T, T::Tail> {}

impl<T, U> IntoIter<T, U> {
    pub fn take_tail(&mut self) -> Option<U> {
        self.tail.take()
    }

    pub fn into_tail(self) -> Option<U> {
        self.tail
    }
}
