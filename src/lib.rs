//! Library to de- and encode [osu!] storyboards.
//!
//! # Usage
//!
//! Based on `rosu-map`'s [`DecodeBeatmap`] trait, the [`Storyboard`] struct provides a way
//! to decode `.osu` or `.osb` files.
//!
//! ```
//! use rosu_storyboard::Storyboard;
//! use rosu_storyboard::elements::ElementKind;
//!
//! let path = "./resources/Himeringo - Yotsuya-san ni Yoroshiku (RLC) [Winber1's Extreme].osu";
//! let storyboard = Storyboard::from_path(path).unwrap();
//!
//! let first_bg_elem = &storyboard.layers["Background"].elements[0];
//! assert!(first_bg_elem.kind, ElementKind::Sprite(_));
//! ```
//!
//! [osu!]: https://osu.ppy.sh/
//! [`DecodeBeatmap`]: rosu_map::DecodeBeatmap
//! [`Storyboard`]: crate::storyboard::Storyboard

#![deny(rustdoc::broken_intra_doc_links, rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_const_for_fn, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::match_same_arms,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::explicit_iter_loop,
    clippy::default_trait_access
)]

pub use self::{
    decode::{ParseStoryboardError, StoryboardState},
    layer::Layer,
    storyboard::Storyboard,
};

mod decode;
mod encode;
mod layer;
mod storyboard;

/// Command types.
pub mod command;

/// Storyboard elements.
pub mod element;

/// Visual elements.
pub mod visual;

/// Re-exported types of `rosu-map`.
pub mod reexport {
    pub use rosu_map::{section::colors::Color, util::Pos};
}
