//! A crate used for calculating offsets of struct members and their spans.
//!
//! Some of the funcationality of the crate makes no sense when used along with structs that
//! are not `#[repr(C, packed)]`, but it is up to the user to make sure that they are.
//! 
//! ## Examples
//! ```
//! #[macro_use]
//! extern crate offset;
//!
//! #[repr(C, packed)]
//! struct HelpMeIAmTrappedInAStructFactory {
//!     help_me_before_they_: [u8; 15],
//!     a: u32
//! }
//!
//! fn main() {
//!     assert_eq!(offset_of!(HelpMeIAmTrappedInAStructFactory, a), 15);
//!     assert_eq!(span_of!(HelpMeIAmTrappedInAStructFactory, a), 15..19);
//!     assert_eq!(span_of!(HelpMeIAmTrappedInAStructFactory, help_me_before_they_[2] .. a), 2..15);
//! }
//! ```
//!
//! This functionality can be useful, for example, for checksum calculations:
//!
//! ```
//! #[repr(C, packed)]
//! struct Message {
//!     header: MessageHeader,
//!     fragment_index: u32,
//!     fragment_count: u32,
//!     payload: [u8; 1024],
//!     checksum: u16
//! }
//! 
//! let checksum_range = &raw[span_of!(Message, header..checksum)];
//! let checksum = crc16(checksum_range);
//! ```

// Support for the usage of this crate without the standard library.
#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
#[doc(hidden)]
pub use std::mem;

#[cfg(not(feature="std"))]
#[doc(hidden)]
pub use core::mem;

/// Calculates the offset of the specified field from the start of the struct.
/// This macro supports arbitrary amount of subscripts and recursive member-accesses.
///
/// *Note*: This macro may not make much sense when used on structs that are not `#[repr(C, packed)]`
/// 
/// ## Examples - Simple
/// ```
/// #[macro_use]
/// extern crate offset;
///
/// #[repr(C, packed)]
/// struct Foo {
///     a: u32,
///     b: u64,
///     c: [u8; 5]
/// }
///
/// fn main() {
///     assert_eq!(offset_of!(Foo, a), 0);
///     assert_eq!(offset_of!(Foo, b), 4);
///     assert_eq!(offset_of!(Foo, c[2]), 14);
/// }
/// ```
///
/// ## Examples - Advanced
/// ```
/// #[macro_use]
/// extern crate offset;
///
/// #[repr(C, packed)]
/// struct UnnecessarilyComplicatedStruct {
///     member: [UnnecessarilyComplexStruct; 12]
/// }
///
/// #[repr(C, packed)]
/// struct UnnecessarilyComplexStruct {
///     a: u32,
///     b: u64,
///     c: [u8; 5]
/// }
///
///
/// fn main() {
///     assert_eq!(offset_of!(UnnecessarilyComplicatedStruct, member[3].c[3]), 66);
/// }
/// ```
#[macro_export]
macro_rules! offset_of {
    ($father:ty, $($field:tt)+) => ({
        let root: $father = unsafe { $crate::mem::uninitialized() };

        let base = &root as *const _ as usize;
        let member = &root.$($field)+ as *const _ as usize;

        $crate::mem::forget(root);

        member - base
    });
}

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
/// extern crate offset;
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
        let root: $father = unsafe { $crate::mem::uninitialized() };

        let start = offset_of!($father, $field_a $([$index])*);
        let end = offset_of!($father, $($field_b)+);

        $crate::mem::forget(root);

        start..end
    });

    ($father:ty,  $field_a:ident $([$index:expr])* ..= $($field_b:tt)+) => ({
        let root: $father = unsafe { $crate::mem::uninitialized() };

        let start = offset_of!($father, $field_a $([$index])*);
        let end = offset_of!($father, $($field_b)+) + 
                    $crate::mem::size_of_val(&root.$($field_b)+);

        $crate::mem::forget(root);

        start..end
    });

    ($father:ty, $($field:tt)+) => ({
        let root: $father = unsafe { $crate::mem::uninitialized() };

        let start = offset_of!($father, $($field)+);
        let end = start + $crate::mem::size_of_val(&root.$($field)+);

        $crate::mem::forget(root);

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
    fn offset_simple() {
        assert_eq!(offset_of!(Foo, a), 0);
        assert_eq!(offset_of!(Foo, b), 4);
        assert_eq!(offset_of!(Foo, c), 8);
    }

    #[test]
    fn offset_index() {
        assert_eq!(offset_of!(Foo, b[2]), 6);
    }

    #[test]
    #[should_panic]
    fn offset_index_out_of_bounds() {
        offset_of!(Foo, b[4]);
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
    fn huge() {
        struct Huge {
            preamble: [u8; 8192],
            member: u8
        }

        assert_eq!(offset_of!(Huge, member), 8192);
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
