
/// A C++ non-const reference. These are different from Rust's `&mut T` in that
/// several C++ references can exist to the same underlying data ("aliasing")
/// and that's not permitted in Rust.
#[repr(transparent)]
pub struct NonConstRef<T>(*mut T);

impl<T> NonConstRef<T> {
    pub fn new(ptr: *mut T) -> Self { Self(ptr)}
}

impl<T> AsRef<T> for NonConstRef<T> {
    fn as_ref(&self) -> &T {
        // Safety: it is guaranteed that no mutable references ever exist
        // to self.0.
        unsafe { self.0.as_ref() }.unwrap()
    }
}

impl<T> std::ops::Deref for NonConstRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}