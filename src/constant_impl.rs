// A helper to get a const fn version of size_of_val
#[doc(hidden)]
pub const fn size_of<T>(_: &T) -> usize {
    ::mem::size_of::<T>()
}

// While constant pointer transmutation isn't stable, union transmutation is
// This hack should go away after rust-lang/rust#51910
#[doc(hidden)]
pub union Transmuter<T: 'static> {
    pub ptr: &'static T,
    pub int: usize,
}
