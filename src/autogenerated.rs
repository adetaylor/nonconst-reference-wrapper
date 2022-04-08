//! Code that cxx might autogenerate for some bindings, backed by C++
//! functions.

use crate::reference_wrapper::NonConstRef;

/// Simulated C++ type.
pub(crate) struct Inner(pub(crate) u8);

impl Inner {
    /// Const C++ method on T
    pub(crate) fn t_const_method(&self) {}
}

impl<'a> NonConstRef<'a, Inner> {
    /// Non-const C++ method on T.
    /// Note the receiver is a &mut NonConstRef<T>, not a &mut T, so this is OK
    /// from an aliasing point of view.
    /// The reeceiver here doesn't HAVE to be &mut. We could just take &self.
    /// We're making it _harder_ for folks to accidentally mutate C++ state
    /// by mutating the same NonConstRef in several places. But fundamentally,
    /// several NonConstRefs can exist to the same C++ data, and there's nothing
    /// we can do to prevent that. Indeed, NonConstRef is actually Clone and
    /// Copy. So this &mut exclusivity is advisory, at best.
    pub(crate) fn t_nonconst_method(&mut self) {}
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

impl<'a> NonConstRef<'a, Outer> {
    pub(crate) fn get_t_non_const(&self) -> NonConstRef<Inner> {
        // This would be in C++ so would simply be:
        //   Inner& Outer::get_t_non_const() { return t; }
        let mutable_t =
            unsafe { std::mem::transmute::<*const Inner, *mut Inner>(&(self.as_ref().t)) };
        NonConstRef::new(mutable_t as *mut Inner)
    }
    /// Non-const C++ method on T.
    pub(crate) fn outer_nonconst_method(&mut self) {}
}

pub(crate) fn take_const_reference(_inner: &Inner) {}
pub(crate) fn take_nonconst_reference(_inner: NonConstRef<Inner>) {}
