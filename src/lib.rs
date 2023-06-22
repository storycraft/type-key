/*
 * Created on Thu Jun 22 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

#![doc = include_str!("../README.md")]

use core::{
    any::{Any, TypeId}
};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeKey(TypeId);

impl TypeKey {
    pub fn of<T: ?Sized>() -> TypeKey {
        TypeKey(
            (|| {
                let _ = PhantomData::<T>;
            })
            .type_id(),
        )
    }

    pub fn of_val<T: ?Sized>(_: &T) -> TypeKey {
        Self::of::<T>()
    }
}

#[cfg(test)]
mod tests {
    use crate::TypeKey;

    #[test]
    fn test_same_type() {
        assert_eq!(TypeKey::of::<()>(), TypeKey::of::<()>())
    }

    #[test]
    fn test_different_type() {
        assert_ne!(TypeKey::of::<u32>(), TypeKey::of::<u8>())
    }

    #[test]
    fn test_closures() {
        assert_ne!(TypeKey::of_val(&|| {}), TypeKey::of_val(&|| {}))
    }
}
