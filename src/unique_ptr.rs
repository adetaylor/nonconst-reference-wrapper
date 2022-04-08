use crate::reference_wrapper::NonConstRef;

/// Mini version of `cxx::UniquePtr`.
pub struct UniquePtr<T>(pub *mut T);

impl<T> UniquePtr<T> {
    /// Formerly, this method returned Option<Pin<&mut T>>.
    /// This is a compatibility break, but actually the returned type is
    /// no less fiddly and annoying than the Pin.
    pub fn as_mut(&mut self) -> Option<NonConstRef<T>> {
        // In reality this wouldn't involve a &mut even momentarily,
        // this is just fo
        Some(NonConstRef::new(self.0))
    }

    /// Previously returned a Pin<&mut T>. Possibly we should rename this.
    pub fn pin_mut(&mut self) -> NonConstRef<T> {
        self.as_mut().unwrap()
    }
}
