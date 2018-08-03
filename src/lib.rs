#![feature(rust_2018_preview)]
#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    future_incompatible
)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(
    // Dozens of false positives (for this crate): https://github.com/SSheldon/rust-objc/pull/71
    replace_consts,
    // Intended
    cast_sign_loss,
    cast_possible_truncation,
))]

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
    color::*, context::*, event::*, events::*, font::*, geometry::*, layout::*, node::*, text::*,
    view::*, window::*,
};
