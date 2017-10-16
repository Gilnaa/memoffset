// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Calculates the offset of the specified field from the start of the struct.
/// This macro supports arbitrary amount of subscripts and recursive member-accesses.
///
/// *Note*: This macro may not make much sense when used on structs that are not `#[repr(C, packed)]`
/// 
/// ## Examples - Simple
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
///     assert_eq!(offset_of!(Foo, a), 0);
///     assert_eq!(offset_of!(Foo, b), 4);
///     assert_eq!(offset_of!(Foo, c[2]), 14);
/// }
/// ```
///
/// ## Examples - Advanced
/// ```
/// #[macro_use]
/// extern crate memoffset;
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
        let root: *const $father = $crate::ptr::null();

        let base = root as usize;
        let member = unsafe { &(*root).$($field)* } as *const _ as usize;

        member - base
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
    fn tuple_struct() {
        #[repr(C,packed)]
        struct Tup(i32, i32);

        assert_eq!(offset_of!(Tup, 0), 0);
    }
}