// Copyright (c) 2017 Gilad Naaman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Calculates the offset of the specified field from the start of the struct.
/// This macro supports arbitrary amount of subscripts and recursive member-accesses.
///
/// *Note*: This macro may not make much sense when used on structs that are not `#[repr(C, packed)]`
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
///     assert_eq!(offset_of!(Foo, a), 0);
///     assert_eq!(offset_of!(Foo, b), 4);
/// }
/// ```
#[macro_export]
#[cfg(memoffset_maybe_uninit)]
macro_rules! offset_of {
    ($parent:tt, $field:tt) => {{
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        let $parent { $field: _, .. };

        // Create an instance of the container and calculate the offset to its field.
        // Here we're using an uninitialized instance of $parent.
        // Since we're not using its field, there's no UB caused by reading uninitialized memory.
        // There *IS*, though, UB caused by creating references to uninitialized data,
        // which is illegal since the compiler is allowed to assume that a reference
        // points to valid data.
        // However, currently we just cannot avoid UB completely.
        let val = $crate::mem::MaybeUninit::<$parent>::uninit();
        let base_ptr = val.as_ptr();
        #[allow(unused_unsafe)] // for when the macro is used in an unsafe block
        let field_ptr = unsafe { &(*base_ptr).$field as *const _ };
        let offset = (field_ptr as usize) - (base_ptr as usize);
        offset
    }};
}

#[macro_export]
#[cfg(not(memoffset_maybe_uninit))]
macro_rules! offset_of {
    ($parent:tt, $field:tt) => {{
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        let $parent { $field: _, .. };

        // This is UB since we're dealing with dangling references.
        // We're never dereferencing it, but it's UB nonetheless.
        // See above for a better version that only works with newer Rust.
        let non_null = $crate::ptr::NonNull::<$parent>::dangling();
        #[allow(unused_unsafe)]
        let base_ptr = unsafe { non_null.as_ref() as *const _ };
        #[allow(unused_unsafe)]
        let field_ptr = unsafe { &(*base_ptr).$field as *const _ };
        let offset = (field_ptr as usize) - (base_ptr as usize);
        offset
    }};
}

#[cfg(test)]
mod tests {
    #[repr(C, packed)]
    struct Foo {
        a: u32,
        b: [u8; 4],
        c: i64,
    }

    #[test]
    fn offset_simple() {
        assert_eq!(offset_of!(Foo, a), 0);
        assert_eq!(offset_of!(Foo, b), 4);
        assert_eq!(offset_of!(Foo, c), 8);
    }

    #[test]
    fn tuple_struct() {
        #[repr(C, packed)]
        struct Tup(i32, i32);

        assert_eq!(offset_of!(Tup, 0), 0);
    }
}
