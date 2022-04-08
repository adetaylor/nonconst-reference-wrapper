

//! Code that cxx might autogenerate for some bindings.

use crate::reference_wrapper::NonConstRef;

/// Simulated C++ type.
pub(crate) struct Inner(pub(crate) u8);

impl Inner {
    /// Const C++ method on T
    pub(crate) fn t_const_method(&self) {}
}

impl NonConstRef<Inner> {
    /// Non-const C++ method on T.
    pub(crate) fn t_nonconst_method(&self) {}
}

// Simulation of a type containing another type
pub(crate) struct Outer {
    pub(crate) t: Inner,
}

impl Outer {
    pub(crate) fn get_t_const(&self) -> &Inner {
        &self.t
    }
    /// Const C++ method on T
    pub(crate) fn outer_const_method(&self) {}
}

impl NonConstRef<Outer> {
    pub(crate) fn get_t_non_const(&self) -> NonConstRef<Inner> {
        // This would be in C++ so would not be scary like this.
        let mutable_t = unsafe {
            std::mem::transmute::<*const Inner,*mut Inner>(&(self.as_ref().t))
        };
        NonConstRef::new(mutable_t as *mut Inner)
    }
    /// Non-const C++ method on T.
    pub(crate) fn outer_nonconst_method(&self) {}
}
