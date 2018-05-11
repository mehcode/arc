extern crate failure;
extern crate palette;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "macos")]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod color;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use self::macos::*;
