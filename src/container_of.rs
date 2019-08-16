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
///     let field_ptr = &container.b;
///     let container2: *const Foo = unsafe { container_of!(field_ptr, Foo, b) };
///     assert_eq!(&container as *const Foo, container2);
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! container_of {
    ($ptr:expr, $container:path, $field:tt) => {{
        let ptr = $ptr as *const _;
        if false {
            // Ensure that the pointer has the correct type.
            let $container { $field: _f, .. };
            _f = $crate::ptr::read(ptr);
        }

        // We don't use .sub because we need to support older Rust versions.
        (ptr as *const u8).offset((offset_of!($container, $field) as isize).wrapping_neg())
            as *const $container
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        #[repr(C)]
        struct Foo {
            a: u32,
            b: [u8; 2],
            c: i64,
        }

        let x = Foo {
            a: 0,
            b: [0; 2],
            c: 0,
        };
        unsafe {
            assert_eq!(container_of!(&x.a, Foo, a), &x as *const _);
            assert_eq!(container_of!(&x.b, Foo, b), &x as *const _);
            assert_eq!(container_of!(&x.c, Foo, c), &x as *const _);
        }
    }

    #[test]
    #[cfg(not(miri))] // this creates unaligned references
    fn simple_packed() {
        #[repr(C, packed)]
        struct Foo {
            a: u32,
            b: [u8; 2],
            c: i64,
        }

        let x = Foo {
            a: 0,
            b: [0; 2],
            c: 0,
        };
        unsafe {
            assert_eq!(container_of!(&x.a, Foo, a), &x as *const _);
            assert_eq!(container_of!(&x.b, Foo, b), &x as *const _);
            assert_eq!(container_of!(&x.c, Foo, c), &x as *const _);
        }
    }

    #[test]
    fn tuple_struct() {
        #[repr(C)]
        struct Tup(i32, i32);

        let x = Tup(0, 0);
        unsafe {
            assert_eq!(container_of!(&x.0, Tup, 0), &x as *const _);
            assert_eq!(container_of!(&x.1, Tup, 1), &x as *const _);
        }
    }

    #[test]
    fn non_copy() {
        use core::cell::RefCell;

        #[repr(C)]
        struct Foo {
            a: RefCell<u8>,
        }

        let x = Foo { a: RefCell::new(0) };
        unsafe {
            assert_eq!(container_of!(&x.a, Foo, a), &x as *const _);
        }
    }
}
