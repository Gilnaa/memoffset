/// Calculates the address of a containing struct from a pointer to one of its
/// fields.
///
/// # Safety
///
/// This is unsafe because it assumes that the given expression is a valid
/// pointer to the specified field of some container type.
///
/// ## Examples
/// ```
/// #[macro_use]
/// extern crate memoffset;
///
/// #[repr(C, packed)]
/// struct Foo {
///     a: u32,
///     b: u64,
///     c: [u8; 5]
/// }
///
/// fn main() {
///     let container = Foo { a: 1, b: 2, c: [3; 5] };
///     let field = &container.b;
///     let container2: *const Foo = unsafe { container_of!(field, Foo, b) };
///     assert_eq!(&container as *const Foo, container2);
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! container_of {
    ($ptr:expr, $container:path, $field:tt) => {
        ($ptr as *const _ as *const u8).offset(-(offset_of!($container, $field) as isize))
            as *mut $container
    };
}
