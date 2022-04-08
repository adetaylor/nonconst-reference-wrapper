use std::marker::PhantomData;

/// A C++ non-const reference. These are different from Rust's `&mut T` in that
/// several C++ references can exist to the same underlying data ("aliasing")
/// and that's not permitted in Rust.
///
/// This type
#[repr(transparent)]
#[derive(Copy)]
pub struct NonConstRef<'a, T>(*mut T, PhantomData<&'a T>);

// Implement manually so that there's no need for the inner type to implement Clone
impl<'a, T> Clone for NonConstRef<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<'a, T> NonConstRef<'a, T> {
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a, T> AsRef<T> for NonConstRef<'a, T> {
    fn as_ref(&self) -> &T {
        // Safety: it is guaranteed that no mutable references ever exist
        // to self.0. self.0 is guaranteed to point to an aligned, initialized,
        // instance of T, or at least such concerns are beyond the scope of this
        // aliasing-related demo.
        unsafe { self.0.as_ref() }.unwrap()
    }
}

impl<'a, T> std::ops::Deref for NonConstRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
