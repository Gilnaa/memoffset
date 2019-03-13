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
/// // Exclusive
/// span_of!(Struct, member_a .. member_b)
/// // Inclusive
/// span_of!(Struct, member_a ..= member_b)
///
/// // Open-ended ranges
/// span_of!(Struct, .. end)
/// span_of!(Struct, start ..)
/// ```
///
/// *Note*: 
/// This macro uses recursion in order to resolve the range expressions, so there is a limit to the
/// complexity of the expression.
/// 
/// It also supports a lambda-like notation for completely arbitrary expressions like:
/// `(x -> &x.foo)..(x -> &x.bar[1])`
///
/// These expressions are more powerful, and are not subject to the recursion limit. Each lambda
/// should act as though it is taking a static reference to an uninitialized object and returning
/// a reference to a field within that same object.
///
/// *Note*: 
/// This macro may not make much sense when used on structs that are not `#[repr(C, packed)]`
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
    (@helper $parent:ty, $id:ident -> $start:expr, $end:expr) => (unsafe {        
        let $id: &'static $parent = $crate::Transmuter::<$parent> { int: 0 }.ptr;
        let start = $crate::Transmuter { ptr: $start }.int;
        start..$end
    });

    (@helper $parent:ty, $id:ident -> $start:expr, $id2:ident -> $end:expr, $extra:expr) => (
        span_of!(@helper $parent, $id -> $start, {
            let $id2 = $id;
            let end = $crate::Transmuter { ptr: $end }.int;
            end + $extra
        })
    );

    (@helper $parent:ty, [] ..=) => (
        compile_error!("Expected a range, found '..='")
    );
    (@helper $parent:ty, [] ..) => (
        compile_error!("Expected a range, found '..'")
    );
    (@helper $parent:ty, [] ..= $($field:tt)+) => (
        span_of!($parent, ..=(x -> &x.$($field)*))
    );
    (@helper $parent:ty, [] .. $($field:tt)+) => (
        span_of!($parent, ..(x -> &x.$($field)*))
    );
    (@helper $parent:ty, $(# $begin:tt)+ [] ..= $($end:tt)+) => (
        span_of!($parent, (x -> &x.$($begin)*)..=(x -> &x.$($end)*))
    );
    (@helper $parent:ty, $(# $begin:tt)+ [] .. $($end:tt)+) => (
        span_of!($parent, (x -> &x.$($begin)*)..(x -> &x.$($end)*))
    );
    (@helper $parent:ty, $(# $begin:tt)+ [] ..) => (
        span_of!($parent, x -> &x.$($begin)*)
    );
    (@helper $parent:ty, $(# $begin:tt)+ [] ..=) => {
        compile_error!(
            "Found inclusive range to the end of a struct. Did you mean '..' instead of '..='?")
    };
    (@helper $parent:ty, $(# $begin:tt)+ [] ) => (
        span_of!($parent, (x -> &x.$($begin)*)..=(x -> &x.$($begin)*))
    );
    (@helper $parent:ty, $(# $begin:tt)+ [] $tt:tt $($rest:tt)*) => {
        span_of!(@helper $parent, $(#$begin)* #$tt [] $($rest)*)
    };
    (@helper $parent:ty, [] $tt:tt $($rest:tt)*) => {
        span_of!(@helper $parent, #$tt [] $($rest)*)
    };


    ($parent:ty,  .. ($id2:ident -> $end:expr)) => (
        span_of!(@helper $parent, x -> x, $id2 -> $end, 0)
    );
    ($parent:ty,  ..= ($id2:ident -> $end:expr)) => (
        span_of!(@helper $parent, x -> x, $id2 -> $end, $crate::size_of($end))
    );

    ($parent:ty, $id:ident -> $start:expr) => (
        span_of!(@helper $parent, $id -> $start, $crate::mem::size_of::<$parent>())
    );

    ($parent:ty, ($id:ident -> $start:expr)..) => (
        span_of!($parent, $id -> $start)
    );
    ($parent:ty, ($id:ident -> $start:expr) .. ($id2:ident -> $end:expr)) => (
        span_of!(@helper $parent, $id -> $start, $id2 -> $end, 0)
    );
    ($parent:ty, ($id:ident -> $start:expr) ..= ($id2:ident -> $end:expr)) => (
        span_of!(@helper $parent, $id -> $start, $id2 -> $end, $crate::size_of($end))
    );
    
    ($parent:ty, ($id:ident -> $start:expr) .. ) => (
        compile_error!("Expected a range, found '..'")
    );
    ($parent:ty, ($id:ident -> $start:expr) ..= ) => (
        compile_error!("Expected a range, found '..='")
    );


    ($parent:ty, $($exp:tt)+) => (span_of!(@helper $parent, [] $($exp)*));
}

#[cfg(test)]
mod tests {
    use ::core::mem;

    #[repr(C, packed)]
    struct Foo {
        a: u32,
        b: [u8; 4],
        c: i64,
    }

    #[test]
    fn span_integer() {
        assert_eq!(span_of!(u32, x -> x), 0..4);
        assert_eq!(span_of!(u32, (x -> x)..(x -> x)), 0..0);
        assert_eq!(span_of!(u32, (x -> x)..=(x -> x)), 0..4);
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
            a: u32,
        }

        #[repr(C, packed)]
        struct Blarg {
            x: u64,
            y: [u8; 56],
            z: Florp,
            egg: [[u8; 4]; 4],
        }

        // Love me some brute force
        assert_eq!(0..8, span_of!(Blarg, x));
        assert_eq!(64..68, span_of!(Blarg, z.a));
        assert_eq!(79..80, span_of!(Blarg, egg[2][3]));

        assert_eq!(8..64, span_of!(Blarg, y[0]..z));
        assert_eq!(0..42, span_of!(Blarg, x..y[34]));
        assert_eq!(0..64, span_of!(Blarg, x     ..= y));
        assert_eq!(58..68, span_of!(Blarg, y[50] ..= z));
    }

    #[test]
    fn ig_test() {
        #[repr(C)]
        struct Member {
            foo: u32,
        }

        #[repr(C)]
        struct Test {
            x: u64,
            y: [u8; 56],
            z: Member,
            egg: [[u8; 4]; 4],
        }

        assert_eq!(span_of!(Test, ..x), 0..0);
        assert_eq!(span_of!(Test, ..=x), 0..8);
        assert_eq!(span_of!(Test, ..y), 0..8);
        assert_eq!(span_of!(Test, ..=y), 0..64);
        assert_eq!(span_of!(Test, ..y[0]), 0..8);
        assert_eq!(span_of!(Test, ..=y[0]), 0..9);
        assert_eq!(span_of!(Test, ..z), 0..64);
        assert_eq!(span_of!(Test, ..=z), 0..68);
        assert_eq!(span_of!(Test, ..z.foo), 0..64);
        assert_eq!(span_of!(Test, ..=z.foo), 0..68);
        assert_eq!(span_of!(Test, ..egg), 0..68);
        assert_eq!(span_of!(Test, ..=egg), 0..84);
        assert_eq!(span_of!(Test, ..egg[0]), 0..68);
        assert_eq!(span_of!(Test, ..=egg[0]), 0..72);
        assert_eq!(span_of!(Test, ..egg[0][0]), 0..68);
        assert_eq!(span_of!(Test, ..=egg[0][0]), 0..69);
        assert_eq!(
            span_of!(Test, x..),
            offset_of!(Test, x)..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, y..),
            offset_of!(Test, y)..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, y[0]..),
            offset_of!(Test, y[0])..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, z..),
            offset_of!(Test, z)..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, z.foo..),
            offset_of!(Test, z.foo)..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, egg..),
            offset_of!(Test, egg)..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, egg[0]..),
            offset_of!(Test, egg[0])..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, egg[0][0]..),
            offset_of!(Test, egg[0][0])..mem::size_of::<Test>()
        );
        assert_eq!(
            span_of!(Test, x..y),
            offset_of!(Test, x)..offset_of!(Test, y)
        );
        assert_eq!(
            span_of!(Test, x..=y),
            offset_of!(Test, x)..offset_of!(Test, y) + mem::size_of::<[u8; 56]>()
        );
        assert_eq!(
            span_of!(Test, x..y[4]),
            offset_of!(Test, x)..offset_of!(Test, y[4])
        );
        assert_eq!(
            span_of!(Test, x..=y[4]),
            offset_of!(Test, x)..offset_of!(Test, y) + mem::size_of::<[u8; 5]>()
        );
        assert_eq!(
            span_of!(Test, x..z.foo),
            offset_of!(Test, x)..offset_of!(Test, z.foo)
        );
        assert_eq!(
            span_of!(Test, x..=z.foo),
            offset_of!(Test, x)..offset_of!(Test, z.foo) + mem::size_of::<u32>()
        );
        assert_eq!(
            span_of!(Test, egg[0][0]..egg[1][0]),
            offset_of!(Test, egg[0][0])..offset_of!(Test, egg[1][0])
        );
    }
}
