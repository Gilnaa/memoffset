// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Produces a range instance representing the sub-slice containing the specified member.
///
/// This macro provides 2 forms of differing functionalities.
///
/// The first form is identical to the appearance of the `offset_of!` macro,
/// and just like `offset_of!`, it has no limit on the depth of fields / subscripts used.
///
/// ```ignore
/// span_of!(Struct, member[index].field)
/// ```
///
/// The second form of `span_of!` returns a sub-slice which starts at one field, and ends at another.
/// The general pattern of this form is:
///
/// ```ignore
/// span_of!(Struct, member_a .. member_b)
/// ```
///
/// Due to `macro_rules`' parsing restrictions, the first expression - the start-anchor - is limited to subscripts, 
/// with no sub-field access.
/// The second expression - the end anchor - has no such limitations.
///
/// By default this form excludes the end-anchor from the range, but inclusive ranges can be opted-in using "..=" instead.
///
/// *Note*: This macro may not make much sense when used on structs that are not `#[repr(C, packed)]`
///
/// ## Examples
/// ```
/// #[macro_use]
/// extern crate memoffset;
///
/// #[repr(C, packed)]
/// struct Florp {
///     a: u32
/// }
/// 
/// #[repr(C, packed)]
/// struct Blarg {
///     x: u64,
///     y: [u8; 56],
///     z: Florp,
///     egg: [[u8; 4]; 4]
/// }
/// 
/// fn main() {
///     assert_eq!(0..8,   span_of!(Blarg, x));
///     assert_eq!(64..68, span_of!(Blarg, z.a));
///     assert_eq!(79..80, span_of!(Blarg, egg[2][3]));
///     
///     assert_eq!(8..64,  span_of!(Blarg, y[0]  ..  z));
///     assert_eq!(0..42,  span_of!(Blarg, x     ..  y[34]));
///     assert_eq!(0..64,  span_of!(Blarg, x     ..= y));
///     assert_eq!(58..68, span_of!(Blarg, y[50] ..= z));
/// }
/// ```
#[macro_export]
macro_rules! span_of {
    ($father:ty,  $field_a:ident $([$index:expr])* .. $($field_b:tt)+) => ({
        let start = offset_of!($father, $field_a $([$index])*);
        let end = offset_of!($father, $($field_b)+);

        start..end
    });

    ($father:ty,  $field_a:ident $([$index:expr])* ..= $($field_b:tt)+) => ({
        let root: *const $father = $crate::ptr::null();

        let start = offset_of!($father, $field_a $([$index])*);
        let end = offset_of!($father, $($field_b)+) + 
                    unsafe { $crate::mem::size_of_val(&(*root).$($field_b)+) };

        start..end
    });
    ($father:ty, $($field:tt)+) => ({
        let root: *const $father = $crate::ptr::null();

        let start = offset_of!($father, $($field)+);
        let end = start + 
                    unsafe { $crate::mem::size_of_val(&(*root).$($field)+) };

        start..end
    });
}

#[cfg(test)]
mod tests {
    #[repr(C, packed)]
    struct Foo {
        a: u32,
        b: [u8; 4],
        c: i64
    }

    #[test]
    fn span_simple() {
        assert_eq!(span_of!(Foo, a), 0..4);
        assert_eq!(span_of!(Foo, b), 4..8);
        assert_eq!(span_of!(Foo, c), 8..16);
    }

    #[test]
    fn span_index() {
        assert_eq!(span_of!(Foo, b[1]), 5..6);
    }

    #[test]
    fn span_forms() {
        #[repr(C, packed)]
        struct Florp {
            a: u32
        }

        #[repr(C, packed)]
        struct Blarg {
            x: u64,
            y: [u8; 56],
            z: Florp,
            egg: [[u8; 4]; 4]
        }

        // Love me some brute force
        assert_eq!(0..8,   span_of!(Blarg, x));
        assert_eq!(64..68, span_of!(Blarg, z.a));
        assert_eq!(79..80, span_of!(Blarg, egg[2][3]));

        assert_eq!(8..64,  span_of!(Blarg, y[0]  ..  z));
        assert_eq!(0..42,  span_of!(Blarg, x     ..  y[34]));
        assert_eq!(0..64,  span_of!(Blarg, x     ..= y));
        assert_eq!(58..68, span_of!(Blarg, y[50] ..= z));
    }
}