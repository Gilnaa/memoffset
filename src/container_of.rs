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
    ($ptr:expr, $container:path, $field:tt) => {{
        let ptr = $ptr as *const _;
        if false {
            // Ensure that the pointer has the correct type.
            let $container { $field: f, .. };
            f = *ptr;
        }

        // We don't use .sub because we need to support older Rust versions.
        (ptr as *const u8).offset((offset_of!($container, $field) as isize).wrapping_neg())
            as *mut $container
    }};
}
