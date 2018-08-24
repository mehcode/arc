#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    future_incompatible
)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
// Dozens of false positives (for this crate): https://github.com/SSheldon/rust-objc/pull/71
#![cfg_attr(feature = "cargo-clippy", allow(replace_consts))]
// DEBUG
#![allow(unused)]
// Intended
#![cfg_attr(
    feature = "cargo-clippy",
    allow(cast_sign_loss, cast_possible_truncation)
)]

#[macro_use]
mod layout;

mod color;
mod context;
mod event;
mod events;
mod font;
mod geometry;
mod node;
mod os;
mod text;
mod view;
mod window;

// TODO: Submit documentation for these types to `yoga`.
pub use yoga::{Align, Edge, FlexDirection, Justify, PositionType, Wrap};

pub use self::{
    color::*, event::*, events::*, font::*, geometry::*, layout::*, node::*, os::NodeId, text::*,
    view::*, window::*,
};
