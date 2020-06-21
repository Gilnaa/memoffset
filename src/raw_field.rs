// Copyright (c) 2020 Gilad Naaman, Ralf Jung
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

/// Deref-coercion protection macro.
#[macro_export]
#[doc(hidden)]
macro_rules! _memoffset__field_check {
    ($type:path, $field:tt) => {
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        #[cfg_attr(allow_clippy, allow(clippy::unneeded_field_pattern))]
        let $type { $field: _, .. };
    };
}

/// Computes a const raw pointer to the given field of the given base pointer
/// to the given parent type.
///
/// The `base` pointer *must not* be dangling, but it *may* point to
/// uninitialized memory.
#[cfg(feature = "unstable_raw")] // Correct variant that uses `raw_const!`.
#[macro_export(local_inner_macros)]
macro_rules! raw_field {
    ($base:expr, $parent:path, $field:tt) => {{
        _memoffset__field_check!($parent, $field);
        let base_ptr: *const $parent = $base;

        // Get the field address without creating a reference.
        // Crucially, we know that this will not trigger a deref coercion because
        // of the `field_check!` we did above.
        #[allow(unused_unsafe)] // for when the macro is used in an unsafe block
        unsafe {
            $crate::ptr::raw_const!((*base_ptr).$field)
        }
    }};
}

/// Computes a const raw pointer to the given field of the given base pointer
/// to the given parent type.
///
/// The `base` pointer *must not* be dangling, but it *may* point to
/// uninitialized memory.
#[cfg(not(feature = "unstable_raw"))] // Incorrect (UB) variant that creates an intermediate reference.
#[macro_export(local_inner_macros)]
macro_rules! raw_field {
    ($base:expr, $parent:path, $field:tt) => {{
        _memoffset__field_check!($parent, $field);
        let base_ptr: *const $parent = $base;

        // Get the field address. This is UB because we are creating a reference to
        // the uninitialized field. Will be updated to use `&raw` before rustc
        // starts exploiting such UB.
        // Crucially, we know that this will not trigger a deref coercion because
        // of the `field_check!` we did above.
        #[allow(unused_unsafe)] // for when the macro is used in an unsafe block
        unsafe {
            &(*base_ptr).$field as *const _
        }
    }};
}
