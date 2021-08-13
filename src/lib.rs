/*! Drew's core foundation library.

 Similar to my other macOS libraries, this provides an elegant, performant, and largely safe wrapper
for macOS low-level libraries.

Design notes:
* We use opaque types to model the underlying CFType (like `__CFString`, etc.)  Then the equivalent to `CFStringRef` is
  `&CFString`, a pointer type.
* Some optimizations are not yet implemented.  Among them, optimizations for static-time strings, inner pointers, zero-copy, etc.

 */
mod base;
mod string;
mod cell;
mod data;
mod property_list;
mod error;
mod dictionary;
mod array;


pub use string::CFStringEncoding;
pub use base::CFString;
pub use property_list::{MutabilityOptions,Format};
pub use data::CFData;
pub use property_list::CFPropertyList;
pub use base::CFTypeBehavior;
pub use dictionary::CFDictionary;
pub use array::CFArray;
pub use error::CFError;
pub use cell::StrongCell;