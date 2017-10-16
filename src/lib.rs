// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A crate used for calculating offsets of struct members and their spans.
//!
//! Some of the funcationality of the crate makes no sense when used along with structs that
//! are not `#[repr(C, packed)]`, but it is up to the user to make sure that they are.
//! 
//! ## Examples
//! ```
//! #[macro_use]
//! extern crate memoffset;
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
//! ```ignore
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
pub use std::{mem, ptr};

#[cfg(not(feature="std"))]
#[doc(hidden)]
pub use core::{mem, ptr};

#[macro_use]
mod offset_of;
#[macro_use]
mod span_of;